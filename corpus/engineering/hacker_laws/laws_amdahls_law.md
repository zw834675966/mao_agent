---
title: "阿姆达尔定律 (Amdahl's Law)"
author: "吉恩·阿姆达尔 (Gene Amdahl)"
date: "1967"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "系统性能与并发定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "阿姆达尔定律"
  - "并行计算"
  - "加速比"
  - "系统性能"
  - "并发瓶颈"
---

〔阿姆达尔定律指出：系统通过增加计算资源所获得的加速比，严格受限于程序中无法并行化的串行部分比例。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Amdahl%27s_law)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E9%98%BF%E5%A7%86%E8%BE%BE%E5%B0%94%E5%AE%9A%E5%BE%8B)

> 阿姆达尔定律显示了计算任务通过增加系统资源可以获得的**加速潜力**。该公式通常用于并行计算中。它可以预测增加处理器数量的实际收益，该收益受到程序可并行比例的限制。

举例说明：如果程序由 A、B 两个部分组成，A 部分必须由单个处理器执行，B 部分可以并行运行。那么向执行程序的系统添加多个处理器只能获得有限的好处。它可以极大地提升 B 部分的运行速度，但 A 部分的运行速度将保持不变。

下图展示了一些运行速度的提升潜能的例子：

![阿姆达尔定律](./images/amdahls-law.svg)

_(图片来源：By Daniels220 at English Wikipedia, Creative Commons Attribution-Share Alike 3.0 Unported, https://en.wikipedia.org/wiki/File:AmdahlsLaw.svg)_

可以看出，50％ 并行化的程序在使用大于 10 个处理单元之后的速度提升收效甚微，而 95％ 并行化的程序在使用超过一千个处理单元之后仍然可以显著提升速度。

随着[摩尔定律](#%E6%91%A9%E5%B0%94%E5%AE%9A%E5%BE%8B-moores-law)减慢，单个处理器的速度增加缓慢，并行化是提高性能的关键。图形编程是一个极好的例子，现代着色器可以并行渲染单个像素或片段。这也是现代显卡通常具有数千个处理核心（GPU 或着色器单元）的原因。

参见：

- [布鲁克斯法则](#%E5%B8%83%E9%B2%81%E5%85%8B%E6%96%AF%E6%B3%95%E5%88%99-brookss-law)
- [摩尔定律](#%E6%91%A9%E5%B0%94%E5%AE%9A%E5%BE%8B-moores-law)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 系统性能与并发定律
