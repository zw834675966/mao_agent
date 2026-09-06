---
title: "单一职责原则 (The Single Responsibility Principle / SRP)"
author: "罗伯特·C·马丁 (Robert C. Martin)"
date: "2000"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "面向对象设计原则 (S in SOLID)"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "单一职责原则"
  - "SRP"
  - "SOLID"
  - "变更原因单一"
  - "模块内聚"
---

〔单一职责原则强调：一个模块或类应该且仅应该对某一类利益相关者（或某一个变更原因）负责。引起它变化的原因有且只有一个。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Single_responsibility_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%8D%95%E4%B8%80%E5%8A%9F%E8%83%BD%E5%8E%9F%E5%88%99)

> 每个模块或者类只应该有一项功能。

[SOLID](#solid) 的第一个原则。这个原则表明模块或者类只应该做一件事。实际上，这意味着对程序功能的单个小更改，应该只需要更改一个组件。例如，更改密码验证复杂性的方式应该只需要更改程序的一部分。

理论上讲，这使代码更健壮，更容易更改。知道正在更改的组件只有一个功能，这意味着测试更改更容易。使用前面的例子，更改密码复杂性组件应该只影响与密码复杂性相关的功能。变更具有许多功能的组件可能要困难得多。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 面向对象设计原则 (S in SOLID)
