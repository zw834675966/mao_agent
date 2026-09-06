---
title: "里氏替换原则 (The Liskov Substitution Principle / LSP)"
author: "芭芭拉·利斯科夫 (Barbara Liskov / 图灵奖得主)"
date: "1987"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "面向对象设计原则 (L in SOLID)"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "里氏替换原则"
  - "LSP"
  - "SOLID"
  - "子类型契约"
  - "行为多态"
---

〔里氏替换原则指出：子类型必须能够完全替换掉它们的基类型，而不会改变程序的正确性与预期行为。继承不应破坏基类契约约束。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Liskov_substitution_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E9%87%8C%E6%B0%8F%E6%9B%BF%E6%8D%A2%E5%8E%9F%E5%88%99)

> 可以在不破坏系统的情况下，用子类型替换类型。

[SOLID](#solid) 的第三个原则。该原则指出，如果组件依赖于类型，那么它应该能够使用该类型的子类型，而不会导致系统失败或者必须知道该子类型的详细信息。

举个例子，假设我们有一个方法，读取 XML 文档。如果该方法使用基类型 **file**，则从 **file** 派生的任何内容，都能用在该方法中。 如果 **file** 支持反向查找，并且 xml 解析器使用该函数，但是派生类型 **network file** 尝试反向查找时失败，则 **network file** 将违反该原则。

该原则与面向对象编程紧密相关，必须仔细建模、层次结构，以避免让系统用户混淆。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 面向对象设计原则 (L in SOLID)
