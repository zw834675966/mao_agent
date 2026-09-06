---
title: "第二系统效应 (The Second-System Effect)"
author: "弗雷德·布鲁克斯 (Fred Brooks / 《人月神话》)"
date: "1975"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "系统重构与过度设计陷阱"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "第二系统效应"
  - "人月神话"
  - "重构失败"
  - "功能膨胀"
  - "过度设计"
---

〔布鲁克斯指出：在设计第二个系统时，由于在第一个系统受限的压抑野心爆发，设计师往往试图把所有未能实现的想法全部塞入，导致第二系统变得极度臃肿、复杂甚至夭折。〕

# 一、 定律与原则核心阐述

[The Second-System Effect on Wikipedia](https://en.wikipedia.org/wiki/Second-system_effect)

[The Second-System Effect in _The Mythical Man-Month_](https://pages.cs.wisc.edu/~param/quotes/man-month.html)

> The second is the most dangerous system a man ever designs.
>
> (Frederick P. Brooks Jr.)

The Second-System Effect describes the tendency for a successful first system to be followed by an over-engineered or bloated second system. Frederick P. Brooks Jr. introduced the phrase in _The Mythical Man-Month_, where he argued that a first system is often built conservatively because its designers are still learning and proceed with restraint.

After that initial success, deferred ideas and increased confidence can make the second system more ambitious. This is relevant to rewrites, major version 2 projects and platform rebuilds: a replacement system can be useful when the first version has taught the team what the real problem is, but it can also become harder to build, understand and maintain if it attempts to solve too many speculative future needs at once.

See also:

- [Brooks' Law](#brooks-law)
- [Gall's Law](#galls-law)
- [The KISS principle](#the-kiss-principle)
- [YAGNI](#yagni)

Real-world examples:

- [OS/360](https://en.wikipedia.org/wiki/OS/360) - Brooks used IBM's OS/360 as a key example of the Second-System Effect in _The Mythical Man-Month_, contrasting its ambitious scope with earlier, simpler IBM operating systems.

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 系统重构与过度设计陷阱
