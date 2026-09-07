---
title: "引用文献: Second-system effect (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Second-system_effect"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《The Second-System Effect on Wikipedia》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Second-system effect (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Second-system_effect](https://en.wikipedia.org/wiki/Second-system_effect)
- **引用锚文本**: The Second-System Effect on Wikipedia
- **抓取状态**: success

# 二、 文献正文内容

The second-system effect (also second-system syndrome) is the tendency for a successful first system (often small and relatively elegant) to be followed by a second system that becomes over-engineered or bloated. The effect is commonly attributed to increased confidence after the first success and to accumulated ideas that were deferred from the first system and then added en masse to the second.
Fred Brooks introduced the phrase in The Mythical Man-Month (1975) while describing IBM's transition from relatively simple operating systems for the IBM 700/7000 series to the much more ambitious OS/360 for the IBM System/360 family (announced in 1964).


== Description ==
In Brooks's formulation, an architect's first system is often "spare and clean" because the designer is still learning and is cautious about uncertain generalizations. The second system is "the most dangerous" because the designer is more confident and is tempted to incorporate every previously deferred improvement, optional feature, and generalization, resulting in a successor that is harder to build, understand, and evolve.
The effect is closely related to feature creep and to design over-generalization, where the second system attempts to anticipate too many future needs at once rather than serving current, validated requirements.


== Additional manifestation ==
Brooks also described a second variant of the effect that is not primarily about adding features: a tendency to refine techniques that have become obsolete because the system's basic assumptions have changed (for example, optimizing around an old hardware or operational model after the environment has shifted). He cited OS/360 as containing many examples of this kind of misapplied refinement.


== Relation to rewrites, prototypes, and planned replacement ==
The second-system effect is frequently discussed in the context of major rewrites (a "version 2" or second implementation), but it can occur in any second large-scale system built after an initial success.
In The Mythical Man-Month, Brooks separately argues that teams should often expect to build a pilot (or throwaway) system to learn what is actually needed; the management question is whether to plan for that throwaway in advance or to mistakenly ship it as the final product. This view is echoed in later system-design literature: Butler Lampson recommends "plan to throw one away" and notes that if there is anything genuinely new about a system's function, the first implementation will likely need to be redone.
As a result, some teams deliberately schedule a second implementation to remove early mistakes, false generalizations, and exploratory scaffolding from the first iteration. This "planned replacement" approach is sometimes framed as sacrificial architecture: accepting that parts or all of the current architecture will be replaced once the domain is better understood, and designing in ways that make replacement easier when the time comes.
A common mitigation is incremental replacement (often described as the "Strangler Fig" approach), where new functionality is built around the legacy system and gradually replaces it, reducing the risk of a single all-at-once second system.


== Mitigation ==
Brooks suggested that avoiding second-system failure requires explicit discipline, including:

resisting "functional ornamentation" and unnecessary generalization,
making resource costs visible for small features (e.g., budgeting memory and performance costs per capability),
ensuring experienced architectural leadership (including architects who have already designed multiple comparable systems).
In practice, mitigation strategies often include prioritizing validated requirements, staged delivery, strict scope management, and architectural review processes designed to challenge speculative features.


== See also ==

Anti-pattern
Feature creep
Inner-platform effect
Osborne effect
Sophomore slump
Unix philosophy
Big ball of mud
Strangler fig pattern


== References ==


== External links ==
Spolsky, Joel (April 6, 2000). "Things You Should Never Do, Part I". Joel on Software. Retrieved October 15, 2021.
Turoff, Adam (August 21, 2007). "Notes on Haskell". Retrieved October 15, 2021.
Gunton, Neil (July 20, 2008). "Rewrites Considered Harmful?". Retrieved October 15, 2021.
Fowler, Chad. "The Big Rewrite". Archived from the original on December 8, 2016.

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Second-system_effect_
