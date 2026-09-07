---
title: "引用文献: Single-responsibility principle (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Single_responsibility_principle"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Single-responsibility principle (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Single_responsibility_principle](https://en.wikipedia.org/wiki/Single_responsibility_principle)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

The single-responsibility principle (SRP) is a computer programming principle that states that "A module should be responsible to one, and only one, actor." The term actor refers to a group (consisting of one or more stakeholders or users) that requires a change in the module.
Robert C. Martin, the originator of the term, expresses the principle as, "A class should have only one reason to change". Because of confusion around the word "reason", he later clarified his meaning in a blog post titled "The Single Responsibility Principle", in which he mentioned Separation of Concerns and stated that "Another wording for the Single Responsibility Principle is: Gather together the things that change for the same reasons. Separate those things that change for different reasons." In some of his talks, he also argues that the principle is, in particular, about roles or actors. For example, while they might be the same person, the role of an accountant is different from a database administrator. Hence, each module should be responsible for each role.


== History ==
The term was introduced by Robert C. Martin in his article "The Principles of OOD" as part of his Principles of Object Oriented Design, made popular by his 2003 book Agile Software Development, Principles, Patterns, and Practices. Martin described it as being based on the principle of cohesion, as described by Tom DeMarco in his book Structured Analysis and System Specification, and Meilir Page-Jones in The Practical Guide to Structured Systems Design. In 2014 Martin published a blog post titled "The Single Responsibility Principle" with a goal to clarify what was meant by the phrase "reason for change."


== Example ==
Martin defines a responsibility as a reason to change, and concludes that a class or module should have one, and only one, reason to be changed (e.g. rewritten). 
As an example, consider a module that compiles and prints a report. Imagine such a module can be changed for two reasons. First, the content of the report could change. Second, the format of the report could change. These two things change for different causes. The single responsibility principle says that these two aspects of the problem are really two separate responsibilities, and should, therefore, be in separate classes or modules. It would be a bad design to couple two things that change for different reasons at different times.
The reason it is important to keep a class focused on a single concern is that it makes the class more robust. Continuing with the foregoing example, if there is a change to the report compilation process, there is a greater danger that the printing code will break if it is part of the same class.


== See also ==
Chain-of-responsibility pattern
Coupling (computer programming)
GRASP (object-oriented design)
Information hiding
SOLID—the "S" in "SOLID" represents the single responsibility principle
Separation of concerns


== References ==


== External links ==
"The Principles of OOD" by Robert Martin
"The Single Responsibility Principle (2007)" by Robert Martin
"The Single Responsibility Principle (2014)" by Robert Martin

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Single_responsibility_principle_
