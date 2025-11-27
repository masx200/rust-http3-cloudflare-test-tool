// HTTP/3 直接测试模块 - 使用 h3 库进行原生 HTTP/3 测试
use anyhow::{Context, Result};
use bytes::{Bytes, Buf};
use h3::{
    client::{builder, SendRequest},
    quic,
};
use http::{Method, HeaderMap, HeaderValue};
use h3_quinn::quinn;
use quinn::{ClientConfig, Endpoint, TransportConfig};
use rcgen::CertificateParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use rustls::pki_types::ServerName;
use rustls::{ClientConfig as RustlsClientConfig, RootCertStore};

// --- 1. HTTP/3 测试配置 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct H3TestConfig {
    pub target_domain: String,
    pub target_ip: String,
    pub port: u16,
    pub sni_host: String,
    pub test_path: String,
    pub user_agent: Option<String>,
    pub max_field_section_size: Option<u64>,
    pub enable_datagram: bool,
    pub enable_extended_connect: bool,
    pub send_grease: bool,
    pub timeout_seconds: u64,
}

impl Default for H3TestConfig {
    fn default() -> Self {
        Self {
            target_domain: "cloudflare.com".to_string(),
            target_ip: "104.16.123.64".to_string(),
            port: 443,
            sni_host: "cloudflare.com".to_string(),
            test_path: "/".to_string(),
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_field_section_size: Some(8192),
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: true,
            timeout_seconds: 10,
        }
    }
}

// --- 2. HTTP/3 测试结果 ---
#[derive(Debug, Clone, Serialize)]
pub struct H3TestResult {
    pub config: H3TestConfig,
    pub target_ip: String,
    pub ip_version: String,
    pub success: bool,
    pub protocol_version: String,
    pub response_status: Option<u16>,
    pub response_headers: HashMap<String, String>,
    pub response_size: Option<usize>,
    pub latency_ms: Option<u64>,
    pub error_message: Option<String>,
    pub alpn_protocol: String,
    pub cipher_suite: Option<String>,
    pub server_name_indication: String,
    pub connection_id: Option<String>,
    pub stream_id: Option<u64>,
    pub test_timestamp: String,
}

