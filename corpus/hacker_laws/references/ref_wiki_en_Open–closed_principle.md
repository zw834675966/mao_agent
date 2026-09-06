---
title: "引用文献: Open–closed principle (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Open–closed principle (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

In object-oriented programming, the open–closed principle (OCP) states "software entities (classes, modules, functions, etc.) should be open for extension, but closed for modification";
that is, such an entity can allow its behaviour to be extended without modifying its source code.
The name open–closed principle has been used in two ways. Both ways use generalizations (for instance, inheritance or delegate functions) to resolve the apparent dilemma, but the goals, techniques, and results are different.
The open–closed principle is one of the five SOLID principles of object-oriented design.


== Meyer's open–closed principle ==
Bertrand Meyer is generally credited for having originated the term open–closed principle, which appeared in his 1988 book Object-Oriented Software Construction.

A module will be said to be open if it is still available for extension. For example, it should be possible to add fields to the data structures it contains, or new elements to the set of functions it performs.
A module will be said to be closed if [it] is available for use by other modules. This assumes that the module has been given a well-defined, stable description (the interface in the sense of information hiding).

At the time Meyer was writing, adding fields or functions to a library inevitably required changes to any programs depending on that library.  Meyer's proposed solution to this problem relied on the notion of object-oriented inheritance (specifically implementation inheritance):

A class is closed, since it may be compiled, stored in a library, baselined, and used by client classes. But it is also open, since any new class may use it as parent, adding new features. When a descendant class is defined, there is no need to change the original or to disturb its clients.


== Polymorphic open–closed principle ==
During the 1990s, the open–closed principle became popularly redefined to refer to the use of abstracted interfaces, where the implementations can be changed and multiple implementations could be created and polymorphically substituted for each other.
In contrast to Meyer's usage, this definition advocates inheritance from abstract base classes. Interface specifications can be reused through inheritance but implementation need not be. The existing interface is closed to modifications and new implementations must, at a minimum, implement that interface.
Robert C. Martin's 1996 article "The Open-Closed Principle" was one of the seminal writings to take this approach. In 2001, Craig Larman related the open–closed principle to the pattern by Alistair Cockburn called Protected Variations, and to the David Parnas discussion of information hiding.


== See also ==
SOLID – the "O" in "SOLID" represents the open–closed principle
Robustness principle


== References ==


== External links ==
The Principles of OOD
The Open/Closed Principle: Concerns about Change in Software Design
The Open-Closed Principle -- and What Hides Behind It

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle_
