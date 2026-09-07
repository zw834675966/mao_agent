---
title: "柯克霍夫原则 (Kerckhoffs's Principle)"
author: "奥古斯特·柯克霍夫 (Auguste Kerckhoffs)"
date: "1883"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "现代密码学与系统安全基础原则"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "柯克霍夫原则"
  - "隐蔽式安全谬误"
  - "现代密码学"
  - "密钥管理"
  - "开源安全"
---

〔柯克霍夫原则申明：即使密码系统的所有设计细节与算法都被敌方所知，只要密钥没有泄露，该系统依然必须是安全的。拒绝“通过隐蔽求安全”（Security through obscurity）。〕

# 一、 定律与原则核心阐述

[Kerckhoffs's principle on Wikipedia](https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle)

> "...design your system assuming that your opponents know it in detail."
>
> _Steven M. Bellovin's formulation of Kerckhoff's Principle_

This principle of cryptography was an axiom created by cryptographer Auguste Kerckhoffs. He stated that a cryptosystem should be secure, even if everything about the system, except the key, is public knowledge. Not to be confused with [_"security through obscurity"_](#todo).

The gold standard for any secret-keeping system is that implementation details should be publicly distributed, without sacrificing or compromising security of said system.

The history of cryptography has shown that open discussion and analysis of cryptographic systems leads to better and more secure systems - as researchers are able to test for and expose potential vulnerabilities.

- [Shannon's Maxim](#todo)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 现代密码学与系统安全基础原则
