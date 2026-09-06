---
title: "引用文献: Robustness principle (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Robustness_principle"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Robustness principle (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Robustness_principle](https://en.wikipedia.org/wiki/Robustness_principle)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

In computing, the robustness principle is a design guideline for software that states: "be conservative in what you do, be liberal in what you accept from others". It is often reworded as: "be conservative in what you send, be liberal in what you accept". The principle is also known as Postel's law, after Jon Postel, who used the wording in an early specification of TCP.
In other words, programs that send messages to other machines (or to other programs on the same machine) should conform completely to the specifications, but programs that receive messages should accept non-conformant input as long as the meaning is clear.


== Interpretation ==
The principle was first written down by Jon Postel in the 1979 IPv4 specification. In 1989, Bob Braden expanded on Postel's principle by recommending that programmers "assume that the network is filled with malevolent entities that will send in packets designed to have the worst possible effect". Protocols should allow for the addition of new codes for existing fields in future versions of protocols by accepting messages with unknown codes (possibly logging them). Programmers should avoid sending messages with "legal but obscure protocol features" that might expose deficiencies in receivers, and design their code "not just to survive other misbehaving hosts, but also to cooperate to limit the amount of disruption such hosts can cause to the shared communication facility".


== Criticism ==
In 2001, Marshall Rose characterized several deployment problems when applying Postel's principle in the design of a new application protocol. For example, a defective implementation that sends non-conforming messages might be used only with implementations that tolerate those deviations from the specification until, possibly several years later, it is connected with a less tolerant application that rejects its messages. In such a situation, identifying the problem is often difficult, and deploying a solution can be costly. Rose therefore recommended, "explicit consistency checks in a protocol ... even if they impose implementation overhead".
In 2018, a paper on privacy-enhancing technologies by Florentin Rochet and Olivier Pereira showed how to exploit Postel's robustness principle inside the Tor routing protocol to compromise the anonymity of onion services and Tor clients.

In 2023, Martin Thomson and David Schinazi argued that Postel's robustness principle actually leads to a lack of robustness, including security:A flaw can become entrenched as a de facto standard. Any implementation of the protocol is required to replicate the aberrant behavior, or it is not interoperable. This is both a consequence of tolerating the unexpected and a product of a natural reluctance to avoid fatal error conditions. Ensuring interoperability in this environment is often referred to as aiming to be "bug-for-bug compatible".


== See also ==

Normalization of deviance
Open–closed principle
Protocol ossification
Static discipline
Unix philosophy


== References ==


== External links ==
History of the (Internet) Robustness Principle (Nick Gall, May 2005) Archived November 15, 2006, at the Wayback Machine
Internet Protocol Archived 2021-04-14 at the Wayback Machine, page 22; J. Postel, IEN 111, August 1979.

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Robustness_principle_
