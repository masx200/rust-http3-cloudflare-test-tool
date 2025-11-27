// 纯 HTTP/3 测试工具 - 使用 h3 库
mod main_h3_only;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_h3_only::main().await
}
