---
title: "最小惊奇原则 (The Principle of Least Astonishment / POLA)"
author: "UNIX 哲学传承与系统人机工程"
date: "1970"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "API设计与用户心理契约原则"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "最小惊奇原则"
  - "POLA"
  - "API直觉性"
  - "一致性体验"
  - "符合直觉"
---

〔最小惊奇原则申明：在设计组件、接口或用户交互时，其行为应当符合用户的普遍心理预期，绝不应该展现出令经验丰富的使用者感到不可思议或惊愕的突兀怪异行为。〕

# 一、 定律与原则核心阐述

[The Principle of Least Astonishment on Wikipedia](https://en.wikipedia.org/wiki/Principle_of_least_astonishment)

> People are part of the system. The design should match the user's experience, expectations, and mental models.
>
> Frans Kaashoek

This principle proposes that systems and interfaces should be designed in a way that features and functionality is easily discovered and matches users expectations. Features that 'surprise' users should be discouraged in favour of features that can be intuitively reasoned about based on existing patterns and practices.

Many examples are present in user interfaces, such as a 'pull down' gesture on a mobile appliation to refresh content. Another example would be command line tools, where many standards exist for how parameters are named, common parameters that should be available and so on.

See also:

- [Convention Over Configuration](#todo)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · API设计与用户心理契约原则
