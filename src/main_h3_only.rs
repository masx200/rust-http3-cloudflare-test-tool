// 纯 HTTP/3 测试工具 - 基于 h3 库
use anyhow::{anyhow, Context, Result};
use bytes::Buf;
use clap::{Arg, Command};
use futures::future;
use h3::error::ConnectionError;
use h3_quinn::quinn;
use rustls_native_certs::load_native_certs;
use std::sync::Arc;
use tracing::{error, info};

// 错误转换辅助函数
fn h3_error_to_anyhow(e: impl std::error::Error + Send + Sync + 'static) -> anyhow::Error {
    anyhow!("{:?}", e)
}

static ALPN: &[u8] = b"h3";

#[derive(Debug, Clone)]
pub struct H3TestConfig {
    pub domain: String,
    pub port: u16,
    pub path: String,
    pub doh_server: String,
    pub timeout_seconds: u64,
}

impl Default for H3TestConfig {
    fn default() -> Self {
        Self {
            domain: "cloudflare.com".to_string(),
            port: 443,
            path: "/".to_string(),
            doh_server: "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query".to_string(),
            timeout_seconds: 10,
        }
    }
}

pub struct H3Tester {
    config: H3TestConfig,
}

impl H3Tester {
    pub fn new(config: H3TestConfig) -> Self {
        Self { config }
    }

    pub async fn test_connection(&self) -> Result<()> {
        info!("🚀 开始 HTTP/3 测试: {}:{}", self.config.domain, self.config.port);

        // 1. DNS 解析
        let mut addrs = tokio::net::lookup_host((self.config.domain.as_str(), self.config.port))
            .await
            .context("DNS 解析失败")?;

        let addr = addrs.next().ok_or_else(|| anyhow::anyhow!("未找到 DNS 地址"))?;

        info!("✅ DNS 解析成功: {} -> {}", self.config.domain, addr);

        // 2. 加载证书
        let mut roots = rustls::RootCertStore::empty();
        match load_native_certs() {
            Ok(certs) => {
                for cert in certs {
                    if let Err(e) = roots.add(cert) {
                        error!("解析信任锚失败: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("加载系统证书失败: {}", e);
            }
        }

        // 3. 配置 TLS
        let mut tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();

        tls_config.enable_early_data = true;
        tls_config.alpn_protocols = vec![ALPN.into()];

        // 4. 创建 QUIC 端点
        let mut client_endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())
            .context("创建 QUIC 客户端端点失败")?;

        let client_config = quinn::ClientConfig::new(Arc::new(
            quinn::crypto::rustls::QuicClientConfig::try_from(tls_config)
                .context("创建 QUIC TLS 配置失败")?,
        ));
        client_endpoint.set_default_client_config(client_config);

        // 5. 建立连接
        let start = std::time::Instant::now();
        let conn = client_endpoint
            .connect(addr, &self.config.domain)
            .context("连接建立失败")?
            .await
            .context("连接超时或被拒绝")?;

        let connect_time = start.elapsed();
        info!("✅ QUIC 连接建立成功，耗时: {:?}", connect_time);

        // 6. 创建 H3 客户端
        let quinn_conn = h3_quinn::Connection::new(conn);

        let (mut driver, mut send_request) = h3::client::new(quinn_conn)
            .await
            .context("创建 H3 客户端失败")?;

        // 7. 发送请求
        let uri = format!("https://{}{}", self.config.domain, self.config.path);
        info!("📡 发送 HTTP/3 请求: {}", uri);

        let req = http::Request::builder()
            .uri(uri)
            .header("Host", &self.config.domain)
            .header("User-Agent", "rust-http3-test-tool/1.0")
            .body(())
            .map_err(|e| anyhow!("构建请求失败: {}", e))?;

        let mut stream = send_request.send_request(req)
            .await
            .map_err(h3_error_to_anyhow)?;

        stream.finish()
            .await
            .map_err(h3_error_to_anyhow)?;

        let resp = stream.recv_response()
            .await
            .map_err(h3_error_to_anyhow)?;

        let status = resp.status();
        let version = resp.version();

        info!("📨 收到响应: {} {:?}", status, version);
        info!("📋 响应头: {:#?}", resp.headers());

        // 读取响应体
        let mut total_bytes = 0;
        while let Some(chunk) = stream.recv_data().await.map_err(h3_error_to_anyhow)? {
            total_bytes += chunk.remaining();
        }

        info!("✅ HTTP/3 测试成功！状态码: {}, 响应大小: {} 字节", status, total_bytes);

        // 优雅地关闭连接 - 使用短暂超时等待
        info!("✅ 测试完成，程序即将退出");

        // 使用短暂的超时等待，而不是无限等待
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                info!("等待超时，直接退出");
            }
            _ = client_endpoint.wait_idle() => {
                info!("连接已空闲");
            }
        }

        // 清理资源
        drop(client_endpoint);

        Ok(())
    }
}

// --- 主程序入口 ---
#[tokio::main]
pub async fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let matches = Command::new("rust-http3-test-tool")
        .version("1.0.0")
        .about("HTTP/3 客户端测试工具 - 基于 h3 库")
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("测试域名")
                .default_value("cloudflare.com"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("端口号")
                .default_value("443"),
        )
        .arg(
            Arg::new("path")
                .short('t')
                .long("path")
                .value_name("PATH")
                .help("请求路径")
                .default_value("/"),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("超时时间（秒）")
                .default_value("10"),
        )
        .get_matches();

    let domain = matches.get_one::<String>("domain").unwrap().clone();
    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(443);
    let path = matches.get_one::<String>("path").unwrap().clone();
    let timeout = matches
        .get_one::<String>("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(10);

    let config = H3TestConfig {
        domain,
        port,
        path,
        doh_server: "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query".to_string(),
        timeout_seconds: timeout,
    };

    let tester = H3Tester::new(config);

    if let Err(e) = tester.test_connection().await {
        error!("❌ 测试失败: {:?}", e);
        std::process::exit(1);
    }

    println!("\n✅ HTTP/3 测试完成！");
    Ok(())
}
