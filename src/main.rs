// Simplified version - focuses on basic DNS resolution and HTTP connection testing
// Now using RFC 8484 DNS over HTTPS (DoH) and Reqwest libraries
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Instant;
use trust_dns_proto::op::{Message, Query};
use trust_dns_proto::rr::{Name, RecordType};
use trust_dns_proto::serialize::binary::BinEncodable;
#[cfg(test)]
mod quinn_test;
// Include DoH and Docs.rs integration tests
#[cfg(test)]
mod doh_docs_test;
#[cfg(test)]
mod test_test;
// #[cfg(test)]
// mod main_new;
mod h3_direct_test;
mod main_h3_test;
#[cfg(test)]
mod http3_test;
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

// --- 3. RFC 8484 DNS over HTTPS (DoH) 实现 ---
// 添加缺失的IPv4地址验证函数
fn is_valid_ipv4_address(ip_str: &str) -> bool {
    match ip_str {
        "0.0.0.0" | "127.0.0.0" | "255.255.255.255" => false, // 排除无效的IPv4地址
        _ => {
            // 使用IPv4地址解析验证
            let parts: Vec<&str> = ip_str.split('.').collect();
            if parts.len() != 4 {
                return false;
            }

            for part in parts {
                if part.parse::<u8>().is_err() {
                    return false;
                }
            }

            // 检查是否为之前那个错误的IP地址
            ip_str != "183.192.65.101" // 排除特定的错误IP
        }
    }
}

// 检查是否为已知的错误IPv4地址
fn is_bad_ipv4_address(ip_str: &str) -> bool {
    ip_str == "183.192.65.101" // 明确标记这个IP为错误
}

// RFC 8484 DNS over HTTPS 查询函数
async fn query_dns_over_https(
    client: &Client,
    domain: &str,
    record_type: RecordType,
    doh_url: &str,
) -> Result<Vec<IpAddr>> {
    // 创建 DNS 查询
    let name = Name::from_ascii(domain).context("Failed to parse domain name")?;
    let query = Query::query(name, record_type);

    // 创建 DNS 消息
    let mut message = Message::new();
    message.set_id(0); // RFC 8484 建议使用 ID 为 0 以提高缓存效率
    message.set_recursion_desired(true);
    message.add_query(query);

    // 序列化 DNS 查询
    let mut request_bytes = Vec::new();
    {
        let mut encoder = trust_dns_proto::serialize::binary::BinEncoder::new(&mut request_bytes);
        message
            .emit(&mut encoder)
            .context("Failed to serialize DNS query")?;
    }

    // 使用 base64url 编码（不包含填充）
    let encoded_query = general_purpose::URL_SAFE_NO_PAD.encode(&request_bytes);

    // 构建 DoH 请求 URL
    let url = format!("{}?dns={}", doh_url, encoded_query);

    // 发送 HTTPS GET 请求
    let response = client
        .get(&url)
        .header("Accept", "application/dns-message")
        .send()
        .await
        .context("Failed to send DoH request")?;

    // 检查响应状态
    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow::anyhow!(
            "DoH server returned non-200 status: {}",
            response.status()
        ));
    }

    // 获取响应体
    let response_bytes = response
        .bytes()
        .await
        .context("Failed to read response body")?;

    // 解析 DNS 响应
    let dns_response =
        Message::from_vec(&response_bytes).context("Failed to parse DNS response")?;

    // 提取 IP 地址
    let mut ip_addresses = Vec::new();
    let answers = dns_response.answers();

    if !answers.is_empty() {
        for record in answers {
            if record.record_type() == record_type {
                if let Some(rdata) = record.data() {
                    match record.record_type() {
                        RecordType::A => {
                            if let trust_dns_proto::rr::RData::A(ipv4) = rdata {
                                ip_addresses.push(IpAddr::V4(*ipv4));
                            }
                        }
                        RecordType::AAAA => {
                            if let trust_dns_proto::rr::RData::AAAA(ipv6) = rdata {
                                ip_addresses.push(IpAddr::V6(*ipv6));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(ip_addresses)
}

async fn resolve_domain_with_rfc8484(client: &Client, task: &InputTask) -> Result<Vec<IpAddr>> {
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
            // 使用 RFC 8484 标准的 DoH 查询
            println!("    -> 使用 RFC 8484 DoH 查询: {}", task.doh_resolve_domain);

            // 查询 A 记录 (IPv4)
            match query_dns_over_https(
                client,
                &task.doh_resolve_domain,
                RecordType::A,
                &task.doh_url,
            )
            .await
            {
                Ok(mut ipv4_addresses) => {
                    ipv4_addresses.retain(|ip| {
                        let ip_str = ip.to_string();
                        is_valid_ipv4_address(&ip_str) && !is_bad_ipv4_address(&ip_str)
                    });

                    for ip in &ipv4_addresses {
                        ips.insert(*ip);
                        println!("    -> 从 RFC 8484 DoH 找到 IPv4: {}", ip);
                    }

                    // 查询 AAAA 记录 (IPv6)
                    match query_dns_over_https(
                        client,
                        &task.doh_resolve_domain,
                        RecordType::AAAA,
                        &task.doh_url,
                    )
                    .await
                    {
                        Ok(ipv6_addresses) => {
                            for ip in &ipv6_addresses {
                                ips.insert(*ip);
                                println!("    -> 从 RFC 8484 DoH 找到 IPv6: {}", ip);
                            }
                        }
                        Err(e) => {
                            println!("    -> RFC 8484 DoH IPv6 查询失败: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("    -> RFC 8484 DoH 查询失败: {:?}", e);
                }
            }
        }
        "a_aaaa" => {
            // 使用 DoH 查询 A 和 AAAA 记录
            println!("    -> 使用 DoH 查询: {}", task.doh_resolve_domain);

            // 查询 A 记录 (IPv4)
            match query_dns_over_https(
                client,
                &task.doh_resolve_domain,
                RecordType::A,
                &task.doh_url,
            )
            .await
            {
                Ok(mut ipv4_addresses) => {
                    ipv4_addresses.retain(|ip| {
                        let ip_str = ip.to_string();
                        is_valid_ipv4_address(&ip_str) && !is_bad_ipv4_address(&ip_str)
                    });

                    for ip in &ipv4_addresses {
                        ips.insert(*ip);
                        println!("    -> 从 DoH 找到 IPv4: {}", ip);
                    }
                }
                Err(e) => {
                    println!("    -> DoH IPv4 查询失败: {:?}", e);
                }
            }

            // 查询 AAAA 记录 (IPv6)
            match query_dns_over_https(
                client,
                &task.doh_resolve_domain,
                RecordType::AAAA,
                &task.doh_url,
            )
            .await
            {
                Ok(ipv6_addresses) => {
                    for ip in &ipv6_addresses {
                        ips.insert(*ip);
                        println!("    -> 从 DoH 找到 IPv6: {}", ip);
                    }
                }
                Err(e) => {
                    println!("    -> DoH IPv6 查询失败: {:?}", e);
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
            // 检查是否为有效的IPv4地址且不是之前那个错误地址
            if is_valid_ipv4_address(ip_str) && !is_bad_ipv4_address(ip_str) {
                ips.insert(ip);
            }
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
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": true,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": true,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "speed.cloudflare.com",
            "test_sni_host": "speed.cloudflare.com",
            "test_host_header": "speed.cloudflare.com",
            "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query",
            "port": 443,
            "prefer_ipv6": true,
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

        match resolve_domain_with_rfc8484(&client, &task).await {
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
