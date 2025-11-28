package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"os"
	"strings"
	"sync"
	"time"

	dns_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/dns"
	h3_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/h3"
	"github.com/miekg/dns"
)

type HostEntry struct {
	Host string `json:"host"`
}

type TestResult struct {
	Host        string  `json:"host"`
	TargetIP    string  `json:"target_ip"`
	IPVersion   string  `json:"ip_version"`
	Success     bool    `json:"success"`
	StatusCode  *uint16 `json:"status_code"`
	Protocol    string  `json:"protocol"`
	LatencyMs   *uint64 `json:"latency_ms"`
	ServerHeader *string `json:"server_header"`
	ErrorMessage *string `json:"error_msg"`
}

// 全局配置
var (
	verbose = flag.Bool("verbose", false, "详细输出")
	concurrency = flag.Int("concurrency", 10, "并发测试数量")
	timeout = flag.Int("timeout", 10, "超时时间(秒)")
)

var (
	dohURL = "https://xget.a1u06h9fe9y5bozbmgz3.qzz.io/cloudflare-dns.com/dns-query"
	defaultPort = 443
)

func main() {
	flag.Parse()

	// 读取hosts.json
	hosts, err := loadHosts("hosts.json")
	if err != nil {
		log.Fatalf("加载hosts.json失败: %v", err)
	}

	fmt.Printf("成功加载 %d 个host，开始测试连通性...\n", len(hosts))

	// 测试所有host
	results := testHostsConnectivity(hosts)

	// 输出结果到JSON文件
	if err := saveResults(results, "connectivity_results.json"); err != nil {
		log.Fatalf("保存结果失败: %v", err)
	}

	fmt.Printf("\n测试完成！结果已保存到 connectivity_results.json\n")
	fmt.Printf("成功: %d, 失败: %d\n",
		len(filterResults(results, true)),
		len(filterResults(results, false)))
}

// 加载hosts.json文件
func loadHosts(filename string) ([]string, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	var hosts []HostEntry
	if err := json.Unmarshal(data, &hosts); err != nil {
		return nil, err
	}

	var hostList []string
	for _, h := range hosts {
		hostList = append(hostList, h.Host)
	}

	return hostList, nil
}

// 测试所有host的连通性
func testHostsConnectivity(hosts []string) []TestResult {
	var wg sync.WaitGroup
	results := make([]TestResult, len(hosts))
	sem := make(chan struct{}, *concurrency)

	for i, host := range hosts {
		wg.Add(1)
		go func(index int, host string) {
			sem <- struct{}{}
			defer func() {
				<-sem
				wg.Done()
			}()

			if *verbose {
				fmt.Printf("测试 %s...\n", host)
			}

			results[index] = testSingleHost(host)
		}(i, host)
	}

	wg.Wait()
	return results
}

// 测试单个host的连通性
func testSingleHost(host string) TestResult {
	// 判断是否为IP地址
	isIP := isIPAddress(host)

	var targetIP string
	var err error

	if isIP {
		// 如果是IP，直接使用
		targetIP = host
		if *verbose {
			fmt.Printf("  %s 是IP地址，直接使用\n", host)
		}
	} else {
		// 如果是域名，进行DNS解析
		if *verbose {
			fmt.Printf("  %s 是域名，进行DoH解析...\n", host)
		}
		targetIPs, err := dohLookup(host, dohURL)
		if err != nil {
			return TestResult{
				Host:        host,
				Success:     false,
				ErrorMessage: stringPtr(fmt.Sprintf("DNS解析失败: %v", err)),
			}
		}
		if len(targetIPs) == 0 {
			return TestResult{
				Host:        host,
				Success:     false,
				ErrorMessage: stringPtr("DNS解析无结果"),
			}
		}
		targetIP = targetIPs[0]
		if *verbose {
			fmt.Printf("  解析到IP: %s\n", targetIP)
		}
	}

	// 构建测试URL
	testURL := fmt.Sprintf("https://%s:%d/", host, defaultPort)

	// 测试HTTP/3连接
	success, protocol, statusCode, serverHeader, latencyMs, err := testHTTP3Connection(
		testURL, host, targetIP, defaultPort, *timeout)

	if err != nil {
		if *verbose {
			fmt.Printf("  HTTP/3失败: %v，尝试HTTP/2...\n", err)
		}
		// 回退到HTTP/2
		success, protocol, statusCode, serverHeader, latencyMs, err = testHTTP2Connection(
			testURL, host, targetIP, defaultPort, *timeout)
	}

	result := TestResult{
		Host:        host,
		TargetIP:    targetIP,
		IPVersion:   "IPv4",
		Success:     success,
		Protocol:    protocol,
		LatencyMs:   uint64Ptr(latencyMs),
	}

	if strings.Contains(targetIP, ":") {
		result.IPVersion = "IPv6"
	}

	if err != nil {
		result.ErrorMessage = stringPtr(err.Error())
	} else {
		result.StatusCode = uint16Ptr(statusCode)
		result.ServerHeader = stringPtr(serverHeader)
	}

	return result
}

