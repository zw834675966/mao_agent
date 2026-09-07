---
title: "两张披萨团队原则 (The Two Pizza Rule)"
author: "杰夫·贝索斯 (Jeff Bezos / Amazon)"
date: "2002"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "团队组织规模与微服务契约"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "两张披萨原则"
  - "团队规模"
  - "亚马逊敏捷"
  - "组织解耦"
  - "沟通成本控制"
---

〔亚马逊创始人贝索斯提出：一个高产的工程团队规模，不应该超过两张大披萨能吃饱的人数（通常为6到10人）。这极大削减了团队内部的沟通开销，促进独立自治交付。〕

# 一、 定律与原则核心阐述

> If you can't feed a team with two pizzas, it's too large.
>
> (Jeff Bezos)

This rule suggests that regardless of the size of the company, teams should be small enough to be fed by two pizzas. Attributed to Jeff Bezos and Amazon, this belief suggests that large teams are inherently inefficient. This is supported by the fact that as the team size increases linearly, the links between people increases quadratically; thus the cost of coordinating and communicating also grows quadratically. If this cost of coordination is essentially overhead, then smaller teams should be preferred.

The number of links between people can be expressed as `n(n-1)/2` where n = number of people.

<img width="220px" alt="Diagram: a complete graph of seven nodes, showing a link between every pair of people" src="./images/complete-graph.svg" />

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 团队组织规模与微服务契约
