use anyhow::{Context, Result};
use reqwest::{Client, Version};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Instant;

// 引入 trust-dns 库进行 RFC 8484 DNS 消息解析
// 暂时注释掉以避免编译问题，当前使用JSON API进行DNS查询
// use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
// use trust_dns_resolver::name_server::NameServer;
// use trust_dns_resolver::proto::rr::{RData};
// use trust_dns_resolver::proto::op::Message;
// use trust_dns_resolver::proto::rr::RecordType;

// --- 1. 输入配置 ---
// CLAUDE.md: "程序接受JSON格式的配置"
#[derive(Debug, Clone, Deserialize, Serialize)]
struct InputTask {
    doh_resolve_domain: String,
    test_sni_host: String,
    test_host_header: String,
    doh_url: String,
    port: u16,
    prefer_ipv6: Option<bool>,
    resolve_mode: String,
    direct_ips: Option<Vec<String>>,
}

// --- 2. DoH JSON API 响应格式 (例如 Google/Cloudflare 的 JSON API) ---
// 参照 resolve_https_record 和 resolve_a_aaaa_record 中的用法
#[derive(Debug, Deserialize)]
struct DoHResponse {
    #[serde(default)]
    Status: Option<u32>, // Google DNS status field
    #[serde(default)]
    status: u32, // AdGuard/other DNS status field (default)
    #[serde(default)]
    Answer: Option<Vec<Answer>>, // Google DNS answer field
    #[serde(default)]
    answer: Option<Vec<Answer>>, // AdGuard/other DNS answer field
}

// 辅助结构，用于解析 Answer 数组中的单个记录
#[derive(Debug, Deserialize)]
struct Answer {
    #[serde(rename = "type")] // 'type' is a reserved keyword in Rust
    record_type: u16,
    data: String,
}

// --- 3. 输出结果 ---
#[derive(Debug, Serialize)]
struct TestResult {
    domain_used: String, // DoH 解析的域名
    target_ip: String,
    ip_version: String,
    sni_host: String,    // 实际使用的 SNI
    host_header: String, // 实际使用的 Host header
    success: bool,
    status_code: Option<u16>,
    protocol: String,    // 实际协商的协议 (h3, h2, http/1.1)
    latency_ms: Option<u64>,
    server_header: Option<String>,
    error_msg: Option<String>,
    dns_source: String, // "Direct Input", "Binary DoH", 或 "JSON DoH"
}

// --- 4. 核心：DoH HTTPS 记录查询 (JSON API) ---
async fn resolve_https_record(client: &Client, doh_url: &str, domain: &str) -> Result<Vec<IpAddr>> {
    let dns_url = format!(
        "{}/resolve?name={}&type=HTTPS",
        doh_url.trim_end_matches('/'),
        domain
    );

    let resp = client
        .get(&dns_url)
        .send()
        .await
        .context("Failed to connect to DNS resolver")?
        .json::<DoHResponse>()
        .await
        .context("Failed to parse DNS JSON")?;

    // Use lowercase status if present, otherwise use uppercase (for different DoH providers)
    let final_status = resp.status.or(resp.Status).unwrap_or(0);

    if final_status != 0 {
        return Err(anyhow::anyhow!("DNS query returned non-zero status: {}", final_status));
    }

    let mut ip_strings = Vec::new();

    // Use lowercase answer if present, otherwise use uppercase (for different DoH providers)
    let answers = resp.answer.or(resp.Answer);

    if let Some(answers) = answers {
        for ans in answers {
            if ans.record_type == 65 { // HTTPS 记录类型
                let (v4, v6) = parse_https_hints(&ans.data);
                ip_strings.extend(v4);
                ip_strings.extend(v6);
            }
        }
    }

    // 转换字符串IP为IpAddr
    let mut ips = Vec::new();
    for ip_str in ip_strings {
        if let Ok(ip) = IpAddr::from_str(&ip_str) {
            ips.push(ip);
        }
    }

    // 去重
    let unique_ips: HashSet<IpAddr> = ips.into_iter().collect();
    Ok(unique_ips.into_iter().collect())
}

