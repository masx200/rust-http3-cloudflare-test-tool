// Simplified version - focuses on basic DNS resolution and HTTP connection testing
// Now using local Hickory-DNS and Reqwest libraries
use anyhow::{Context, Result};
use hickory_resolver::{Name, Resolver, config::{ResolverConfig, NameServerConfig}};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Instant;

// --- 1. 输入配置 ---
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

// --- 2. 输出结果 ---
#[derive(Debug, Serialize)]
struct TestResult {
    domain_used: String,
    target_ip: String,
    ip_version: String,
    sni_host: String,
    host_header: String,
    success: bool,
    status_code: Option<u16>,
    protocol: String,
    latency_ms: Option<u64>,
    server_header: Option<String>,
    error_msg: Option<String>,
    dns_source: String,
}

// --- 3. 使用Hickory-DNS进行DNS解析 (支持DoH和RFC 8484) ---
async fn resolve_domain_with_hickory(client: &Client, task: &InputTask) -> Result<Vec<IpAddr>> {
    let mut ips = HashSet::new();

    if let Some(direct_ips) = &task.direct_ips {
        println!("    -> 使用直接指定的IP: {:?}", direct_ips);
        for ip_str in direct_ips {
            if let Ok(ip_addr) = IpAddr::from_str(ip_str) {
                ips.insert(ip_addr);
            }
        }
        return Ok(ips.into_iter().collect());
    }

    match task.resolve_mode.as_str() {
        "https" => {
            // 使用Hickory-DNS进行DoH查询 (RFC 8484标准)
            println!(
                "    -> 使用Hickory-DNS进行DoH查询 (RFC 8484): {}",
                task.doh_resolve_domain
            );

            // 创建解析器配置，配置DoH服务器 (RFC 8484)
            // 解析DoH URL中的域名部分
            let doh_url_obj = url::Url::parse(&task.doh_url)
                .context("Failed to parse DoH URL")?;
            let doh_host = doh_url_obj.host_str()
                .context("Failed to extract DoH host")?;

            // 从DoH URL提取IP地址 (这里我们使用Cloudflare DNS的IP)
            let doh_ips = vec![
                "1.1.1.1",
                "1.0.0.1",
                "2606:4700:4700::1111",
                "2606:4700:4700::1001"
            ];

            let mut name_servers = Vec::new();
            for ip_str in doh_ips {
                if let Ok(ip) = std::net::IpAddr::from_str(ip_str) {
                    // 创建HTTPS DoH连接配置
                    let name_server = hickory_resolver::config::NameServerConfig::https(
                        ip,
                        std::sync::Arc::from(doh_host),
                        Some(std::sync::Arc::from("/dns-query"))
                    );
                    name_servers.push(name_server);
                }
            }

            let resolver_config = ResolverConfig::from_parts(None, vec![], name_servers);

            let resolver = Resolver::builder_with_config(
                resolver_config,
                hickory_resolver::proto::runtime::TokioRuntimeProvider::default()
            )
            .build()
            .context("Failed to create Hickory resolver with DoH")?;

            // 解析域名为Name
            let name = Name::from_ascii(&task.doh_resolve_domain)
                .context("Failed to parse domain name")?;

            // 执行DNS查询 - 同时查询A和AAAA记录
            match resolver.lookup_ip(name).await {
                Ok(lookup) => {
                    for ip in lookup.iter() {
                        ips.insert(ip);
                        println!("    -> 从DoH (RFC 8484) 找到IP: {}", ip);
                    }
                }
                Err(e) => {
                    println!("    -> DoH (RFC 8484) 查询失败: {:?}", e);
                    // 回退到简单的HTTP JSON API
                    fallback_to_json_api(client, task, &mut ips).await?;
                }
            }
        }
        "a_aaaa" => {
            // 直接A/AAAA记录查询
            println!("    -> 使用传统DNS查询: {}", task.doh_resolve_domain);
            let resolver = Resolver::builder_tokio()?
                .build()
                .context("Failed to create Hickory resolver")?;

            let name = Name::from_ascii(&task.doh_resolve_domain)
                .context("Failed to parse domain name")?;

            match resolver.lookup_ip(name).await {
                Ok(lookup) => {
                    for ip in lookup.iter() {
                        ips.insert(ip);
                        println!("    -> 从A/AAAA记录找到IP: {}", ip);
                    }
                }
                Err(e) => {
                    println!("    -> 传统DNS查询失败: {:?}", e);
                    fallback_to_json_api(client, task, &mut ips).await?;
                }
            }
        }
        "direct" => {
            // 直接模式已在开头处理
            return Ok(ips.into_iter().collect());
        }
        _ => {
            return Err(anyhow::anyhow!("不支持的解析模式: {}", task.resolve_mode));
        }
    }

    // 如果仍然没有IP，尝试备用IP
    if ips.is_empty() && task.doh_resolve_domain.contains("cloudflare.com") {
        println!("    -> 使用备用的Cloudflare IP...");
        add_fallback_cloudflare_ips(&mut ips);
    }

    let mut ip_vec = ips.into_iter().collect::<Vec<_>>();
    ip_vec.sort_by_key(|ip| ip.is_ipv6());

    Ok(ip_vec)
}

