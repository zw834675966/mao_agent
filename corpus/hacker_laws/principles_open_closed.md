---
title: "开闭原则 (The Open/Closed Principle / OCP)"
author: "伯特兰·迈耶 (Bertrand Meyer)"
date: "1988"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "面向对象设计原则 (O in SOLID)"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "开闭原则"
  - "OCP"
  - "SOLID"
  - "对扩展开放"
  - "对修改关闭"
---

〔开闭原则主张：软件实体（类、模块、函数等）应当对扩展开放，对修改关闭。当系统引入新需求时，应通过新增代码而非修改既有经测试代码来实现。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%BC%80%E9%97%AD%E5%8E%9F%E5%88%99)

> 实体应开放扩展并关闭修改。

[SOLID](#solid) 的第二个原则。这个原则指出实体（可以是类、模块、函数等）应该能够使它们的行为易于扩展，但是它们的扩展行为不应该被修改。

举一个假设的例子，想象一个能够将 Markdown 转换为 HTML 的模块。如果可以扩展模块，而不修改内部模块来处理新的 markdown 特征，而无需修改内部模块，则可以认为是开放扩展。如果用户不能修改处理现有 Markdown 特征的模块，那么它被认为是关闭修改。

这个原则与面向对象编程紧密相关，让我们可以设计对象以便于扩展，但是可以避免以意想不到的方式改变其现有对象的行为。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 面向对象设计原则 (O in SOLID)