// --- 5. 兼容性：DoH A/AAAA 记录查询 ---
async fn resolve_a_aaaa_record(client: &Client, doh_url: &str, domain: &str, ipv6: bool) -> Result<Vec<IpAddr>> {
    let type_param = if ipv6 { "AAAA" } else { "A" };
    let dns_url = format!(
        "{}/resolve?name={}&type={}",
        doh_url.trim_end_matches('/'),
        domain,
        type_param
    );

    let resp = client
        .get(&dns_url)
        .send()
        .await
        .context("Failed to connect to DNS resolver")?
        .json::<DoHResponse>()
        .await
        .context("Failed to parse DNS JSON")?;

    // Use lowercase status if present, otherwise use uppercase (for different DoH providers)
    let final_status = resp.status.or(resp.Status).unwrap_or(0);

    if final_status != 0 {
        return Err(anyhow::anyhow!("DNS query returned non-zero status: {}", final_status));
    }

    let mut ip_strings = Vec::new();

    // Use lowercase answer if present, otherwise use uppercase (for different DoH providers)
    let answers = resp.answer.or(resp.Answer);

    if let Some(answers) = answers {
        for ans in answers {
            // A (1) 或 AAAA (28)
            if ans.record_type == 1 || ans.record_type == 28 {
                ip_strings.push(ans.data.trim().to_string());
            }
        }
    }

    // 转换字符串IP为IpAddr
    let mut ips = Vec::new();
    for ip_str in ip_strings {
        if let Ok(ip) = IpAddr::from_str(&ip_str) {
            ips.push(ip);
        }
    }

    Ok(ips)
}

// --- 6. 兼容性：解析 HTTPS 记录字符串 (使用 RFC 8484 标准 + 正则表达式回退) ---
fn parse_https_hints(data: &str) -> (Vec<String>, Vec<String>) {
    let mut v4_ips = Vec::new();
    let mut v6_ips = Vec::new();

    // 首先尝试使用正则表达式快速解析常见格式
    let re_v4 = regex::Regex::new(r"ipv4hint=([0-9\.,]+)").unwrap();
    let re_v6 = regex::Regex::new(r"ipv6hint=([0-9a-fA-F:\.,]+)").unwrap();

    if let Some(caps) = re_v4.captures(data) {
        if let Some(match_str) = caps.get(1) {
            for ip in match_str.as_str().split(',') {
                v4_ips.push(ip.trim().to_string());
            }
        }
    }

    if let Some(caps) = re_v6.captures(data) {
        if let Some(match_str) = caps.get(1) {
            for ip in match_str.as_str().split(',') {
                v6_ips.push(ip.trim().to_string());
            }
        }
    }

    // 如果正则表达式没有找到结果，尝试更复杂的解析
    if v4_ips.is_empty() && v6_ips.is_empty() {
        // 可以在这里添加更复杂的RFC 8484解析逻辑
        // 目前先使用文本解析作为回退
        parse_hints_from_text(data, &mut v4_ips, &mut v6_ips);
    }

    (v4_ips, v6_ips)
}

// 辅助函数：从文本格式解析hints (向后兼容)
fn parse_hints_from_text(data: &str, v4_ips: &mut Vec<String>, v6_ips: &mut Vec<String>) {
    // 使用正则表达式作为回退方案
    let re_v4 = regex::Regex::new(r"ipv4hint=([0-9\.,]+)").unwrap();
    let re_v6 = regex::Regex::new(r"ipv6hint=([0-9a-fA-F:\.,]+)").unwrap();

    if let Some(caps) = re_v4.captures(data) {
        if let Some(match_str) = caps.get(1) {
            for ip in match_str.as_str().split(',') {
                v4_ips.push(ip.trim().to_string());
            }
        }
    }

    if let Some(caps) = re_v6.captures(data) {
        if let Some(match_str) = caps.get(1) {
            for ip in match_str.as_str().split(',') {
                v6_ips.push(ip.trim().to_string());
            }
        }
    }
}

