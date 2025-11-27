# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## 项目概述

这是一个用Rust开发的HTTP/3连接测试工具，主要用于测试Cloudflare和其他支持HTTP/3的服务的连通性和性能。项目使用DNS
over HTTPS (DoH)解析目标域名的IP地址，并进行HTTP/3连接测试。

## 核心功能

### DNS解析功能

- **DoH JSON API**: 调用Google DNS、Cloudflare DNS等DoH服务
- **HTTPS记录解析**: 解析HTTPS (类型65) DNS记录以获取SVCB参数
- **A/AAAA记录解析**: 支持传统的IPv4和IPv6地址解析
- **直接IP模式**: 支持手动指定IP地址进行测试

### HTTP/3连接测试

- 使用reqwest库进行HTTP/3协议通信
- 支持SNI (Server Name Indication) 配置
- 可配置目标端口 (默认443)
- 收集连接指标：延迟、协议版本、响应码等

## 关键架构决策

### DNS解析器演进

**重要**: 项目已从正则表达式DNS解析迁移到符合RFC 8484标准的解析方式：

1. **原始实现**: 使用简单正则表达式匹配 `ipv4hint=` 和 `ipv6hint=` 模式
2. **当前实现**: 优先使用RFC 8484标准，保留正则表达式作为回退方案
3. **未来计划**: 完整的trust-dns库集成以支持完整的二进制DNS消息解析

### 错误处理策略

- 使用anyhow库进行统一的错误处理
- DNS解析失败时提供清晰的错误信息
- 网络连接错误时记录详细信息

## 常用命令

### 构建和测试

```bash
# 构建项目
cargo build --release

# 运行测试
cargo test

# 运行程序
cargo run --release
```

### 开发环境变量

```bash
# 启用HTTP/3不稳定特性 (如需要)
RUSTFLAGS='--cfg reqwest_unstable' cargo build

# 使用特定DNS解析器
RUSTFLAGS='--cfg reqwest_unstable' cargo run
```

## 代码结构

```
src/
├── main.rs              # 主程序入口和核心逻辑
├── dns_parser.rs       # DNS解析相关功能 (未来模块化)
├── http_client.rs      # HTTP/3客户端封装 (未来模块化)
└── types.rs           # 数据结构定义 (未来模块化)
```

## 配置文件格式

程序接受JSON格式的配置，支持以下字段：

```json
[
  {
    "doh_resolve_domain": "example.com",
    "test_sni_host": "example.com",
    "test_host_header": "example.com",
    "doh_url": "https://fresh-reverse-proxy-middle.masx201.dpdns.org/token/4yF6nSCifSLs8lfkb4t8OWP69kfpgiun/https/dns.adguard-dns.com/dns-query",
    "port": 443,
    "prefer_ipv6": false,
    "resolve_mode": "https", // "https", "a_aaaa", "direct"
    "direct_ips": ["1.1.1.1", "2606:4700:4700::1"] // 仅用于direct模式
  }
]
```

## 开发指南

### 添加新的DoH服务提供商

1. 在 `InputTask` 结构中添加新的URL模板
2. 确保URL格式支持正确的查询参数
3. 测试新服务的响应格式兼容性

### 添加新的DNS记录类型支持

1. 在 `parse_https_hints` 函数中添加新的解析逻辑
2. 更新RFC 8484兼容的解析器
3. 添加相应的测试用例

### HTTP/3协议扩展

1. 在reqwest配置中添加新的HTTP/3设置
2. 实现特定的ALPN协议协商
3. 添加性能测试和基准测试

## 测试策略

### 单元测试

- 运行 `cargo test` 执行所有单元测试
- 重点测试DNS解析逻辑的正确性
- 验证HTTP/3连接建立过程

### 集成测试

- 使用真实HTTP/3服务进行端到端测试
- 测试不同网络条件下的性能
- 验证与主流Cloudflare服务的兼容性

### 性能基准

- 使用内置的延迟测量功能
- 对比不同DoH服务的响应时间
- 分析HTTP/3 vs HTTP/2 vs HTTP/1.1性能差异

## 部署注意事项

### 生产环境

- 确保使用TLS证书验证
- 配置适当的超时设置
- 监控连接池和资源使用

### 安全考虑

- 不在日志中记录敏感信息
- 验证DNS响应的完整性
- 使用安全的HTTP客户端配置

## 故障排除

### 常见问题

1. **DNS解析失败**: 检查DoH服务URL和网络连接
2. **HTTP/3握手失败**: 确认目标服务器支持HTTP/3
3. **证书错误**: 验证SNI配置和证书链
4. **性能问题**: 检查网络延迟和服务器响应时间

### 调试技巧

- 使用 `-vv` 参数增加详细日志输出
- 先用简单的A/AAA记录测试连通性
- 逐步增加复杂性：先测试HTTP/1.1，再HTTP/2，最后HTTP/3

## 贡献指南

### 代码风格

- 使用 `cargo fmt` 进行代码格式化
- 使用 `cargo clippy` 进行静态分析
- 遵循Rust官方命名约定

### 提交流程

1. 创建功能分支：`git checkout -b feature/new-feature`
2. 提交流码：`git commit -m "feat: add new feature"`
3. 推送分支：`git push origin feature/new-feature`
4. 创建Pull Request并描述变更内容

### 文档更新

- 更新README.md中的新功能说明
- 添加代码注释解释复杂逻辑
- 更新CHANGELOG.md记录重要变更

## 版本历史

### v0.1.0 (当前)

- 基本的HTTP/3连接测试功能
- 支持DoH JSON API DNS解析
- 正则表达式HTTPS记录解析

### v0.2.0 (计划中)

- RFC 8484兼容的DNS解析器
- 改进的错误处理和日志
- 模块化的代码结构

## 许可证

本项目采用MIT许可证。详见LICENSE文件。

## 联系方式

- 项目主页: [GitHub仓库地址]
- 问题反馈: [GitHub Issues]
- 文档: [文档链接]

---

**注意**: 此文档应与代码实现保持同步。当添加新功能时，请相应更新此文件。
