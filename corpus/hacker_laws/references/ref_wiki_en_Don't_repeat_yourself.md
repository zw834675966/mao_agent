---
title: "引用文献: Don't repeat yourself (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Don%27t_repeat_yourself"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《The DRY Principle on Wikipedia》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Don't repeat yourself (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Don%27t_repeat_yourself](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)
- **引用锚文本**: The DRY Principle on Wikipedia
- **抓取状态**: success

# 二、 文献正文内容

"Don't repeat yourself" (DRY) is a principle of software development aimed at reducing repetition of information which is likely to change, replacing it with abstractions that are less likely to change, or using data normalization which avoids redundancy in the first place.
The DRY principle is stated as "Every piece of knowledge must have a single, unambiguous, authoritative representation within a system". The principle has been formulated by Andy Hunt and Dave Thomas in their book The Pragmatic Programmer. They apply it quite broadly to include database schemas, test plans, the build system, even documentation.  When the DRY principle is applied successfully, a modification of any single element of a system does not require a change in other logically unrelated elements. Additionally, elements that are logically related all change predictably and uniformly, and are thus kept in sync. Besides using methods and subroutines in their code, Thomas and Hunt rely on code generators, automatic build systems, and scripting languages to observe the DRY principle across layers.


== Single choice principle ==
A particular case of DRY is the single choice principle.  It was defined by Bertrand Meyer as:
"Whenever a software system must support a set of alternatives, one and only one module in the system should know their exhaustive list." It was applied when designing Eiffel.


== Alternatives ==


=== WET ===
The opposing view to DRY is called WET, a backronym commonly taken to stand for write everything twice (alternatively write every time, we enjoy typing or waste everyone's time). WET solutions are common in multi-tiered architectures where a developer may be tasked with, for example, adding a comment field on a form in a web application. The text string "comment" might be repeated in the label, the HTML tag, in a read function name, a private variable, database DDL, queries, and so on. A DRY approach eliminates that redundancy by using frameworks that reduce or eliminate all those editing tasks except the most important ones, leaving the extensibility of adding new knowledge variables in one place.
This conceptualization of "WET" as an alternative to "DRY" programming has been around since at least 2002 in the Java world, though it is not known who coined the term.


=== AHA ===
Another approach to abstractions is to avoid hasty abstractions (AHA). AHA is rooted in the understanding that the deeper the investment engineers have made into abstracting a piece of software, the more they perceive that the cost of that investment can never be recovered (sunk cost fallacy). Thus, engineers tend to continue to iterate on the same abstraction each time the requirement changes. AHA programming assumes that both WET and DRY solutions inevitably create software that is rigid and difficult to maintain. Instead of starting with an abstraction, or abstracting at a specific number of duplications, software can be more flexible and robust if abstraction is done when it is needed, or, when the duplication itself has become the barrier and it is known how the abstraction needs to function. According to Kent C. Dodds, it is optimizing for change first, and avoiding premature optimization.
The term was coined by software engineer Cher Scarlett and the practice was described originally by software engineer and author Sandi Metz as "prefer duplication over the wrong abstraction".


== See also ==


== References ==


== External links ==
Don't Repeat Yourself at WikiWikiWeb
Once and Only Once at WikiWikiWeb
97 Things Every Programmer Should Know (O'Reilly)
The myth of over-normalization (discussion of academic extremes vs. real-world database scenarios)
Wilson G, Aruliah DA, Brown CT, Chue Hong NP, Davis M, Guy RT, et al. (2014). "Best Practices for Scientific Computing". PLOS Biol. 12 (1) e1001745. arXiv:1210.0530. doi:10.1371/journal.pbio.1001745. PMC 3886731. PMID 24415924. Don't Repeat Yourself (or Others)

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself_
