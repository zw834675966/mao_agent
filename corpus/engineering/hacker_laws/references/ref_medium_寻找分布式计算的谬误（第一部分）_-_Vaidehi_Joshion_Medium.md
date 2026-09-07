---
title: "引用文献: Foraging for the Fallacies of Distributed Computing (寻找分布式计算的谬误) - BaseCS"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://medium.com/baseds/foraging-for-the-fallacies-of-distributed-computing-part-1-1b35c3b85b53"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
---

〔本文档为黑客定律与工程哲学文库中《寻找分布式计算的谬误（第一部分） - Vaidehi Joshion Medium》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Foraging for the Fallacies of Distributed Computing (寻找分布式计算的谬误) - BaseCS
- **原文链接**: [https://medium.com/baseds/foraging-for-the-fallacies-of-distributed-computing-part-1-1b35c3b85b53](https://medium.com/baseds/foraging-for-the-fallacies-of-distributed-computing-part-1-1b35c3b85b53)
- **引用锚文本**: 寻找分布式计算的谬误（第一部分） - Vaidehi Joshion Medium
- **抓取状态**: success

# 二、 文献正文内容

# Foraging for the Fallacies of Distributed Computing (寻找分布式计算的谬误)

作者：Vaidehi Joshi (发表于 BaseCS 计算机科学专栏)

### 背景与起源

1994年，Sun Microsystems 的副总裁兼计算专家 L. Peter Deutsch 与 James Gosling 等人正式系统化提出了“分布式计算的八大谬论”（The Fallacies of Distributed Computing）。这是每一位从单机应用迈向分布式系统架构的工程师必须掌握的底线常识。

### 八大谬论逐条解析

1. **谬论一：网络是可靠的 (The network is reliable)**
   - 现实：光缆会被挖断，交换机会丢包，网络分区（Network Partition）是常态而非意外。必须设计重试机制、幂等接口与降级容错。
2. **谬论二：延迟为零 (Latency is zero)**
   - 现实：光速有限，进程间调用远慢于单机内存访问。在网络调用中过度频繁发起小请求（Chatty APIs）会彻底拖垮系统吞吐。
3. **谬论三：带宽是无限的 (Bandwidth is infinite)**
   - 现实：虽然现代带宽不断增加，但在高并发微服务与大数据传输下，带宽极易成为严重颈瓶，需采用紧凑序列化（Protobuf/Avro）与压缩。
4. **谬论四：网络是安全的 (The network is secure)**
   - 现实：内网并不等于安全。微服务内部通信同样需要TLS加密、mTLS身份互信与零信任（Zero Trust）架构。
5. **谬论五：拓扑结构永不改变 (Topology doesn't change)**
   - 现实：云原生与容器环境下，Pod与节点频繁启停、IP动态变化。必须依赖服务发现（Consul/DNS/K8s Service）。
6. **谬论六：只有一名系统管理员 (There is one administrator)**
   - 现实：跨多集群、跨云厂商乃至跨组织协同，不同团队有不同的防火墙规则与安全策略。
7. **谬论七：传输成本为零 (Transport cost is zero)**
   - 现实：序列化与反序列化（JSON/XML解析）消耗大量CPU算力，跨区域传输流量产生高额云账单。
8. **谬论八：网络是同构的 (The network is homogeneous)**
   - 现实：异构设备、多种操作系统协议栈并存，必须依托通用标准契约（如 HTTP/gRPC/OpenAPI）。

---
_本地归档时间: 2026-09-05 | 来源: https://medium.com/baseds/foraging-for-the-fallacies-of-distributed-computing-part-1-1b35c3b85b53_
