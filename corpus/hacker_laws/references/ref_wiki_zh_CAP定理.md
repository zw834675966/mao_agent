---
title: "引用文献: CAP定理 (ZH Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://zh.wikipedia.org/wiki/CAP%E5%AE%9A%E7%90%86"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《中文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: CAP定理 (ZH Wikipedia)
- **原文链接**: [https://zh.wikipedia.org/wiki/CAP%E5%AE%9A%E7%90%86](https://zh.wikipedia.org/wiki/CAP%E5%AE%9A%E7%90%86)
- **引用锚文本**: 中文维基百科
- **抓取状态**: success

# 二、 文献正文内容

在理論計算機科學中，CAP定理（CAP theorem），又被稱作布魯爾定理（Brewer's theorem），它指出對於一個分布式计算系統來說，不可能同時滿足以下三點：

一致性（Consistency）
所有节点访问同一份最新的数据副本
可用性（Availability）
每次请求都能获取到非错的响应——但是不保证获取的数据为最新数据
分区容错性（Partition tolerance）
以实际效果而言，分区相当于对通信的时限要求。系统如果不能在时限内达成数据一致性，就意味着发生了分区的情况，必须就当前操作在C和A之间做出选择。
理解CAP理论的最简单方式是想象两个节点分处分区两侧。允许至少一个节点更新状态会导致数据不一致，即丧失了C性质。如果为了保证数据一致性，将分区一侧的节点设置为不可用，那么又丧失了A性质。除非两个节点可以互相通信，才能既保证C又保证A，这又会导致丧失P性质。


== 歷史 ==
這個定理起源於加州大學柏克萊分校（University of California, Berkeley）的計算機科學家埃里克·布鲁尔在2000年的分佈式計算原理研討會（PODC）上提出的一個猜想。在2002年，麻省理工学院（MIT）的赛斯·吉尔伯特和南希·林奇發表了布魯爾猜想的證明，使之成爲一個定理。
吉尔伯特和林奇证明的CAP定理比布鲁尔设想的某种程度上更加狭义。定理讨论了在两个互相矛盾的请求到达彼此连接不通的两个不同的分布式节点的时候的处理方案。


== PACELC理論 ==

2010年，耶鲁大学教授丹尼爾·阿巴迪提出PACELC理論，擴展了CAP定理。原有的CAP定理重寫為PAC，ELC則代表「Else, choose between (low) latency and consistency」。在網絡正常（沒有分區）的情況下，須選擇低延遲或一致性。
根據PACELC理論，分散式系統可分為四種不同的運作模式

PA/EL: 相比數據一致性，優先選擇可用性和低延遲
PA/EC: 出現分區時選擇可用性，無分區時選擇一致性
PC/EL: 出現分區時選擇一致性，無分區時選擇低延遲
PC/EC: 任何時候，以保持數據一致性為優先
設計師可以自行設定数据库管理系统的運作模式。例如，一個系統包含兩份數據，可讀寫的主數據庫和只讀的副數據庫。如果副數據庫與主數據庫同步，則系統屬於EC。如果副數據庫並非同步，但允許讀取，則系統屬於EL。如果副數據庫不適合設定為同步備份，設計師可停用讀取功能，讓用戶只讀取主數據庫，此時系統為EC。


== 参考文献 ==


== 外部連結 ==
"Problems with CAP, and Yahoo's little known NoSQL system"（页面存档备份，存于互联网档案馆） by Daniel Abadi（页面存档备份，存于互联网档案馆）
"CAP equivalent for analytics"（页面存档备份，存于互联网档案馆）
"Consistency Models in Non-Relational Databases" by Guy Harrison :  A good explanation of CAP Theorem, 最终一致性 and how consistency problems can be handled in distributed environments.
"A Simple introduction to CAP theorem"


== 參見 ==
分佈式計算的謬論

---
_本地归档时间: 2026-09-05 | 来源: https://zh.wikipedia.org/wiki/CAP%E5%AE%9A%E7%90%86_
