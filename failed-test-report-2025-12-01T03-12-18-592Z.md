# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/1 11:12:18
- **数据来源**: connectivity_results.json
- **总测试数**: 25431
- **失败测试数**: 270
- **成功测试数**: 25161
- **失败率**: 1.06%

---

## 失败测试详情

### 错误类型统计

- **连接超时**: 268 次
- **未知错误**: 2 次

### 失败测试列表

| 序号  | 主机/域名            | 目标IP                    | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                           |
| ----- | -------------------- | ------------------------- | ------ | ---- | ------ | -------- | ------ | -------------------------------------------------- |
| 344   | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 345   | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 380   | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 394   | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 447   | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 458   | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 527   | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 803   | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1016  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 1060  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1077  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 1113  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 1177  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1192  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1211  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 1231  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 1273  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 1368  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 1506  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1623  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 1681  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 1740  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 1896  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1933  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 1942  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 1961  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 1968  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2040  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 2065  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2132  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 2201  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2280  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2319  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 2369  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2471  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2528  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2535  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2598  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 2630  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 2632  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2748  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2784  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 2807  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 3080  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 3099  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 3110  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 3173  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 3240  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 3308  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 3515  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 3627  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 3797  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 3928  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 4063  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 4431  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 4505  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 4512  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 4871  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 5122  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 5571  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 5583  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 5608  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 5743  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 5877  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 6125  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 6232  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 6256  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 6324  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 6583  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 6809  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 6810  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 6896  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 6921  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 7213  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 7214  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 7271  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 7292  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 7476  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 7526  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 7649  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 7689  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 7806  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 7896  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 7942  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 8021  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 8054  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 8095  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 8280  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 8302  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 8306  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 8493  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 8542  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 8566  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 8661  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 9007  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 9205  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 9329  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 9404  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 9423  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 9511  | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 9631  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 9775  | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 9905  | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 9976  | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 10016 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 10076 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 10135 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 10416 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 10427 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 10474 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 10518 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 10591 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 10771 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 10849 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 10961 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 11000 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 11089 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 11199 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 11308 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 11377 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 11462 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 11591 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 11716 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 11747 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 11860 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 11937 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 11957 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 12067 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 12143 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 12152 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 12251 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 12269 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 12298 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 12507 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 12698 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 12972 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 13179 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 13201 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 13496 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 13510 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 13605 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 13722 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 13846 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 13981 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 14018 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 14189 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 14241 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 14573 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 14579 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 14781 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 14874 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 14924 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 15037 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 15111 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 15119 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 15261 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 15286 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 15431 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 15455 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 15788 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 15868 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 15956 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 16011 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16077 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 16120 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16282 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 16335 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 16384 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 16538 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16540 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 16599 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 16701 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16760 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 16771 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16846 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16854 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 16933 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 16983 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 17089 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 17091 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 17224 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 17510 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 17632 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 17638 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 17678 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 17792 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 17850 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 17956 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 17993 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 18033 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 18034 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 18123 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 18208 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 18486 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 18680 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 18839 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 18855 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 18922 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 19014 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 19233 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 19434 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 19447 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 19756 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 19838 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 19886 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 19981 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 20080 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 20234 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 20275 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 20629 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 20741 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 20760 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 20895 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 20947 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 20970 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 20971 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 20997 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 21038 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 21113 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 21216 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 21283 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 21309 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 21375 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 21527 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 21557 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 21918 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 22225 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 22342 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 22819 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 22826 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 22827 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 22840 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 22845 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 23079 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 23162 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 23211 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 23328 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 23331 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 23332 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 23372 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 23444 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 23845 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 23881 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 23895 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 23904 | 72806a5a-a251-48b... | 2606:4700:3034::6815:3db6 | IPv6   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.... |
| 24164 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 24200 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 24255 | silkbook.com         | 172.67.75.208             | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.... |
| 24306 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 24542 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 24615 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 24668 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 24686 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 24702 | 172.67.49.134        | 172.67.49.134             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 24745 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 24806 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 24839 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25089 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25156 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25169 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 25170 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25275 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25317 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25372 | 172.64.201.25        | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 25402 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 25403 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 25404 | www.sean-now.com     | 172.80.107.154            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.80.107.154:443: i/o timeout           |
| 25405 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 25422 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 25428 | trevor.ns.cloudfl... | 108.162.195.154           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 269 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 268 次失败
- **h2**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