// 判断是否为IP地址
func isIPAddress(host string) bool {
	ip := net.ParseIP(host)
	return ip != nil
}

// DoH查询
func dohLookup(domain, dohURL string) ([]string, error) {
	msg := new(dns.Msg)
	msg.SetQuestion(dns.Fqdn(domain), dns.TypeA)

	// 首先尝试A记录
	ips, err := performDoHQuery(msg, dohURL)
	if err != nil {
		fmt.Printf("  DoH查询A记录失败: %v\n", err)
	} else if len(ips) > 0 {
		return ips, nil
	}

	// 如果A记录没有结果，尝试AAAA记录
	msg.SetQuestion(dns.Fqdn(domain), dns.TypeAAAA)
	ips, err = performDoHQuery(msg, dohURL)
	if err != nil {
		fmt.Printf("  DoH查询AAAA记录失败: %v\n", err)
		return nil, err
	}

	return ips, nil
}

// 执行DoH查询
func performDoHQuery(msg *dns.Msg, dohURL string, dohIPs ...string) ([]string, error) {
	resp, err := dns_experiment.DohClient(msg, dohURL, dohIPs...)
	if err != nil {
		return nil, err
	}

	var ips []string
	for _, ans := range resp.Answer {
		switch a := ans.(type) {
		case *dns.A:
			ips = append(ips, a.A.String())
		case *dns.AAAA:
			ips = append(ips, a.AAAA.String())
		}
	}

	return ips, nil
}

// 测试HTTP/3连接
func testHTTP3Connection(testURL, hostHeader, targetIP string, port int, timeoutSec int) (
	bool, string, uint16, string, uint64, error) {

	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(timeoutSec)*time.Second)
	defer cancel()

	transport := h3_experiment.CreateHTTP3TransportWithIP(targetIP)

	client := &http.Client{
		Transport: transport,
		Timeout:   time.Duration(timeoutSec) * time.Second,
	}

	req, err := http.NewRequestWithContext(ctx, "GET", testURL, nil)
	if err != nil {
		return false, "none", 0, "", 0, err
	}

	req.Header.Set("Host", hostHeader)
	req.Header.Set("User-Agent", "curl/8.12.1")

	resp, err := client.Do(req)
	if err != nil {
		return false, "h3", 0, "", 0, err
	}
	defer resp.Body.Close()

	latencyMs := uint64(time.Since(time.Now()).Milliseconds())
	statusCode := uint16(resp.StatusCode)
	serverHeader := resp.Header.Get("Server")

	return resp.StatusCode < 500, "h3", statusCode, serverHeader, latencyMs, nil
}

// 测试HTTP/2连接
func testHTTP2Connection(testURL, hostHeader, targetIP string, port int, timeoutSec int) (
	bool, string, uint16, string, uint64, error) {

	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(timeoutSec)*time.Second)
	defer cancel()

	targetAddr := fmt.Sprintf("[%s]:%d", targetIP, port)
	if !strings.Contains(targetIP, ":") {
		targetAddr = fmt.Sprintf("%s:%d", targetIP, port)
	}

	dialer := &net.Dialer{
		Timeout: time.Duration(timeoutSec) * time.Second,
	}

	conn, err := dialer.Dial("tcp", targetAddr)
	if err != nil {
		return false, "none", 0, "", 0, err
	}
	defer conn.Close()

	// 构建HTTP/2请求
	req, err := http.NewRequestWithContext(ctx, "GET", testURL, nil)
	if err != nil {
		return false, "none", 0, "", 0, err
	}

	req.Header.Set("Host", hostHeader)
	req.Header.Set("User-Agent", "curl/8.12.1")

	start := time.Now()
	client := &http.Client{
		Transport: &http.Transport{
			Dial: func(network, addr string) (net.Conn, error) {
				return conn, nil
			},
		},
		Timeout: time.Duration(timeoutSec) * time.Second,
	}

	resp, err := client.Do(req)
	if err != nil {
		return false, "h2", 0, "", 0, err
	}
	defer resp.Body.Close()

	latencyMs := uint64(time.Since(start).Milliseconds())
	statusCode := uint16(resp.StatusCode)
	serverHeader := resp.Header.Get("Server")

	return resp.StatusCode < 500, "h2", statusCode, serverHeader, latencyMs, nil
}

// 保存结果到JSON文件
func saveResults(results []TestResult, filename string) error {
	jsonData, err := json.MarshalIndent(results, "", "  ")
	if err != nil {
		return err
	}

	return os.WriteFile(filename, jsonData, 0644)
}

// 辅助函数
func stringPtr(s string) *string {
	return &s
}

func uint16Ptr(n uint16) *uint16 {
	return &n
}

func uint64Ptr(n uint64) *uint64 {
	return &n
}

func filterResults(results []TestResult, success bool) []TestResult {
	var filtered []TestResult
	for _, r := range results {
		if r.Success == success {
			filtered = append(filtered, r)
		}
	}
	return filtered
}
