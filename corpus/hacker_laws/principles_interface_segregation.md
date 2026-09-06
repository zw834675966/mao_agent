---
title: "接口隔离原则 (The Interface Segregation Principle / ISP)"
author: "罗伯特·C·马丁 (Robert C. Martin)"
date: "2000"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "面向对象设计原则 (I in SOLID)"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "接口隔离原则"
  - "ISP"
  - "SOLID"
  - "胖接口拆分"
  - "高内聚接口"
---

〔接口隔离原则要求：客户端不应该被迫依赖于它们不使用的方法。宁可定义多个专门细粒度的瘦接口，也不要定义单一臃肿的万能胖接口。〕

# 一、 定律与原则核心阐述

- [英文在线地址](http://www.hyrumslaw.com/)

> 当 API 有足够多的用户时，你在合同中的承诺已不重要：你系统的所有可观察行为都将被某些人所依赖。
>
> _海勒姆·赖特 (Hyrum Wright)_

隐式接口定律表明，当你的 API 有足够多的用户时，API 的所有行为（包括那些未囊括在公共说明中的一部分）最终都会被其他人所依赖。 一个简单的例子是 API 的响应时间这种非功能性因素；还有一个更微妙的例子是：用户使用正则表达式匹配错误提示来判断 API 的错误类型，即使 API 文档中没有任何关于错误提示的内容，而是指导用户应该使用相应的错误代码。一些用户依然会使用错误提示内容（而非错误代码），这种情况下变更 API 错误提示信息，实际上会破坏 API 的使用。

参见：

- [抽象泄漏定律](#%E6%8A%BD%E8%B1%A1%E6%B3%84%E6%BC%8F%E5%AE%9A%E5%BE%8B-the-law-of-leaky-abstractions)
- [XKCD 1172](https://xkcd.com/1172/)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 面向对象设计原则 (I in SOLID)
