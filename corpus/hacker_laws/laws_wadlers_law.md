---
title: "沃德勒定律 (Wadler's Law)"
author: "菲利普·沃德勒 (Philip Wadler / Haskell共同设计者)"
date: "1990"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "编程语言设计与争议专注度定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "沃德勒定律"
  - "语法糖争议"
  - "编程语言设计"
  - "类型系统"
  - "讨论内耗"
---

〔沃德勒定律观察到：在编程语言设计中，对某项特性讨论所耗费的精力和争议程度，与其语义的重要性成反比。团队会就变量命名或符号语法争辩数月，而几分钟草率通过核心类型系统。〕

# 一、 定律与原则核心阐述

- [英文在线地址](https://wiki.haskell.org/Wadler's_Law)

> 任何语言设计中，讨论下面列表中某个要素所花费的总时间与其位置成正比。
>
> 0. 语义 (Semantics)
> 1. 语法 (Syntax)
> 1. 词法 (Lexical syntax)
> 1. 注释语法 (Lexical syntax of comments)
>
> （简而言之，在语义上花费一个小时，就要在注释语法上花费八个小时）。

与 [帕金森琐碎定理](#%E5%B8%95%E9%87%91%E6%A3%AE%E7%90%90%E7%A2%8E%E5%AE%9A%E7%90%86-the-law-of-triviality) 类似, 沃德勒定律指出，在设计语言时，与这些特征的重要性相比，花在语言结构上的时间过多。

参见：

- [帕金森琐碎定理](#%E5%B8%95%E9%87%91%E6%A3%AE%E7%90%90%E7%A2%8E%E5%AE%9A%E7%90%86-the-law-of-triviality)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 编程语言设计与争议专注度定律
