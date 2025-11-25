use anyhow::{Context, Result};
use reqwest::{Client, Version};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;


// --- 1. 输入配置 (支持三种域名参数分离) ---
#[derive(Debug, Deserialize, Clone)]
struct InputTask {
    doh_resolve_domain: String, // 1. 用于 DoH 查询，获取 IP Hint 的域名
    test_sni_host: String,      // 2. 用于 TLS SNI 的域名
    test_host_header: String,   // 3. 用于 HTTP Host Header 的值
    doh_url: String,            // DoH 解析服务的 URL
    port: u16,                  // 目标端口，通常是 443
    prefer_ipv6: Option<bool>,  // 仅测试 IPv6 Hint (true) 或 IPv4 Hint (false)
    // 可选：直接指定IP列表（跳过DNS解析）
    direct_ips: Option<Vec<String>>,
    // 解析模式："binary" 使用RFC 8484二进制格式，"json" 使用JSON API
    resolve_mode: Option<String>,
}

// --- 2. Google DoH JSON 响应结构 (用于兼容性) ---
#[derive(Debug, Deserialize)]
struct DoHResponse {
    #[serde(rename = "Status")]
    status: i32,
    #[serde(rename = "Answer")]
    answer: Option<Vec<DoHAnswer>>,
}

#[derive(Debug, Deserialize)]
struct DoHAnswer {
    name: String,
    #[serde(rename = "type")]
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

// --- 4. 核心：DoH HTTPS 记录查询与解析 ---
async fn resolve_https_record(client: &Client, doh_url: &str, domain: &str) -> Result<Vec<IpAddr>> {
    // 使用 JSON API 模式获取 HTTPS 记录
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

    if resp.status != 0 {
        return Err(anyhow::anyhow!("DNS query returned non-zero status: {}", resp.status));
    }

    let mut ip_strings = Vec::new();

    if let Some(answers) = resp.answer {
        for ans in answers {
            if ans.record_type == 65 {
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

    Ok(ips)
}

// --- 5. 兼容性：DoH JSON API 解析 (用于 Google DNS API) ---
async fn resolve_doh_json(client: &Client, domain: &str, ipv6: bool) -> Result<Vec<String>> {
    let type_param = if ipv6 { "AAAA" } else { "A" };
    let dns_url = format!(
        "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.google/resolve?name={}&type={}",
        domain, type_param
    );

    let resp = client
        .get(&dns_url)
        .send()
        .await
        .context("Failed to connect to DNS resolver")?
        .json::<DoHResponse>()
        .await
        .context("Failed to parse DNS JSON")?;

    if resp.status != 0 {
        return Err(anyhow::anyhow!("DNS query returned non-zero status: {}", resp.status));
    }

    let mut ips = Vec::new();
    if let Some(answers) = resp.answer {
        for ans in answers {
            // A (1) 或 AAAA (28)
            if ans.record_type == 1 || ans.record_type == 28 {
                ips.push(ans.data);
            }
        }
    }
    Ok(ips)
}

// --- 6. 兼容性：解析 HTTPS 记录字符串 (使用正则表达式) ---
fn parse_https_hints(data: &str) -> (Vec<String>, Vec<String>) {
    let mut v4_ips = Vec::new();
    let mut v6_ips = Vec::new();

    // 正则匹配 ipv4hint=... 和 ipv6hint=...
    let re_v4 = Regex::new(r"ipv4hint=([0-9\.,]+)").unwrap();
    let re_v6 = Regex::new(r"ipv6hint=([0-9a-fA-F:\.,]+)").unwrap();

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

    (v4_ips, v6_ips)
}

async fn resolve_https_json(client: &Client, domain: &str) -> Result<Vec<String>> {
    let dns_url = format!(
        "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.google/resolve?name={}&type=HTTPS",
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

    if resp.status != 0 {
        return Err(anyhow::anyhow!("DNS query returned non-zero status: {}", resp.status));
    }

    let mut all_ips = Vec::new();

    if let Some(answers) = resp.answer {
        for ans in answers {
            if ans.record_type == 65 {
                let (v4, v6) = parse_https_hints(&ans.data);
                all_ips.extend(v4);
                all_ips.extend(v6);
            }
        }
    }

    // 去重
    let unique_ips: HashSet<String> = all_ips.into_iter().collect();
    Ok(unique_ips.into_iter().collect())
}

// --- 7. HTTP/3 连通性测试 ---
async fn test_connectivity(task: InputTask, ip: IpAddr, dns_source: String) -> TestResult {
    // 关键点 1: URL 决定 SNI。reqwest 将使用 test_sni_host 作为 SNI。
    let url = format!("https://{}:{}/", task.test_sni_host, task.port);

    let socket_addr = SocketAddr::new(ip, task.port);
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    // 客户端构建：强制绑定 IP，并设置 H2
    let client_build = Client::builder()
        // 关键点 2: 强制 IP 连接 (将 test_sni_host 解析到特定的 IP)
        .resolve_to_addrs(&task.test_sni_host, &[socket_addr])
        .danger_accept_invalid_certs(true)
        .http2_prior_knowledge()
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
            "resolve_mode": "binary"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.adguard-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": false,
            "resolve_mode": "binary"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.google/resolve",
            "port": 443,
            "prefer_ipv6": true,
            "resolve_mode": "json"
        },
        {
            "doh_resolve_domain": "cloudflare.com",
            "test_sni_host": "cloudflare.com",
            "test_host_header": "cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/dns.google/resolve",
            "port": 443,
            "prefer_ipv6": null,
            "resolve_mode": "json",
            "direct_ips": ["104.16.123.96", "172.67.214.232"]
        }
    ]
    "#;

    let tasks: Vec<InputTask> = serde_json::from_str(input_json)
        .context("Invalid JSON format in input")?;

    let mut futures = Vec::new();

    for task in tasks {
        let client_ref = doh_http_client.clone();

        println!(">>> 正在通过 {} 解析 {} 的 HTTPS 记录...", task.doh_url, task.doh_resolve_domain);

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
        let resolve_mode = task.resolve_mode.as_deref().unwrap_or("binary");

        match resolve_mode {
            "binary" => {
                // 使用JSON API模式解析HTTPS记录
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

                        // 回退到A/AAAA记录解析
                        println!("    -> 回退到A/AAAA记录解析...");
                        let resolve_ipv6 = task.prefer_ipv6.unwrap_or(false);
                        match resolve_doh_json(&client_ref, &task.doh_resolve_domain, resolve_ipv6).await {
                            Ok(ip_strings) => {
                                println!("    -> A/AAAA解析成功，获取到 {} 个IP地址: {:?}", ip_strings.len(), ip_strings);

                                for ip_str in ip_strings {
                                    if let Ok(ip_addr) = IpAddr::from_str(&ip_str) {
                                        let task_clone = task.clone();
                                        futures.push(tokio::spawn(async move {
                                            test_connectivity(task_clone, ip_addr, "A/AAAA DoH (Fallback)".to_string()).await
                                        }));
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("    [X] A/AAAA解析也失败: {:?}", e);
                            }
                        }
                    }
                }
            },
            "json" => {
                // 使用JSON API模式解析A/AAAA记录
                let resolve_ipv6 = task.prefer_ipv6.unwrap_or(false);
                match resolve_doh_json(&client_ref, &task.doh_resolve_domain, resolve_ipv6).await {
                    Ok(ips) => {
                        if ips.is_empty() {
                            println!("    [!] 未找到IP地址");
                            continue;
                        }
                        println!("    -> 解析成功，获取到 {} 个IP地址: {:?}", ips.len(), ips);

                        for ip_str in ips {
                            if let Ok(ip_addr) = IpAddr::from_str(&ip_str) {
                                let task_clone = task.clone();
                                futures.push(tokio::spawn(async move {
                                    test_connectivity(task_clone, ip_addr, "JSON DoH".to_string()).await
                                }));
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("    [X] JSON DNS解析失败: {:?}", e);
                    }
                }
            },
            _ => {
                eprintln!("    [!] 不支持的解析模式: {}", resolve_mode);
            }
        }
    }

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