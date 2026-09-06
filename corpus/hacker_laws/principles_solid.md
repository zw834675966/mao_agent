---
title: "SOLID 软件设计五大原则总论 (SOLID Principles)"
author: "罗伯特·C·马丁 (Robert C. Martin / Uncle Bob)"
date: "2000"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "面向对象架构设计核心五大原则"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "SOLID原则"
  - "面向对象设计"
  - "架构整洁之道"
  - "松耦合高内聚"
  - "可维护性"
---

〔SOLID是由罗伯特·C·马丁整理的五个面向对象设计核心原则缩写：单一职责（S）、开闭原则（O）、里氏替换（L）、接口隔离（I）、依赖反转（D）。它们是现代可维护软件架构的奠基支柱。〕

# 一、 定律与原则核心阐述

这是一个缩写，指的是：

- S：[单一功能原则 (The Single Responsibility Principle)](#%E5%8D%95%E4%B8%80%E5%8A%9F%E8%83%BD%E5%8E%9F%E5%88%99-the-single-responsibility-principle)
- O：[开闭原则 (The Open/Closed Principle)](#%E5%BC%80%E9%97%AD%E5%8E%9F%E5%88%99-the-openclosed-principle)
- L：[里氏替换原则 (The Liskov Substitution Principle)](#%E9%87%8C%E6%B0%8F%E6%9B%BF%E6%8D%A2%E5%8E%9F%E5%88%99-the-liskov-substitution-principle)
- I：[接口隔离原则 (The Interface Segregation Principle)](#%E6%8E%A5%E5%8F%A3%E9%9A%94%E7%A6%BB%E5%8E%9F%E5%88%99-the-interface-segregation-principle)
- D：[依赖反转原则 (The Dependency Inversion Principle)](#%E4%BE%9D%E8%B5%96%E5%8F%8D%E8%BD%AC%E5%8E%9F%E5%88%99-the-dependency-inversion-principle)

这些是 [Object-Oriented Programming](#todo) 的关键原则。诸如此类的设计原则能够帮助开发人员构建更易于维护的系统。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 面向对象架构设计核心五大原则
