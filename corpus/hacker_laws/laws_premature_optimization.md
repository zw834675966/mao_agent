---
title: "过早优化效应 (Premature Optimization Effect)"
author: "高德纳 (Donald Knuth / 《计算机程序设计艺术》)"
date: "1974"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "性能调优与工程优先级定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "过早优化"
  - "高德纳"
  - "性能基准"
  - "代码可读性"
  - "系统瓶颈"
---

〔高德纳名言指出：过早优化是万恶之源（Premature optimization is the root of all evil）。应当首先编写清晰正确的代码，再基于实际性能基准数据进行有针对性的瓶颈优化。〕

# 一、 定律与原则核心阐述

- [英文在线网站](http://wiki.c2.com/?PrematureOptimization)

> 过早优化是万恶之源。
>
> [高德纳 (唐纳德克努特的中文名)](https://twitter.com/realdonaldknuth?lang=en)

在高德纳的[《goto 语句的结构化编程》](http://wiki.c2.com/?StructuredProgrammingWithGoToStatements)论文中，他写到：“程序员们浪费了大量的时间去思考或者担心他们的程序中的非关键部分的速度。而在考虑调试和维护的时候，这些所谓提高效率的做法实际上十分不妥。我们应该放弃小的效率点，并且要在 97% 的时间提醒自己，**过早优化是万恶之源**。而且连那关键的 3% 也不能够放过。”

然而，_过早优化_ （简而言之）可以定义为在我们知道需要做什么之前进行优化。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 性能调优与工程优先级定律
