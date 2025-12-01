#!/usr/bin/env node

import { existsSync, readFileSync, writeFileSync } from "fs";
import { basename, join } from "path";

/**
 * 生成HTTP/3连接测试失败报告
 * 从connectivity_results.json中提取所有失败的测试结果并生成格式化报告
 */

class TestReportGenerator {
  constructor(resultsFilePath) {
    this.resultsFilePath = resultsFilePath;
    this.failedTests = [];
    this.statistics = {
      total: 0,
      failed: 0,
      success: 0,
      failureRate: 0,
    };
  }

  /**
   * 读取并解析测试结果文件
   */
  loadResults() {
    try {
      console.log("正在读取测试结果文件...");
      const fileContent = readFileSync(this.resultsFilePath, "utf8");
      const results = JSON.parse(fileContent);

      console.log(`成功读取 ${results.length} 条测试记录`);
      return results;
    } catch (error) {
      console.error("读取测试结果文件失败:", error.message);
      process.exit(1);
    }
  }

  /**
   * 分析测试结果，提取失败的测试
   */
  analyzeResults(results) {
    console.log("正在分析测试结果...");

    this.statistics.total = results.length;

    results.forEach((result, index) => {
      if (result.success === false) {
        this.failedTests.push({
          index: index + 1,
          host: result.host || "Unknown",
          target_ip: result.target_ip || "Unknown",
          ip_version: result.ip_version || "Unknown",
          protocol: result.protocol || "none",
          status_code: result.status_code,
          latency_ms: result.latency_ms || 0,
          server_header: result.server_header || "N/A",
          error_msg: result.error_msg || "No error message",
          timestamp: result.timestamp || new Date().toISOString(),
        });
      } else if (result.success === true) {
        this.statistics.success++;
      }
    });

    this.statistics.failed = this.failedTests.length;
    this.statistics.failureRate = (
      (this.statistics.failed / this.statistics.total) *
      100
    ).toFixed(2);

    console.log(
      `分析完成: 失败 ${this.statistics.failed} 条，成功 ${this.statistics.success} 条`,
    );
  }

  /**
   * 生成Markdown格式的报告
   */
  generateMarkdownReport() {
    const reportDate = new Date().toLocaleString("zh-CN");

    let report = `# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: ${reportDate}
- **数据来源**: ${basename(this.resultsFilePath)}
- **总测试数**: ${this.statistics.total}
- **失败测试数**: ${this.statistics.failed}
- **成功测试数**: ${this.statistics.success}
- **失败率**: ${this.statistics.failureRate}%

---

## 失败测试详情

`;

    if (this.failedTests.length === 0) {
      report += `🎉 **恭喜！所有测试都成功了！**\n\n`;
    } else {
      // 按错误类型分组统计
      const errorGroups = this.groupErrorsByType();

      report += `### 错误类型统计\n\n`;
      Object.entries(errorGroups).forEach(([errorType, count]) => {
        report += `- **${errorType}**: ${count} 次\n`;
      });

      report += `\n### 失败测试列表\n\n`;
      report += `| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |\n`;
      report += `|------|-----------|--------|--------|------|--------|----------|--------|----------|\n`;

      this.failedTests.forEach((test) => {
        const host =
          test.host.length > 20
            ? test.host.substring(0, 17) + "..."
            : test.host;
        const errorMsg =
          test.error_msg.length > 50
            ? test.error_msg.substring(0, 47) + "..."
            : test.error_msg;
        const serverHeader =
          test.server_header.length > 15
            ? test.server_header.substring(0, 12) + "..."
            : test.server_header;

        report += `| ${test.index} | ${host} | ${test.target_ip} | ${test.ip_version} | ${test.protocol} | ${
          test.status_code || "N/A"
        } | ${test.latency_ms} | ${serverHeader} | ${errorMsg} |\n`;
      });
    }

    report += `

---

## 详细分析

### 按IP版本统计
`;

    // 按IP版本统计
    const ipv4Failed = this.failedTests.filter(
      (t) => t.ip_version === "IPv4",
    ).length;
    const ipv6Failed = this.failedTests.filter(
      (t) => t.ip_version === "IPv6",
    ).length;

    report += `- **IPv4 失败**: ${ipv4Failed} 次\n`;
    report += `- **IPv6 失败**: ${ipv6Failed} 次\n\n`;

    // 按协议统计
    const protocolStats = {};
    this.failedTests.forEach((test) => {
      protocolStats[test.protocol] = (protocolStats[test.protocol] || 0) + 1;
    });

    report += `### 按协议统计\n\n`;
    Object.entries(protocolStats).forEach(([protocol, count]) => {
      report += `- **${protocol}**: ${count} 次失败\n`;
    });

    report += `

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
`;

    return report;
  }

