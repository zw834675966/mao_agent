---
title: "抽象泄漏定律 (The Law of Leaky Abstractions)"
author: "乔尔·斯波尔斯基 (Joel Spolsky / Stack Overflow联合创始人)"
date: "2002"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "软件分层与底层透明性定律"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "抽象泄漏定律"
  - "分层抽象"
  - "TCP协议抽象"
  - "ORM隐患"
  - "底层调试"
---

〔乔尔·斯波尔斯基指出：所有重大的抽象机制，在某种程度上都是有漏洞的（All non-trivial abstractions, to some degree, are leaky）。一旦发生故障，开发者依然必须精通其底层机制。〕

# 一、 定律与原则核心阐述

- [英文在线地址](https://www.joelonsoftware.com/2002/11/11/the-law-of-leaky-abstractions/)

> 在某种程度上，所有非平凡的抽象都是有泄漏的。
>
> [乔尔斯·波尔斯基](https://twitter.com/spolsky) (Joel Spolsky)

该定律指出，通常用于简化复杂系统的抽象，在某些情况下将底层系统泄漏出来，使得抽象表现出意外的行为。

例如加载文件并读取其内容。文件系统 API 是较低级别内核系统的抽象，它们本身是与磁盘（或 SSD 的闪存）上的数据更改相关的物理过程的抽象。在大多数情况下，处理文件（如二进制数据流）的抽象将起作用。但是，对于磁盘驱动器，顺序读取数据将比随机访问快得多（由于页面错误的开销增加）。但对于 SSD 驱动器，此开销不会出现。需要理解基础细节来处理这种情况（例如，数据库索引文件的良好结构可以减少随机访问的开销），开发人员需要合理的抽象，来处理不同的细节。

当引入更多的抽象时，上面的例子会变得更复杂。Linux 操作系统允许通过网络访问文件，但在本地表示为**普通**文件。如果存在网络故障，这种抽象将会**泄漏**。如果开发人员将这些文件视为**普通**文件，而不考虑它们可能会受到网络延迟和故障的影响，那么解决方案就会出错。

描述该定律的文章表明，过度依赖抽象，加上对底层过程的理解不足，实际上使得问题在某些情况下更加复杂。

参见：

- [隐式接口定律](#%E9%9A%90%E5%BC%8F%E6%8E%A5%E5%8F%A3%E5%AE%9A%E5%BE%8B-hyrums-law-or-the-law-of-implicit-interfaces)

真实的例子：

- [Photoshop 启动缓慢](https://forums.adobe.com/thread/376152)：我过去遇到过一个问题，就是 Photoshop 启动缓慢，有时需要几分钟。问题好像是 Photoshop 启动时，会读取当前默认打印机的一些信息。但是，如果该打印机实际上是一台网络打印机，则可能需要很长的时间。将网络打印机与本地打印机当作同样的抽象，导致连接不良的情况下出现问题。

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · 软件分层与底层透明性定律
