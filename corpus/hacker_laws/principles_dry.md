---
title: "DRY 原则 / 不要重复自己 (The DRY Principle)"
author: "安迪·亨特 与 戴夫·托马斯 (Andy Hunt & Dave Thomas / 《程序员修炼之道》)"
date: "1999"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "核心工程实践与代码重用原则"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "DRY原则"
  - "不要自我重复"
  - "正交性"
  - "单一事实来源"
  - "可维护性"
---

〔DRY原则倡导：系统中的每一项知识或逻辑都必须在系统中具有单一、明确、权威且不可歧义的表述形式，杜绝多处复制粘贴导致的一致性维护噩梦。〕

# 一、 定律与原则核心阐述

[The DRY Principle on Wikipedia](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)

> Every piece of knowledge must have a single, unambiguous, authoritative representation within a system.

DRY is an acronym for _Don't Repeat Yourself_. This principle aims to help developers reducing the repetition of code and keep the information in a single place and was cited in 1999 by Andrew Hunt and Dave Thomas in the book [The Pragmatic Programmer](https://en.wikipedia.org/wiki/The_Pragmatic_Programmer)

> The opposite of DRY would be _WET_ (Write Everything Twice or We Enjoy Typing).

In practice, if you have the same piece of information in two (or more) different places, you can use DRY to merge them into a single one and reuse it wherever you want/need.

See also:

- [The Pragmatic Programmer](https://en.wikipedia.org/wiki/The_Pragmatic_Programmer)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 核心工程实践与代码重用原则
