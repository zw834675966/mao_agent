---
title: "切斯特顿围栏 (Chesterton's Fence)"
author: "G·K·切斯特顿 (G. K. Chesterton)"
date: "1929"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "遗留系统重构与认知审慎原则"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "切斯特顿围栏"
  - "遗留系统重构"
  - "未知历史背景"
  - "谨慎重构"
  - "软件维护"
---

〔切斯特顿围栏指出：如果你在路中间看到一堵看似多余的围栏，在你弄明白它为什么被建在那里之前，绝不要擅自拆除它。在重构遗留代码时切忌盲目删去看似多余的古怪检查。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Wikipedia:Chesterton%27s_fence)

> 在了解现有情况背后的原因之前，不应该进行改进。

该原则与软件工程中的消除技术负债 (Technical debt) 相关。程序的每一行最初都是出于某种原因编写的，因此根据切斯特森围栏原则，在更改或删除代码之前，即使看起来似乎是多余的或不正确的，也应该尝试完全理解代码的上下文和含义。

该原则的名字来源于 [G.K. Chesterson](https://en.wikipedia.org/wiki/G._K._Chesterton) 的一则故事。一个男人横穿马路中央的栅栏，他向市长抱怨这道栅栏没有用还挡路，并要求拆除它。市长问他为什么要在那里建栅栏，那个人回答说不知道。市长接着说：“如果你不知道它的用途，我肯定不会让你把它拆了。你去查查它的用途，之后我可能会允许你拆掉它。”

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 遗留系统重构与认知审慎原则