impl H3TestResult {
    pub fn success(config: &H3TestConfig, ip: &str, version: &str) -> Self {
        Self {
            config: config.clone(),
            target_ip: ip.to_string(),
            ip_version: version.to_string(),
            success: true,
            protocol_version: "HTTP/3".to_string(),
            response_status: Some(200),
            response_headers: HashMap::new(),
            response_size: Some(0),
            latency_ms: Some(0),
            error_message: None,
            alpn_protocol: "h3".to_string(),
            cipher_suite: None,
            server_name_indication: config.sni_host.clone(),
            connection_id: None,
            stream_id: None,
            test_timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn failure(config: &H3TestConfig, ip: &str, version: &str, error: String) -> Self {
        Self {
            config: config.clone(),
            target_ip: ip.to_string(),
            ip_version: version.to_string(),
            success: false,
            protocol_version: "HTTP/3".to_string(),
            response_status: None,
            response_headers: HashMap::new(),
            response_size: None,
            latency_ms: None,
            error_message: Some(error),
            alpn_protocol: "h3".to_string(),
            cipher_suite: None,
            server_name_indication: config.sni_host.clone(),
            connection_id: None,
            stream_id: None,
            test_timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// --- 3. HTTP/3 测试器 ---
pub struct H3Tester {
    client_config: ClientConfig,
    transport_config: TransportConfig,
}

impl H3Tester {
    pub fn new() -> Result<Self> {
        // 配置 TLS
        let mut root_store = RootCertStore::empty();
        root_store.add_parsable_certificates(
            rustls_native_certs::load_native_certs()
                .context("Failed to load native certificates")?,
        );

        let crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let mut client_config = ClientConfig::new(Arc::new(crypto));
        root_store.add_parsable_certificates(
            rustls_native_certs::load_native_certs()
                .context("Failed to load native certificates")?
        );

        let tls_config = RustlsClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        client_config.crypto = Arc::new(tls_config);

        // 配置 ALPN
        client_config.alpn_protocols = vec![
            "h3".into(),
            "h3-29".into(),
            "h3-32".into(),
            "h3-33".into(),
            "h3-34".into(),
        ];

        let mut transport_config = TransportConfig::default();
        transport_config.max_idle_timeout(Some(Duration::from_secs(10)));
        transport_config.max_concurrent_uni_streams(100u32.into());
        transport_config.max_concurrent_bidi_streams(100u32.into());
        transport_config.datagram_send_buffer_size(1024 * 1024);

        Ok(Self {
            client_config,
            transport_config,
        })
    }

    pub async fn test_http3_connection(&self, config: &H3TestConfig) -> Result<H3TestResult> {
        let start_time = Instant::now();
        let ip_addr: IpAddr = config.target_ip.parse()
            .context("Invalid IP address")?;
        let socket_addr = SocketAddr::new(ip_addr, config.port);

        let ip_version = if ip_addr.is_ipv6() { "IPv6" } else { "IPv4" };

        println!("    -> 开始 HTTP/3 测试: {} ({})", config.target_domain, socket_addr);

        // 创建 QUIC 端点
        let endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        let mut quic_conn = endpoint
            .connect_with(self.client_config.clone(), socket_addr, &config.sni_host)?
            .await
            .context("Failed to establish QUIC connection")?;

        println!("    -> QUIC 连接建立成功");

        // 创建 HTTP/3 连接
        let mut h3_builder = builder();
        if let Some(max_size) = config.max_field_section_size {
            h3_builder.max_field_section_size(max_size);
        }
        h3_builder.enable_datagram(config.enable_datagram);
        h3_builder.enable_extended_connect(config.enable_extended_connect);
        h3_builder.send_grease(config.send_grease);

        let (mut h3_conn, mut h3_request) = h3_builder
            .build(quic_conn)
            .await
            .context("Failed to build HTTP/3 connection")?;

        println!("    -> HTTP/3 连接建立成功");

        // 发送请求
        let request_url = format!("https://{}{}{}", config.target_domain, config.port, config.test_path);
        let user_agent = config.user_agent.as_deref().unwrap_or("rust-h3-test-tool/1.0");

        let mut request = h3_request
            .send_request(Method::GET, &request_url)
            .await
            .context("Failed to send HTTP/3 request")?;

        // 设置请求头 - 注意：这些头设置方法可能需要根据h3库的API调整
        // h3::ext::Header::Method(&mut request, Method::GET);
        // h3::ext::Header::Scheme(&mut request, h3::ext::Scheme::Https);
        // h3::ext::Header::Authority(&mut request, &config.target_domain);
        // h3::ext::Header::Path(&mut request, &config.test_path);
        // h3::ext::Header::UserAgent(&mut request, user_agent);
        // h3::ext::Header::Accept(&mut request, "*/*");

        println!("    -> HTTP/3 请求已发送: {} {}", Method::GET, config.test_path);

        // 接收响应
        let timeout_duration = Duration::from_secs(config.timeout_seconds);
        let response_result = timeout(timeout_duration, async {
            let mut response = request.recv_response().await?;
            let mut response_headers = HashMap::new();

            // 读取响应头
            while let Some(header) = response.recv_header().await? {
                let name = String::from_utf8_lossy(header.name);
                let value = String::from_utf8_lossy(header.value);
                response_headers.insert(name.to_string(), value.to_string());
                println!("    -> 响应头: {} = {}", name, value);
            }

            let status = response.status();
            let mut response_size = 0u64;

            // 读取响应体
            let mut body_data = Vec::new();
            while let Some(chunk) = response.recv_data().await? {
                response_size += chunk.len() as u64;
                body_data.extend_from_slice(&chunk);
            }

            println!("    -> 响应状态: {}", status);
            println!("    -> 响应大小: {} bytes", response_size);

            Ok::<(h3::ext::StatusCode, HashMap<String, String>, usize), anyhow::Error>(
                (status, response_headers, response_size as usize)
            )
        }).await;

        let latency = start_time.elapsed().as_millis() as u64;

        match response_result {
            Ok(Ok((status, headers, size))) => {
                let mut result = H3TestResult::success(config, &config.target_ip, ip_version);
                result.response_status = Some(status.as_u16());
                result.response_headers = headers;
                result.response_size = Some(size);
                result.latency_ms = Some(latency);
                result.protocol_version = "HTTP/3".to_string();

                println!("    -> HTTP/3 测试成功: {} - {}ms - {} bytes", status, latency, size);
                Ok(result)
            }
            Ok(Err(e)) => {
                let error_msg = format!("HTTP/3 response error: {}", e);
                println!("    -> {}", error_msg);
                Ok(H3TestResult::failure(config, &config.target_ip, ip_version, error_msg))
            }
            Err(_) => {
                let error_msg = format!("HTTP/3 request timeout after {} seconds", config.timeout_seconds);
                println!("    -> {}", error_msg);
                Ok(H3TestResult::failure(config, &config.target_ip, ip_version, error_msg))
            }
        }
    }

    pub async fn test_http3_connectivity(&self, configs: &[H3TestConfig]) -> Result<Vec<H3TestResult>> {
        println!("🚀 开始 HTTP/3 连通性测试");
        println!("================================");

        let mut results = Vec::new();
        let mut tasks = Vec::new();

        for config in configs {
            println!("正在测试: {} ({})", config.target_domain, config.target_ip);

            let tester = self.clone();
            let config_clone = config.clone();

            let task = tokio::spawn(async move {
                match tester.test_http3_connection(&config_clone).await {
                    Ok(result) => result,
                    Err(e) => H3TestResult::failure(
                        &config_clone,
                        &config_clone.target_ip,
                        if config_clone.target_ip.contains(':') { "IPv6" } else { "IPv4" },
                        format!("Test execution error: {}", e),
                    ),
                }
            });

            tasks.push(task);
        }

        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => println!("任务执行失败: {:?}", e),
            }
        }

        Ok(results)
    }
}

impl Clone for H3Tester {
    fn clone(&self) -> Self {
        Self {
            client_config: self.client_config.clone(),
            transport_config: self.transport_config.clone(),
        }
    }
}

// --- 4. 协议信息提取 ---
pub fn extract_protocol_info(connection: &quinn::Connection) -> (String, Option<String>) {
    let alpn = connection.alpn_protocol()
        .map(|p| p.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let cipher_suite = connection.crypto_session()
        .and_then(|s| s.suite())
        .map(|s| format!("{:?}", s));

    (alpn, cipher_suite)
}

// --- 5. 测试报告生成 ---
pub fn generate_test_report(results: &[H3TestResult]) -> String {
    let mut report = String::new();
    report.push_str("=== HTTP/3 测试报告 ===\n\n");

    let successful = results.iter().filter(|r| r.success).count();
    let total = results.len();

    report.push_str(&format!("总测试数: {}\n", total));
    report.push_str(&format!("成功: {}\n", successful));
    report.push_str(&format!("失败: {}\n\n", total - successful));

    // 按域名分组
    let mut grouped: HashMap<String, Vec<&H3TestResult>> = HashMap::new();
    for result in results {
        grouped.entry(result.config.target_domain.clone())
            .or_default()
            .push(result);
    }

    for (domain, domain_results) in grouped {
        report.push_str(&format!("📡 域名: {}\n", domain));
        report.push_str(&format!("{}\n", "=".repeat(50)));

        for result in domain_results {
            if result.success {
                report.push_str(&format!(
                    "✅ {} ({}) - {}ms - {} bytes - {}\n",
                    result.target_ip,
                    result.ip_version,
                    result.latency_ms.unwrap_or(0),
                    result.response_size.unwrap_or(0),
                    result.alpn_protocol
                ));
            } else {
                report.push_str(&format!(
                    "❌ {} ({}) - 错误: {}\n",
                    result.target_ip,
                    result.ip_version,
                    result.error_message.as_deref().unwrap_or("未知错误")
                ));
            }
        }
        report.push('\n');
    }

    // 延迟统计
    let latencies: Vec<u64> = results.iter()
        .filter_map(|r| r.latency_ms)
        .collect();

    if !latencies.is_empty() {
        let avg_latency = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;
        let min_latency = latencies.iter().min().unwrap();
        let max_latency = latencies.iter().max().unwrap();

        report.push_str("⏱️  延迟统计 (ms):\n");
        report.push_str(&format!("平均: {:.2}\n", avg_latency));
        report.push_str(&format!("最小: {}\n", min_latency));
        report.push_str(&format!("最大: {}\n", max_latency));
    }

    report
}

// --- 6. 预定义测试配置 ---
pub fn get_default_h3_test_configs() -> Vec<H3TestConfig> {
    vec![
        H3TestConfig {
            target_domain: "cloudflare.com".to_string(),
            target_ip: "104.16.123.64".to_string(),
            port: 443,
            sni_host: "cloudflare.com".to_string(),
            test_path: "/cdn-cgi/trace".to_string(),
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_field_section_size: Some(8192),
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: true,
            timeout_seconds: 10,
        },
        H3TestConfig {
            target_domain: "google.com".to_string(),
            target_ip: "142.250.196.206".to_string(),
            port: 443,
            sni_host: "google.com".to_string(),
            test_path: "/".to_string(),
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_field_section_size: Some(8192),
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: true,
            timeout_seconds: 10,
        },
        H3TestConfig {
            target_domain: "facebook.com".to_string(),
            target_ip: "31.13.66.35".to_string(),
            port: 443,
            sni_host: "facebook.com".to_string(),
            test_path: "/".to_string(),
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_field_section_size: Some(8192),
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: true,
            timeout_seconds: 10,
        },
    ]
}