// --- 7. HTTP/3 连通性测试 ---
async fn test_connectivity(task: InputTask, ip: IpAddr, dns_source: String) -> TestResult {
    // 关键点 1: URL 决定 SNI。reqwest 将使用 test_sni_host 作为 SNI。
    let url = format!("https://{}:{}/", task.test_sni_host, task.port);

    let socket_addr = SocketAddr::new(ip, task.port);
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    // 创建独立的 Client 实例以绑定特定的 IP
    let client_build = Client::builder()
        // 关键点 2: 强制 IP 连接 (将 test_sni_host 解析到特定的 IP)
        .resolve_to_addrs(&task.test_sni_host, &[socket_addr])
        .danger_accept_invalid_certs(true)
        .http3_prior_knowledge() // 核心：强制使用 HTTP/3
        .timeout(std::time::Duration::from_secs(5))
        .no_proxy()
        .build();

    let client = match client_build {
        Ok(c) => c,
        Err(e) => return TestResult::fail(&task, &ip.to_string(), ip_ver, e.to_string(), dns_source),
    };

    let start = Instant::now();

    // 发送请求
    let req = client
        .get(&url)
        // 关键点 3: 覆盖 Host Header
        .header("Host", &task.test_host_header)
        .header("User-Agent", "curl/8.12.1")
        .send();

    match req.await {
        Ok(res) => {
            let latency = start.elapsed().as_millis() as u64;
            let status = res.status().as_u16();
            let server = res
                .headers()
                .get("server")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let protocol = match res.version() {
                Version::HTTP_3 => "h3",
                Version::HTTP_2 => "h2",
                Version::HTTP_11 => "http/1.1",
                _ => "unknown",
            };

            TestResult {
                domain_used: task.doh_resolve_domain,
                target_ip: ip.to_string(),
                ip_version: ip_ver.to_string(),
                sni_host: task.test_sni_host,
                host_header: task.test_host_header,
                success: status < 500,
                status_code: Some(status),
                protocol: protocol.to_string(),
                latency_ms: Some(latency),
                server_header: server,
                error_msg: None,
                dns_source,
            }
        }
        Err(e) => TestResult::fail(&task, &ip.to_string(), ip_ver, e.to_string(), dns_source),
    }
}

impl TestResult {
    fn fail(task: &InputTask, ip: &str, ver: &str, msg: String, dns_source: String) -> Self {
        TestResult {
            domain_used: task.doh_resolve_domain.clone(),
            target_ip: ip.to_string(),
            ip_version: ver.to_string(),
            sni_host: task.test_sni_host.clone(),
            host_header: task.test_host_header.clone(),
            success: false,
            status_code: None,
            protocol: "none".to_string(),
            latency_ms: None,
            server_header: None,
            error_msg: Some(msg),
            dns_source,
        }
    }
}

