package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

type HostEntry struct {
	Host string `json:"host"`
}

func main() {
	// 读取输入文件
	inputFile := "上游节点-workers.txt"
	outputFile := "hosts.json"

	// 检查输入文件是否存在
	if _, err := os.Stat(inputFile); os.IsNotExist(err) {
		fmt.Printf("错误: 文件 %s 不存在\n", inputFile)
		os.Exit(1)
	}

	// 打开输入文件
	file, err := os.Open(inputFile)
	if err != nil {
		fmt.Printf("错误: 无法打开文件 %s: %v\n", inputFile, err)
		os.Exit(1)
	}
	defer file.Close()

	// 读取所有行并提取host
	var hosts []string
	scanner := bufio.NewScanner(file)
	hostRegex := regexp.MustCompile(`@([^:]+):`)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		// 提取host
		matches := hostRegex.FindStringSubmatch(line)
		if len(matches) > 1 {
			host := matches[1]
			hosts = append(hosts, host)
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("错误: 读取文件时出错: %v\n", err)
		os.Exit(1)
	}

	// 创建输出JSON
	outputData := make([]HostEntry, len(hosts))
	for i, host := range hosts {
		outputData[i] = HostEntry{Host: host}
	}

	// 写入JSON文件
	jsonData, err := json.MarshalIndent(outputData, "", "  ")
	if err != nil {
		fmt.Printf("错误: JSON序列化失败: %v\n", err)
		os.Exit(1)
	}

	// 获取绝对路径
	absPath, err := filepath.Abs(outputFile)
	if err != nil {
		fmt.Printf("警告: 无法获取绝对路径: %v\n", err)
		absPath = outputFile
	}

	// 写入文件
	err = os.WriteFile(outputFile, jsonData, 0644)
	if err != nil {
		fmt.Printf("错误: 无法写入文件 %s: %v\n", outputFile, err)
		os.Exit(1)
	}

	fmt.Printf("成功提取 %d 个host到 %s\n", len(hosts), absPath)
	fmt.Printf("文件路径: %s\n", absPath)

	// 显示前5个host作为预览
	if len(hosts) > 0 {
		fmt.Println("\n前5个host预览:")
		for i := 0; i < min(5, len(hosts)); i++ {
			fmt.Printf("  %d. %s\n", i+1, hosts[i])
		}
	}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
