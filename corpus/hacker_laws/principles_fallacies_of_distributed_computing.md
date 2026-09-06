---
title: "分布式计算的八大谬论 (The Fallacies of Distributed Computing)"
author: "彼得·德伊奇 等 (L. Peter Deutsch et al. / Sun Microsystems)"
date: "1994"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "分布式系统设计底线警示"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "分布式计算谬论"
  - "网络不可靠"
  - "延迟非零"
  - "带宽有限"
  - "分布式系统容错"
---

〔分布式计算的八大谬论揭示了初学者在分布式系统中普遍持有的8个致命错误假定：网络可靠、延迟为零、带宽无限、网络安全、拓扑恒定、只有一名管理员、传输开销为零、网络同质。〕

# 一、 定律与原则核心阐述

[英文维基百科](https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing)

又称 _网络计算的谬误_，这是一系列关于分布式计算的猜想（或者看法），这些猜想可能会引起软件开发中的失败。这些假设是：

- 网络可靠
- 延迟为零
- 带宽无限
- 网络安全
- 拓扑恒定
- 单一管理员
- 运输成本为零
- 网络为同构的

前 4 个项目由 [Bill Joy](https://en.wikipedia.org/wiki/Bill_Joy) 和 [Tom Lyon](https://twitter.com/aka_pugs) 于 1991 左右提出。并被 [James Gosling](https://en.wikipedia.org/wiki/James_Gosling) 首次归类于“网络计算的谬误”；后 [L. Peter Deutsch](https://en.wikipedia.org/wiki/L._Peter_Deutsch) 添加了第 5、6、7 个谬误；90 年代末，Gosling 添加了最后一个谬误。

这些内容受到了 [太阳微系统 (Sun Microsystems)](https://en.wikipedia.org/wiki/Sun_Microsystems) 内部当时所发生的事情的启发。

在设计弹性代码的时候，应该仔细考虑这些谬误，并假设其中任何一个谬误都可能引起处理分布式系统的复杂性和现实性时的逻辑缺陷。

参见:

- [寻找分布式计算的谬误（第一部分） - Vaidehi Joshion Medium](https://medium.com/baseds/foraging-for-the-fallacies-of-distributed-computing-part-1-1b35c3b85b53)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 分布式系统设计底线警示
