---
title: "古德哈特定律 (Goodhart's Law)"
author: "查尔斯·古德哈特 (Charles Goodhart)"
date: "1975"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "度量指标与绩效反噬定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "古德哈特定律"
  - "研发度量"
  - "KPI异化"
  - "代码行数度量"
  - "测试覆盖率"
---

〔古德哈特定律指出：当一个指标变成目标时，它就不再是一个好的指标。人们会操纵指标本身而背离最初的业务初衷。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Goodhart's_law)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%8F%A4%E5%BE%B7%E5%93%88%E7%89%B9%E5%AE%9A%E5%BE%8B)

> 当压力施于其上以进行控制时，任何观测到的统计恒性都倾向消散。
>
> _查尔斯·古德哈特 (Charles Goodhart)_

另见：

> 当一个措施本身成为目标时，它就不再是一个好的措施。
>
> _玛丽莲·斯特拉腾 (Marilyn Strathern)_

根据这一定律，由测量驱动的优化反而可能导致测量结果本身的说服力下降。盲目使用一些过度严格筛选的方法 ([KPIs](https://zh.wikipedia.org/wiki/%E9%97%9C%E9%8D%B5%E7%B8%BE%E6%95%88%E6%8C%87%E6%A8%99)) 可能会产生一些不良的影响。人们会倾向于用“钻空子”的行为去做局部优化，从而满足一些特定的度量标准，而不会在意整体的结果。

现实中的例子：

- Assert-free 测试可以达到代码覆盖率的预期，但度量的目的应该是创造经过良好测试的软件。
- 由 commits 的行数来评价开发人员的表现，从而导致了不合理的代码库扩增。

参见

- [古德哈特定律：错误的测量如何导致不道德的行为](https://coffeeandjunk.com/goodharts-campbells-law/)
- [呆伯特与无 bug 软件](https://dilbert.com/strip/1995-11-13)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 度量指标与绩效反噬定律
