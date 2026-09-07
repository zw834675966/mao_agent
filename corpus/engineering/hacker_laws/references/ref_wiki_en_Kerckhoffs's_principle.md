---
title: "引用文献: Kerckhoffs's principle (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《Kerckhoffs's principle on Wikipedia》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Kerckhoffs's principle (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle](https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle)
- **引用锚文本**: Kerckhoffs's principle on Wikipedia
- **抓取状态**: success

# 二、 文献正文内容

Kerckhoffs's principle (also called Kerckhoffs's desideratum, assumption, axiom, doctrine or law) of cryptography was stated by the Dutch cryptographer Auguste Kerckhoffs in the 19th century. The principle holds that a cryptosystem should be secure, even if everything about the system, except the key, is public knowledge. This concept is widely embraced by cryptographers, in contrast to security through obscurity, which is not.
Kerckhoffs's principle was phrased by the American mathematician Claude Shannon as "the enemy knows the system", i.e., "one ought to design systems under the assumption that the enemy will immediately gain full familiarity with them".  In that form, it is called Shannon's maxim.
Another formulation by American researcher and professor Steven M. Bellovin is:

In other words—design your system assuming that your opponents know it in detail. (A former official at NSA's National Computer Security Center told me that the standard assumption there was that serial number 1 of any new device was delivered to the Kremlin.)


== Origins ==
The invention of telegraphy radically changed military communications and increased the number of messages that needed to be protected from the enemy dramatically, leading to the development of field ciphers which had to be easy to use without large confidential codebooks prone to capture on the battlefield. It was this environment which led to the development of Kerckhoffs's requirements.
Auguste Kerckhoffs was a professor of German language at HEC Paris. In early 1883, Kerckhoffs's article, La cryptographie militaire, was published in two parts in the Journal of Military Science, in which he stated six design rules for military ciphers.  Translated from French, they are:

The system must be practically, if not mathematically, indecipherable;
It should not require secrecy, and it should not be a problem if it falls into enemy hands;
It must be possible to communicate and remember the key without using written notes, and correspondents must be able to change or modify it at will;
It must be applicable to telegraph communications;
It must be portable, and should not require several persons to handle or operate;
Lastly, given the circumstances in which it is to be used, the system must be easy to use and should not be stressful to use or require its users to know and comply with a long list of rules.
Some are no longer relevant given the ability of computers to perform complex encryption. The second rule, now known as Kerckhoffs's principle, is still critically important.


== Explanation of the principle ==
Kerckhoffs viewed cryptography as a rival to, and a better alternative than, steganographic encoding, which was common in the nineteenth century for hiding the meaning of military messages. One problem with encoding schemes is that they rely on humanly-held secrets such as "dictionaries" which disclose for example, the secret meaning of words. Steganographic-like dictionaries, once revealed, permanently compromise a corresponding encoding system. Another problem is that the risk of exposure increases as the number of users holding the secrets increases.
Nineteenth century cryptography, in contrast, used simple tables which provided for the transposition of alphanumeric characters, generally given row-column intersections which could be modified by keys which were generally short, numeric, and could be committed to human memory. The system was considered "indecipherable" because tables and keys do not convey meaning by themselves. Secret messages can be compromised only if a matching set of table, key, and message falls into enemy hands in a relevant time frame. Kerckhoffs viewed tactical messages as only having a few hours of relevance. Systems are not necessarily compromised, because their components (i.e. alphanumeric character tables and keys) can be easily changed.


=== Advantage of secret keys ===
Using secure cryptography is supposed to replace the difficult problem of keeping messages secure with a much more manageable one, keeping relatively small keys secure. A system that requires long-term secrecy for something as large and complex as the whole design of a cryptographic system obviously cannot achieve that goal. It only replaces one hard problem with another. However, if a system is secure even when the enemy knows everything except the key, then all that is needed is to manage keeping the keys secret.
There are a large number of ways the internal details of a widely used system could be discovered. The most obvious is that someone could bribe, blackmail, or otherwise threaten staff or customers into explaining the system. In war, for example, one side will probably capture some equipment and people from the other side. Each side will also use spies to gather information.
If a method involves software, someone could do memory dumps or run the software under the control of a debugger in order to understand the method. If hardware is being used, someone could buy or steal some of the hardware and build whatever programs or gadgets needed to test it. Hardware can also be dismantled so that the chip details can be examined under the microscope.


=== Maintaining security ===
A generalization some make from Kerckhoffs's principle is: "The fewer and simpler the secrets that one must keep to ensure system security, the easier it is to maintain system security." Bruce Schneier ties it in with a belief that all security systems must be designed to fail as gracefully as possible:

Kerckhoffs's principle applies beyond codes and ciphers to security systems in general: every secret creates a potential failure point. Secrecy, in other words, is a prime cause of brittleness—and therefore something likely to make a system prone to catastrophic collapse. Conversely, openness provides ductility.
Any security system depends crucially on keeping some things secret. However, Kerckhoffs's principle points out that the things kept secret ought to be those least costly to change if inadvertently disclosed.
For example, a cryptographic algorithm may be implemented by hardware and software that is widely distributed among users. If security depends on keeping that secret, then disclosure leads to major logistic difficulties in developing, testing, and distributing implementations of a new algorithm – it is "brittle". On the other hand, if keeping the algorithm secret is not important, but only the keys used with the algorithm must be secret, then disclosure of the keys simply requires the simpler, less costly process of generating and distributing new keys.


== Applications ==
In accordance with Kerckhoffs's principle, the majority of civilian cryptography makes use of publicly known algorithms. By contrast, ciphers used to protect classified government or military information are often kept secret (see Type 1 encryption). However, it should not be assumed that government/military ciphers must be kept secret to maintain security. It is possible that they are intended to be as cryptographically sound as public algorithms, and the decision to keep them secret is in keeping with a layered security posture.


== Security through obscurity ==

It is moderately common for companies to keep the inner workings of a system secret. Some argue this "security by obscurity" makes the product safer and less vulnerable to attack. A counter-argument is that keeping the innards secret may improve security in the short term, but in the long run, only systems that have been published and analyzed should be trusted.
Steven Bellovin and Randy Bush commented:

Security Through Obscurity Considered Dangerous
Hiding security vulnerabilities in algorithms, software, and/or hardware decreases the likelihood they will be repaired and increases the likelihood that they can and will be exploited. Discouraging or outlawing discussion of weaknesses and vulnerabilities is extremely dangerous and deleterious to the security of computer systems, the network, and its citizens.
Open Discussion Encourages Better Security
The long history of cryptography and cryptoanalysis has shown time and time again that open discussion and analysis of algorithms exposes weaknesses not thought of by the original authors, and thereby leads to better and more secure algorithms.  As Kerckhoffs noted about cipher systems in 1883 [Kerc83], "Il faut qu'il n'exige pas le secret, et qu'il puisse sans inconvénient tomber entre les mains de l'ennemi."  (Roughly, "the system must not require secrecy and must be able to be stolen by the enemy without causing trouble.")


== References ==


== Notes ==
This article incorporates material from the Citizendium article "Kerckhoffs' Principle", which is licensed under the Creative Commons Attribution-ShareAlike 3.0 Unported License but not under the GFDL.


== External links ==
Reference to Kerckhoffs's original paper, with scanned original text
Caraco, Jean-Claude; Géraud-Stewart, Rémi; Naccache, David (2020). "Kerckhoffs' Legacy". Cryptology ePrint Archive. Paper 2020/556. Retrieved 26 November 2022.

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle_
