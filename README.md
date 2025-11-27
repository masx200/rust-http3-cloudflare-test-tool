# 🚀 HTTP/3 Cloudflare 测试工具

     [![Go Version](https://img.shields.io/badge/Go-1.21+-blue.svg)](https://golang.org/)
     [![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
     [![HTTP/3](https://img.shields.io/badge/Protocol-HTTP%2F3-orange.svg)](https://en.wikipedia.org/wiki/HTTP/3)
     [![QUIC](https://img.shields.io/badge/Transport-QUIC-purple.svg)](https://en.wikipedia.org/wiki/QUIC)

     一个用于测试 Cloudflare HTTP/3 服务的 Go 语言工具，支持多种 DNS 解析方式和协议回退机制。

## ✨ 功能特性

     - 🔥 **HTTP/3 支持** - 基于 QUIC 协议的最新 HTTP 协议
     - 🌐 **多协议回退** - HTTP/3 → HTTP/2 → HTTP/1.1 智能回退
     - 🔍 **多种 DNS 解析** - DoH、DoQ、DoT、传统 DNS 支持
     - ⚡ **并发测试** - 多 IP 地址并发连接测试
     - 📊 **详细报告** - JSON 格式的详细测试结果
     - ⚙️ **灵活配置** - 支持配置文件和命令行参数
     - 🛡️ **IPv4/IPv6** - 完整的双栈 IP 地址支持
     - 🎯 **IP 地址过滤** - 智能过滤无效和特定 IP 地址

## 🚀 快速开始

     ### 安装依赖

     ```bash
     go mod tidy
     ```

     ### 构建项目

     ```bash
     go build -o http3-test-tool main.go
     ```

     ### 运行测试

     ```bash
     # 使用默认配置
     ./http3-test-tool

     # 指定测试域名
     ./http3-test-tool -domain "example.com"

     # 使用配置文件
     ./http3-test-tool -config "config.json"

     # 详细输出模式
     ./http3-test-tool -verbose
     ```

## 📋 使用示例

     ### 1. 基本测试

     ```bash
     # 测试 Cloudflare 服务
     ./http3-test-tool -domain "local-aria2-webui.masx200.ddns-ip.net" -test-url "https://local-aria2-webui.masx200.ddns-ip.net"

     # 指定 DoH 服务
     ./http3-test-tool -domain "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io" \
       -doh-url "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query"
     ```

     ### 2. 使用配置文件

     创建 `config.json`:

     ```json
     [
       {
         "doh_resolve_domain": "local-aria2-webui.masx200.ddns-ip.net",
         "test_sni_host": "local-aria2-webui.masx200.ddns-ip.net",
         "test_host_header": "local-aria2-webui.masx200.ddns-ip.net",
         "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query",
         "port": 443,
         "prefer_ipv6": false,
         "resolve_mode": "https"
       },
       {
         "doh_resolve_domain": "local-aria2-webui.masx200.ddns-ip.net",
         "test_sni_host": "local-aria2-webui.masx200.ddns-ip.net",
         "test_host_header": "local-aria2-webui.masx200.ddns-ip.net",
         "doh_url": "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query",
         "port": 443,
         "prefer_ipv6": false,
         "resolve_mode": "direct",
         "direct_ips": ["162.159.140.220", "172.67.214.232"]
       }
     ]
     ```

     运行测试:

     ```bash
     ./http3-test-tool -config config.json
     ```

     ### 3. DNS 解析模式

     ```bash
     # DNS over HTTPS (DoH) - RFC 8484
     ./http3-test-tool -resolve-mode "https" -domain "example.com"

     # 传统 A/AAAA 记录查询
     ./http3-test-tool -resolve-mode "a_aaaa" -domain "example.com"

     # 直接 IP 地址模式
     ./http3-test-tool -resolve-mode "direct" -domain "example.com"
     ```

## 📊 输出示例

     ```json
     [
       {
         "domain_used": "local-aria2-webui.masx200.ddns-ip.net",
         "target_ip": "162.159.140.220",
         "ip_version": "IPv4",
         "sni_host": "local-aria2-webui.masx200.ddns-ip.net",
         "host_header": "local-aria2-webui.masx200.ddns-ip.net",
         "success": true,
         "status_code": 200,
         "protocol": "h3",
         "latency_ms": 127,
         "server_header": "cloudflare",
         "dns_source": "DoH (https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query)"
       },
       {
         "domain_used": "local-aria2-webui.masx200.ddns-ip.net",
         "target_ip": "2606:4700:d0::a29f:4801",
         "ip_version": "IPv6",
         "sni_host": "local-aria2-webui.masx200.ddns-ip.net",
         "host_header": "local-aria2-webui.masx200.ddns-ip.net",
         "success": true,
         "status_code": 200,
         "protocol": "h2",
         "latency_ms": 156,
         "server_header": "cloudflare",
         "dns_source": "DoH (https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query)"
       }
     ]
     ```

## ⚙️ 配置选项

     ### 命令行参数

     | 参数 | 默认值 | 说明 |
     |------|--------|------|
     | `-config` | "" | 配置文件路径 |
     | `-domain` | "" | 测试域名 |
     | `-doh-url` | "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query" | DoH 服务 URL |
     | `-resolve-mode` | "https" | DNS 解析模式 (https, a_aaaa, direct) |
     | `-test-url` | "https://hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io" | 测试 URL |
     | `-port` | 443 | 目标端口 |
     | `-verbose` | false | 详细输出模式 |

     ### 配置文件格式

     ```json
     {
       "doh_resolve_domain": "要解析的域名",
       "test_sni_host": "SNI 主机名",
       "test_host_header": "HTTP Host 头",
       "doh_url": "DNS over HTTPS 服务 URL",
       "port": 443,
       "prefer_ipv6": false,
       "resolve_mode": "https",
       "direct_ips": ["直接 IP 地址列表"]
     }
     ```

## 🏗️ 项目架构

     ```
     ├── main.go                    # 主程序入口
     ├── config.json                # 配置文件示例
     ├── go.mod                     # Go 模块定义
     ├── README.md                  # 项目文档
     ├── CLAUDE.md                  # Claude 开发指南
     ├── http3-reverse-proxy-server-experiment/  # HTTP/3 实验库
     │   ├── h3/                    # HTTP/3 实现
     │   ├── dns/                   # DNS 解析服务
     │   ├── load_balance/          # 负载均衡
     │   └── adapter/               # 协议适配器
     └── src/                       # 原始 Rust 代码（参考）
         └── main.rs               # Rust 版本实现
     ```

## 🛠️ 开发指南

     ### 构建

     ```bash
     go build -v ./...
     ```

     ### 测试

     ```bash
     # 运行所有测试
     go test -v ./...

     # 运行特定包测试
     go test -v ./h3/
     go test -v ./dns/
     go test -v ./load_balance/
     ```

     ### 性能分析

     ```bash
     # 启用 pprof 调试
     go run main.go -debug-pprof
     ```

## 📚 核心组件

     ### 1. DNS 解析引擎
     - **DoH (DNS over HTTPS)**: RFC 8484 标准实现
     - **传统 DNS**: A/AAAA 记录查询
     - **直接模式**: 使用预定义 IP 地址

     ### 2. HTTP/3 传输层
     - 基于 QUIC 协议实现
     - 连接池和轮询机制
     - 自定义 IP 绑定支持

     ### 3. 协议回退机制
     - HTTP/3 (QUIC) → HTTP/2 (TCP) → HTTP/1.1
     - 自动检测和切换
     - 详细连接状态报告

     ### 4. 负载均衡系统
     - 随机负载均衡算法
     - 主被动健康检查
     - 故障转移策略

## 🔒 安全特性

     - ✅ **IP 地址过滤** - 自动过滤无效和恶意 IP
     - ✅ **SNI 配置** - 支持 Server Name Indication
     - ✅ **超时保护** - 防止连接挂起
     - ✅ **并发控制** - 合理的并发限制

## 🌍 支持的协议

     | 协议 | 说明 | 状态 |
     |------|------|------|
     | HTTP/3 | 基于 QUIC 的下一代 HTTP | ✅ 支持 |
     | HTTP/2 | 二进制帧协议 | ✅ 支持 |
     | HTTP/1.1 | 传统文本协议 | ✅ 支持 |
     | DoH | DNS over HTTPS | ✅ 支持 |
     | DoQ | DNS over QUIC | ✅ 支持 |
     | DoT | DNS over TLS | ✅ 支持 |

## 🤝 贡献指南

     我们欢迎各种形式的贡献！

     1. **Fork 项目**
        ```bash
        git clone https://gitee.com/masx200/golang-http3-cloudflare-test-tool.git
        ```

     2. **创建功能分支**
        ```bash
        git checkout -b feature/amazing-feature
        ```

     3. **提交更改**
        ```bash
        git commit -m 'Add amazing feature'
        ```

     4. **推送分支**
        ```bash
        git push origin feature/amazing-feature
        ```

     5. **创建 Pull Request**

## 📄 许可证

     本项目采用 [MIT 许可证](LICENSE)。

## 🙏 致谢

     - [quic-go](https://github.com/quic-go/quic-go) - QUIC 协议 Go 实现
     - [miekg/dns](https://github.com/miekg/dns) - DNS 库
     - [gin-gonic](https://github.com/gin-gonic/gin) - HTTP Web 框架
     - [Cloudflare](https://local-aria2-webui.masx200.ddns-ip.net/) - HTTP/3 服务支持

## 📞 联系方式

     - 项目主页: [Gitee](https://gitee.com/masx200/golang-http3-cloudflare-test-tool)
     - 问题反馈: [Issues](https://gitee.com/masx200/golang-http3-cloudflare-test-tool/issues)

     ---

     <div align="center">

     **🌟 如果这个项目对您有帮助，请给我们一个 Star！🌟**

     Made with ❤️ by [masx200](https://gitee.com/masx200)

     </div>