  /**
   * 按错误类型分组
   */
  groupErrorsByType() {
    const errorGroups = {};

    this.failedTests.forEach((test) => {
      let errorType = "未知错误";

      if (test.error_msg) {
        if (
          test.error_msg.includes("timeout") ||
          test.error_msg.includes("超时")
        ) {
          errorType = "连接超时";
        } else if (
          test.error_msg.includes("connection") ||
          test.error_msg.includes("连接")
        ) {
          errorType = "连接错误";
        } else if (
          test.error_msg.includes("DNS") ||
          test.error_msg.includes("解析")
        ) {
          errorType = "DNS解析错误";
        } else if (
          test.error_msg.includes("TLS") ||
          test.error_msg.includes("SSL") ||
          test.error_msg.includes("证书")
        ) {
          errorType = "TLS/SSL错误";
        } else if (test.protocol === "none") {
          errorType = "协议协商失败";
        }
      } else {
        errorType = "无错误信息";
      }

      errorGroups[errorType] = (errorGroups[errorType] || 0) + 1;
    });

    return errorGroups;
  }

  /**
   * 生成JSON格式的报告
   */
  generateJsonReport() {
    return {
      report_info: {
        generated_at: new Date().toISOString(),
        source_file: basename(this.resultsFilePath),
        total_tests: this.statistics.total,
        failed_tests: this.statistics.failed,
        success_tests: this.statistics.success,
        failure_rate: parseFloat(this.statistics.failureRate),
      },
      statistics: {
        by_ip_version: {
          ipv4: this.failedTests.filter((t) => t.ip_version === "IPv4").length,
          ipv6: this.failedTests.filter((t) => t.ip_version === "IPv6").length,
        },
        by_protocol: this.getProtocolStatistics(),
        by_error_type: this.groupErrorsByType(),
      },
      failed_tests: this.failedTests,
    };
  }

  /**
   * 获取协议统计信息
   */
  getProtocolStatistics() {
    const protocolStats = {};
    this.failedTests.forEach((test) => {
      protocolStats[test.protocol] = (protocolStats[test.protocol] || 0) + 1;
    });
    return protocolStats;
  }

  /**
   * 保存报告到文件
   */
  saveReport(format = "markdown") {
    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");

    if (format === "markdown" || format === "both") {
      const markdownReport = this.generateMarkdownReport();
      const markdownFile = `failed-test-report-${timestamp}.md`;
      writeFileSync(markdownFile, markdownReport, "utf8");
      console.log(`Markdown报告已保存到: ${markdownFile}`);
    }

    if (format === "json" || format === "both") {
      const jsonReport = this.generateJsonReport();
      const jsonFile = `failed-test-report-${timestamp}.json`;
      writeFileSync(jsonFile, JSON.stringify(jsonReport, null, 2), "utf8");
      console.log(`JSON报告已保存到: ${jsonFile}`);
    }
  }

  /**
   * 在控制台显示简要报告
   */
  displaySummary() {
    console.log("\n" + "=".repeat(50));
    console.log("HTTP/3 连接测试失败报告摘要");
    console.log("=".repeat(50));
    console.log(`总测试数: ${this.statistics.total}`);
    console.log(
      `失败测试数: ${this.statistics.failed} (${this.statistics.failureRate}%)`,
    );
    console.log(`成功测试数: ${this.statistics.success}`);

    if (this.failedTests.length > 0) {
      console.log("\n主要失败原因:");
      const errorGroups = this.groupErrorsByType();
      Object.entries(errorGroups)
        .sort(([, a], [, b]) => b - a)
        .slice(0, 5)
        .forEach(([errorType, count]) => {
          console.log(`  - ${errorType}: ${count} 次`);
        });
    }

    console.log("=".repeat(50));
  }
}
import { fileURLToPath } from 'node:url';
import { dirname } from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
// 主执行函数
function main() {
  const resultsFilePath = join(__dirname, "connectivity_results.json");

  console.log("HTTP/3 连接测试失败报告生成器");
  console.log("=".repeat(40));

  // 检查文件是否存在
  if (!existsSync(resultsFilePath)) {
    console.error(`错误: 找不到测试结果文件 ${resultsFilePath}`);
    console.log("请确保 connectivity_results.json 文件存在于当前目录中");
    process.exit(1);
  }

  // 创建报告生成器
  const generator = new TestReportGenerator(resultsFilePath);

  // 加载和分析测试结果
  const results = generator.loadResults();
  generator.analyzeResults(results);

  // 显示简要报告
  generator.displaySummary();

  // 保存报告（默认生成Markdown和JSON两种格式）
  generator.saveReport("both");

  console.log("\n报告生成完成！");
}

// 如果直接运行此脚本
if (import.meta.main) {
  main();
}

export default TestReportGenerator;