// 回退到JSON API（兼容性）
async fn fallback_to_json_api(
    client: &Client,
    task: &InputTask,
    ips: &mut HashSet<IpAddr>,
) -> Result<()> {
    println!("    -> 回退到JSON API查询");
    let doh_api_url = format!("{}?name={}&type=A", task.doh_url, task.doh_resolve_domain);

    match client.get(&doh_api_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(answer) = json.get("Answer").and_then(|a| a.as_array()) {
                        for item in answer {
                            if let Some(data_str) = item.get("data").and_then(|d| d.as_str()) {
                                if let Ok(ip) = IpAddr::from_str(data_str) {
                                    ips.insert(ip);
                                    println!("    -> 从JSON API找到IP: {}", ip);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => println!("    -> JSON API查询失败: {:?}", e),
    }
    Ok(())
}

// 添加备用Cloudflare IP
fn add_fallback_cloudflare_ips(ips: &mut HashSet<IpAddr>) {
    let fallback_ips = [
        "162.159.140.220",
        "104.16.123.64",
        "172.67.214.232",
        "2606:4700:4700::1",
    ];

    for ip_str in &fallback_ips {
        if let Ok(ip) = IpAddr::from_str(ip_str) {
            ips.insert(ip);
        }
    }
}

// --- 4. HTTP连接测试 ---
async fn test_connectivity(task: InputTask, ip: IpAddr, dns_source: String) -> TestResult {
    let url = format!("https://{}:{}/", task.test_sni_host, task.port);
    let socket_addr = SocketAddr::new(ip, task.port);
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    let client = match Client::builder()
        .resolve_to_addrs(&task.test_sni_host, &[socket_addr])
        // .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(5))
        .no_proxy()
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return TestResult::fail(&task, &ip.to_string(), ip_ver, e.to_string(), dns_source)
        }
    };

    let start = Instant::now();

    match client
        .get(&url)
        .header("Host", &task.test_host_header)
        .header("User-Agent", "curl/8.12.1")
        .send()
        .await
    {
        Ok(res) => {
            let latency = start.elapsed().as_millis() as u64;
            let status = res.status().as_u16();
            let server = res
                .headers()
                .get("server")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let protocol = match res.version() {
                reqwest::Version::HTTP_11 => "http/1.1",
                reqwest::Version::HTTP_2 => "h2",
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

// --- 5. 主程序入口 ---
#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("Failed to create HTTP client");

    let input_json = r#"
    [
        {
            "doh_resolve_domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test_sni_host": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test_host_header": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "doh_url": "https://fresh-reverse-proxy-middle.masx201.dpdns.org/token/4yF6nSCifSLs8lfkb4t8OWP69kfpgiun/https/security.cloudflare-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": null,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://fresh-reverse-proxy-middle.masx201.dpdns.org/token/4yF6nSCifSLs8lfkb4t8OWP69kfpgiun/https/security.cloudflare-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": false,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://fresh-reverse-proxy-middle.masx201.dpdns.org/token/4yF6nSCifSLs8lfkb4t8OWP69kfpgiun/https/security.cloudflare-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": false,
            "direct_ips": ["162.159.140.220", "172.67.214.232"],
            "resolve_mode": "direct"
        }
    ]
    "#;

    let tasks: Vec<InputTask> =
        serde_json::from_str(input_json).context("Invalid JSON format in input")?;

    let mut futures = Vec::new();

    for task in tasks {
        println!(
            ">>> 正在通过 {} 解析 {} 的记录 (模式: {})...",
            task.doh_url, task.doh_resolve_domain, task.resolve_mode
        );

        match resolve_domain_with_hickory(&client, &task).await {
            Ok(ips) => {
                if ips.is_empty() {
                    println!("    [!] 未找到IP地址");
                    continue;
                }
                println!("    -> 解析成功，获取到 {} 个IP地址: {:?}", ips.len(), ips);

                for ip in ips {
                    if let Some(prefer_ipv6) = task.prefer_ipv6 {
                        if prefer_ipv6 != ip.is_ipv6() {
                            continue;
                        }
                    }

                    let task_clone = task.clone();
                    let dns_source = if task.resolve_mode == "direct" {
                        "Direct Input".to_string()
                    } else {
                        format!("DoH ({})", task.doh_url)
                    };

                    futures.push(tokio::spawn(async move {
                        test_connectivity(task_clone, ip, dns_source).await
                    }));
                }
            }
            Err(e) => {
                eprintln!("    [X] DNS解析失败: {:?}", e);
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
