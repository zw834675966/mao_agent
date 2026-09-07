---
title: "随机鹦鹉理论 (The Stochastic Parrot)"
author: "埃米莉·M·本德、蒂姆尼特·格布鲁 等 (Emily M. Bender, Timnit Gebru et al.)"
date: "2021"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "自然语言模型批判与认知局限理论"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "随机鹦鹉"
  - "大型语言模型"
  - "幻觉批判"
  - "统计模式匹配"
  - "真正理解与符号推理"
---

〔随机鹦鹉理论批判指出：大型语言模型本质上是通过统计概率拼接庞大训练语料库中的语言形式，缺乏对外部世界真实语义、意图及逻辑的真正深层理解。〕

# 一、 定律与原则核心阐述

[On the Dangers of Stochastic Parrots - Bender, Gebru, et al. (2021)](https://dl.acm.org/doi/10.1145/3442188.3445922)

> Contrary to how it may seem when we observe its output, an LM is a system for haphazardly stitching together sequences of linguistic forms it has observed in its vast training data, according to probabilistic information about how they combine, but without any reference to meaning: a stochastic parrot.
>
> _Emily M. Bender, Timnit Gebru, et al. (2021)_

The term argues that Large Language Models (LLMs) produce statistically likely sequences of text based on training data, without genuine comprehension. Essentially - confident-sounding output is not evidence of correctness or understanding.

Models can (and do) "hallucinate" - producing plausible sounding output or confidently making statements which are demonstrably wrong. This does not devalue these models, but highlights important characteristics which must be accounted for when using them.

See also:

- [The Bitter Lesson](#the-bitter-lesson)
- [All Models Are Wrong (George Box's Law)](#all-models-are-wrong-george-boxs-law)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 自然语言模型批判与认知局限理论
