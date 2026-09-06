---
title: "柯林汉定律 (Kernighan's Law)"
author: "布莱恩·柯林汉 (Brian Kernighan / 《C程序设计语言》作者)"
date: "1978"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "可维护性与调试难度定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "柯林汉定律"
  - "代码调试"
  - "极简代码"
  - "聪明反被聪明误"
  - "代码可读性"
---

〔柯林汉定律断言：调试代码的难度是初次编写代码的两倍。因此，如果你在写代码时用尽了全部聪明才智，根据定义你将没有足够的能力去调试它。〕

# 一、 定律与原则核心阐述

> 调试在一开始就比编写程序困难一倍。因此，按照定义，如果你的代码写得非常巧妙，那么你就没有足够的能力来调试它。
>
> _布莱恩·柯林汉 (Brian Kernighan)_

柯林汉定律是以[布莱恩·柯林汉](https://zh.wikipedia.org/wiki/%E5%B8%83%E8%90%8A%E6%81%A9%C2%B7%E6%9F%AF%E6%9E%97%E6%BC%A2) ([Brian Kernighan](https://en.wikipedia.org/wiki/Brian_Kernighan)) 的名字命名的，引述自柯林汉和普劳格 (P.J. Plauger) 的《[编程格调](https://book.douban.com/subject/26335120/)》 ([The Elements of Programming Style](https://en.wikipedia.org/wiki/The_Elements_of_Programming_Style)) 一书中的一句话：

> 每个人都知道，调试在一开始就比编写程序困难一倍。那么，如果您在编写它时尽可能地巧妙，又如何来调试它？

尽管这有些夸张，但它提出的论点是，简单的代码会比复杂的代码更可取，因为调试复杂代码的过程中出现的任何问题都会十分棘手，甚至无法解决。

参见：

- [KISS 原则 (The KISS Principle)](#kiss-%e5%8e%9f%e5%88%99-the-kiss-principle)
- [Unix 哲学 (The Unix Philosophy)](#unix-%e5%93%b2%e5%ad%a6-the-unix-philosophy)
- [奥卡姆剃刀 (Occam's Razor)](#%e5%a5%a5%e5%8d%a1%e5%a7%86%e5%89%83%e5%88%80-occams-razor)

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 可维护性与调试难度定律
