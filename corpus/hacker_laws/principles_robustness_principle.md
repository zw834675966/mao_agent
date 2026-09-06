---
title: "稳健原则 / 波斯塔尔法则 (The Robustness Principle or Postel's Law)"
author: "乔恩·波斯塔尔 (Jon Postel / 互联网协议先驱)"
date: "1980"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "协议互操作与容错通信原则"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "波斯塔尔法则"
  - "稳健原则"
  - "宽进严出"
  - "协议设计"
  - "容错能力"
---

〔波斯塔尔法则奉行：“对自己严格，对他人宽容”（Be conservative in what you send, be liberal in what you accept）。发送符合严格标准的数据，宽容接纳非严格标准的外部输入。〕

# 一、 定律与原则核心阐述

- [英文维基百科](https://en.wikipedia.org/wiki/Robustness_principle)

> 在自己所做的事情上要保守, 在接受别人的事情上要自由。

通常应用于服务器应用程序开发中，该原则指出，你发送给其他人的内容应尽可能最小且符合要求，并且处理不符合要求的输入。

该原则的目标是构建稳健的系统。如果可以理解意图，它们可以处理不良的输入。但是，接受错误格式的输入可能存在安全隐患，特别是此类的输入未经过充分测试。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 协议互操作与容错通信原则