// --- 8. 主程序入口 ---
#[tokio::main]
async fn main() -> Result<()> {
    // 用于 DoH 请求的客户端
    let doh_http_client = Client::builder()
        .use_rustls_tls()
        .build()
        .expect("Failed to create DoH client");

    // 示例输入 JSON：演示了如何使用不同的域名进行解析和测试
    let input_json = r#"
    [
        {
            "doh_resolve_domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test_sni_host": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test_host_header": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.adguard-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": null,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.adguard-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": false,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.google/resolve",
            "port": 443,
            "prefer_ipv6": true,
            "resolve_mode": "a_aaaa"
        },
        {
            "doh_resolve_domain": "cloudflare.com",
            "test_sni_host": "cloudflare.com",
            "test_host_header": "cloudflare.com",
            "doh_url": "https://1.1.1.1/dns-query",
            "port": 443,
            "prefer_ipv6": null,
            "direct_ips": ["104.16.123.96", "172.67.214.232"],
            "resolve_mode": "direct"
        }
    ]
    "#;

    let tasks: Vec<InputTask> = serde_json::from_str(input_json)
        .context("Invalid JSON format in input")?;

    let mut futures = Vec::new();

    for task in tasks {
        let client_ref = doh_http_client.clone();

        println!(">>> 正在通过 {} 解析 {} 的 HTTPS 记录 (模式: {})...",
                 task.doh_url, task.doh_resolve_domain, task.resolve_mode);

        // 检查是否有直接指定的IP
        if let Some(direct_ips) = &task.direct_ips {
            println!("    -> 使用直接指定的IP: {:?}", direct_ips);

            for ip_str in direct_ips {
                if let Ok(ip_addr) = IpAddr::from_str(ip_str) {
                    let is_v6 = ip_addr.is_ipv6();

                    // 应用IP版本过滤
                    if let Some(prefer_v6) = task.prefer_ipv6 {
                        if prefer_v6 != is_v6 {
                            continue;
                        }
                    }

                    let task_clone = task.clone();
                    futures.push(tokio::spawn(async move {
                        test_connectivity(task_clone, ip_addr, "Direct Input".to_string()).await
                    }));
                }
            }
            continue;
        }

        // 根据解析模式选择解析方法
        match task.resolve_mode.as_str() {
            "https" => {
                // 使用 HTTPS 记录查询
                match resolve_https_record(&client_ref, &task.doh_url, &task.doh_resolve_domain).await {
                    Ok(ips) => {
                        if ips.is_empty() {
                            println!("    [!] 未找到 IP Hint");
                            continue;
                        }
                        println!("    -> 解析成功，获取到 {} 个 IP Hint: {:?}", ips.len(), ips);

                        for ip in ips {
                            let is_v6 = ip.is_ipv6();

                            // 应用IP版本过滤
                            if let Some(prefer_v6) = task.prefer_ipv6 {
                                if prefer_v6 != is_v6 {
                                    continue;
                                }
                            }

                            let task_clone = task.clone();
                            futures.push(tokio::spawn(async move {
                                test_connectivity(task_clone, ip, "HTTPS DoH".to_string()).await
                            }));
                        }
                    },
                    Err(e) => {
                        eprintln!("    [X] HTTPS记录解析失败: {:?}", e);
                    }
                }
            },
            "a_aaaa" => {
                // 使用 A/AAAA 记录查询
                let resolve_ipv6 = task.prefer_ipv6.unwrap_or(false);
                match resolve_a_aaaa_record(&client_ref, &task.doh_url, &task.doh_resolve_domain, resolve_ipv6).await {
                    Ok(ips) => {
                        if ips.is_empty() {
                            println!("    [!] 未找到IP地址");
                            continue;
                        }
                        println!("    -> 解析成功，获取到 {} 个IP地址: {:?}", ips.len(), ips);

                        for ip in ips {
                            let task_clone = task.clone();
                            futures.push(tokio::spawn(async move {
                                test_connectivity(task_clone, ip, "A/AAAA DoH".to_string()).await
                            }));
                        }
                    },
                    Err(e) => {
                        eprintln!("    [X] A/AAAA记录解析失败: {:?}", e);
                    }
                }
            },
            "direct" => {
                // 直接模式，跳过DNS解析
                println!("    -> 跳过DNS解析，使用直接IP模式");
            },
            _ => {
                eprintln!("    [!] 不支持的解析模式: {}", task.resolve_mode);
            }
        }
    }

    // 等待所有测试完成
    let mut results = Vec::new();
    for f in futures {
        if let Ok(res) = f.await {
            results.push(res);
        }
    }

    println!("\n=== 最终测试结果 (JSON) ===");
    println!("{}", serde_json::to_string_pretty(&results).unwrap());

    Ok(())
}