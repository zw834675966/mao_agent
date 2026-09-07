---
title: "引用文献: Law of Demeter (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Law_of_Demeter"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Law of Demeter (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Law_of_Demeter](https://en.wikipedia.org/wiki/Law_of_Demeter)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

The Law of Demeter (LoD) or principle of least knowledge is a design guideline for developing software, particularly object-oriented programs. In its general form, the LoD is a specific case of loose coupling. The guideline was proposed by Ian Holland at Northeastern University towards the end of 1987, and the following three recommendations serve as a succinct summary:

Each unit should have only limited knowledge about other units: only units "closely" related to the current unit.
Each unit should only talk to its friends; don't talk to strangers.
Only talk to your immediate friends.
The fundamental notion is that a given object should assume as little as possible about the structure or properties of anything else (including its subcomponents), in accordance with the principle of "information hiding". It may be viewed as a corollary to the principle of least privilege, which dictates that a module possess only the information and resources necessary for its legitimate purpose.
It is so named for its origin in the Demeter Project, an adaptive programming and aspect-oriented programming effort. The project was named in honor of Demeter, "distribution-mother" and the Greek goddess of agriculture, to signify a bottom-up philosophy of programming which is also embodied in the law itself.


== History ==
The law of Demeter dates back to 1987 when it was first proposed by Ian Holland, who was working on the Demeter Project. This project was the birthplace of a lot of aspect-oriented programming (AOP) principles.
A quote in one of the remainders of the project seems to clarify the origins of the name:

Demeter
The Greek goddess of Agriculture.
  The Demeter project was named after Demeter because we were working
  on a hardware description language Zeus and we were looking for a tool
  to simplify the implementation of Zeus. We were looking for a tool name
  related to Zeus and we chose a sister of Zeus: Demeter.
  We later promoted the idea that Demeter-style software development is
  about growing software as opposed to building software.
  We introduced the concept of a growth plan which is basically
  a sequence of more and more complex UML class diagrams.

  Growth plans are useful for building systems incrementally.


== In object-oriented programming ==
An object a can request a service (call a method) of an object instance b, but object a should not "reach through" object b to access yet another object, c, to request its services. Doing so would mean that object a implicitly requires greater knowledge of object b's internal structure.
Instead, b's interface should be modified if necessary so it can directly serve object a's request, propagating it to any relevant subcomponents. Alternatively, a might have a direct reference to object c and make the request directly to that. If the law is followed, only object b knows its own internal structure.
More formally, the Law of Demeter for functions requires that a method m of an object a may only invoke the methods of the following kinds of objects:

a itself;
m's parameters;
any objects instantiated within m;
a's attributes;
global variables accessible by a in the scope of m.
In particular, an object should avoid invoking methods of an object returned by another method. For many modern object-oriented languages that use a dot as field identifier, the law can be stated simply as "use only one dot". That is, the code a.m().n() breaks the law where a.m() does not. As an analogy, when one wants a dog to walk, one does not command the dog's legs to walk directly; instead, one commands the dog which then commands its own legs.


== Advantages ==
The advantage of following the Law of Demeter is that the resulting software tends to be more maintainable and adaptable. Since objects are less dependent on the internal structure of other objects, object implementation can be changed without reworking their callers.
Basili et al. published experimental results in 1996 suggesting that a lower Response For a Class (RFC, the number of methods potentially invoked in response to calling a method of that class) can reduce the probability of software bugs. Following the Law of Demeter can result in a lower RFC. However, the results also suggest that an increase in Weighted Methods per Class (WMC, the number of methods defined in each class) can increase the probability of software bugs. Following the Law of Demeter can also result in a higher WMC.
A multilayered architecture can be considered to be a systematic mechanism for implementing the Law of Demeter in a software system.
In a layered architecture, code within each layer can only make calls to code within the layer and code within the next layer down.
"Layer skipping" would violate the layered architecture.


== Disadvantages ==
Although the LoD increases the adaptiveness of a software system, it may result in having to write many wrapper methods to propagate calls to components; in some cases, this can add noticeable time and space overhead.
At the method level, the LoD leads to narrow interfaces, giving access to only as much information as it needs to do its job, as each method needs to know about a small set of methods of closely related objects. On the other hand, at the class level, if the LoD is not used correctly, wide (i.e., enlarged) interfaces may be developed that require introducing many auxiliary methods. This is due to poor design rather than a consequence of the LoD per se. If a wrapper method is being used, it means that the object being called through the wrapper should have been a dependency in the calling class.
One proposed solution to the problem of enlarged class interfaces is the aspect-oriented approach, where the behavior of the method is specified as an aspect at a high level of abstraction. The wide interfaces are managed through a language that specifies implementations. Both the traversal strategy and the adaptive visitor use only a minimal set of classes that participate in the operation, and the information about the connections between these classes is abstracted out.


== See also ==
Facade pattern
Principle of least astonishment
Single-responsibility principle


== References ==


== Further reading ==


== External links ==
Law of Demeter (LoD)
"Object-Oriented Programming: An Objective Sense of Style" (OOPSLA '88 Proceedings) (PDF)
The Paperboy, The Wallet, and The Law Of Demeter (PDF)
Phil Haack: "The Law of Demeter is not a Dot Counting Exercise"
Lieber: "Law of Demeter: Principle of Least Knowledge"
"Adaptive Object-Oriented Software, The Demeter Method"
The Demeter Project —- What is Demeter?

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Law_of_Demeter_
