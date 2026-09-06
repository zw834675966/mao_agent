---
title: "引用文献: Fallacies of distributed computing (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Fallacies of distributed computing (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing](https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

The fallacies of distributed computing are a set of assertions made by L. Peter Deutsch and others at Sun Microsystems describing false assumptions that programmers new to distributed applications invariably make. 


== The fallacies ==
The originally listed fallacies are

The network is reliable;
Latency is zero;
Bandwidth is infinite;
The network is secure;
Topology doesn't change;
There is one administrator;
Transport cost is zero;
The network is homogeneous;


== The effects of the fallacies ==
Software applications are written with little error-handling on networking errors. During a network outage, such applications may stall or infinitely wait for an answer packet, permanently consuming memory or other resources. When the failed network becomes available, those applications may also fail to retry any stalled operations or require a (manual) restart.
Ignorance of network latency, and of the packet loss it can cause, induces application- and transport-layer developers to allow unbounded traffic, greatly increasing dropped packets and wasting bandwidth.
Ignorance of bandwidth limits on the part of traffic senders can result in bottlenecks.
Complacency regarding network security results in being blindsided by malicious users and programs that continually adapt to security measures.
Changes in network topology can have effects on both bandwidth and latency issues, and therefore can have similar problems.
Multiple administrators, as with subnets for rival companies, may institute conflicting policies of which senders of network traffic must be aware in order to complete their desired paths.
The "hidden" costs of building and maintaining a network or subnet are non-negligible and must consequently be noted in budgets to avoid vast shortfalls.
If a system assumes a homogeneous network, then it can lead to the same problems that result from the first three fallacies.


== History ==
The list of fallacies originated at Sun Microsystems. L. Peter Deutsch, one of the original Sun "Fellows", first created a list of seven fallacies in 1994; incorporating four fallacies Bill Joy and Dave Lyon had already identified in "The Fallacies of Networked Computing". Around 1997, James Gosling, another Sun Fellow and the inventor of Java, added the eighth fallacy.
In an episode of "Software Engineering Radio"  Peter Deutsch added a ninth fallacy: "It's really an expansion of number 4. It extends beyond the boundaries of the physical network. ... The party you are communicating with is trustworthy."
Later in 2020, Mark Richards and Neal Ford expanded upon the original "Fallacies of Distributed Computing" by introducing three additional fallacies to address contemporary challenges in distributed systems: 

Versioning is simple
Compensating updates always work
Observability is optional


== See also ==
CAP theorem
PACELC design principle
Distributed computing
Fine vs coarse grained SOA


== References ==


== External links ==
Deutsch, Peter digital ability. "The Eight Fallacies of Distributed Computing". nighthacks.com. Retrieved 2024-07-24.
Yoavi, Neli (January 2008). "Fallacies of Distributed Computing Explained". Retrieved 2024-07-24 – via ResearchGate.

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing_
