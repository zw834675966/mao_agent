---
title: "海勒姆定律 / 隐式接口定律 (Hyrum's Law)"
author: "海勒姆·赖特 (Hyrum Wright / Google)"
date: "2012"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "API设计与契约约束定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "海勒姆定律"
  - "隐式接口"
  - "向下兼容"
  - "破坏性变更"
  - "API契约"
---

〔海勒姆定律指出：当一个API的用户足够多时，在接口规范中未承诺的所有系统实现细节与观察行为，最终都会被某些调用方依赖。〕

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
- **所属文库分类**: 黑客定律与工程哲学文库 · API设计与契约约束定律
