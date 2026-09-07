---
title: "软件设计与架构核心原则权威文献大全"
author: "dwmkerr 与全球黑客定律开源贡献者"
date: "2024"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "软件设计与架构原则合辑"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "SOLID原则"
  - "DRY原则"
  - "KISS原则"
  - "YAGNI"
  - "分布式计算谬论"
  - "最小惊奇原则"
---

〔本篇辑录现代软件架构、面向对象与工程治理中最重要的21条设计原则、反思与批判理论。〕


# 1、 所有模型都是错的 / 乔治·博克斯定律 (All Models Are Wrong)

〔统计学家乔治·博克斯经典论断：“所有模型都是错的，但其中有一些是有用的”（All models are wrong, but some are useful）。在领域建模中不应追求百分百复刻现实，而应追求对解决核心问题最有用的抽象。〕

- [英文维基百科](https://en.wikipedia.org/wiki/All_models_are_wrong)

> All models are wrong, but some are useful.
> 所有的模型都是错的，但有些是有用的。
>
> _乔治·伯克斯 (George Box)_

这一原则表明，所有的系统模型都是有缺陷的，但只要它们没有太多缺陷，那便有可能是有用的。这一原则源于统计学，同时也适用于科学和计算模型。

大多数软件的一个基本要求都是对某种特定系统进行建模。无论是计算机网络、资源库、社会关系图还是任何其他类型的系统，设计者都必须依据适当的细节程度来建模。过多的细节可能会导致太高的复杂度，过少的细节可能会使模型无法正常工作。

参见:

- [抽象泄漏定律 (The Law of Leaky Abstractions)](#抽象泄漏定律-the-law-of-leaky-abstractions)

---

# 2、 切斯特顿围栏 (Chesterton's Fence)

〔切斯特顿围栏指出：如果你在路中间看到一堵看似多余的围栏，在你弄明白它为什么被建在那里之前，绝不要擅自拆除它。在重构遗留代码时切忌盲目删去看似多余的古怪检查。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Wikipedia:Chesterton%27s_fence)

> 在了解现有情况背后的原因之前，不应该进行改进。

该原则与软件工程中的消除技术负债 (Technical debt) 相关。程序的每一行最初都是出于某种原因编写的，因此根据切斯特森围栏原则，在更改或删除代码之前，即使看起来似乎是多余的或不正确的，也应该尝试完全理解代码的上下文和含义。

该原则的名字来源于 [G.K. Chesterson](https://en.wikipedia.org/wiki/G._K._Chesterton) 的一则故事。一个男人横穿马路中央的栅栏，他向市长抱怨这道栅栏没有用还挡路，并要求拆除它。市长问他为什么要在那里建栅栏，那个人回答说不知道。市长接着说：“如果你不知道它的用途，我肯定不会让你把它拆了。你去查查它的用途，之后我可能会允许你拆掉它。”

---

# 3、 柯克霍夫原则 (Kerckhoffs's Principle)

〔柯克霍夫原则申明：即使密码系统的所有设计细节与算法都被敌方所知，只要密钥没有泄露，该系统依然必须是安全的。拒绝“通过隐蔽求安全”（Security through obscurity）。〕

[Kerckhoffs's principle on Wikipedia](https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle)

> "...design your system assuming that your opponents know it in detail."
>
> _Steven M. Bellovin's formulation of Kerckhoff's Principle_

This principle of cryptography was an axiom created by cryptographer Auguste Kerckhoffs. He stated that a cryptosystem should be secure, even if everything about the system, except the key, is public knowledge. Not to be confused with [_"security through obscurity"_](#todo).

The gold standard for any secret-keeping system is that implementation details should be publicly distributed, without sacrificing or compromising security of said system.

The history of cryptography has shown that open discussion and analysis of cryptographic systems leads to better and more secure systems - as researchers are able to test for and expose potential vulnerabilities.

- [Shannon's Maxim](#todo)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

---

# 4、 死海效应 (The Dead Sea Effect)

〔死海效应描述：高水平的核心人才更容易找到更好的机会而离开糟糕的公司（水分蒸发），而平庸缺乏流动性的员工则会沉淀留任（高浓度盐分），最终导致整个组织的技术能力急剧退化。〕

- [Bruce F. Webster 的博客文章](http://brucefwebster.com/2008/04/11/the-wetware-crisis-the-dead-sea-effect/)

> "... 那些更有才华，更有效率的 IT 工程师最有可能离开——消失 ... （而那些倾向于）留下来的“剩下的人”——是最没有才华和效率的 IT 工程师。"
>
> _Bruce F. Webster_

死海效应表明，在任何一个组织中，工程师的技能、才华和效能往往与他们在公司的时间呈反比。

通常情况下，技术好的工程师很容易在其他的地方找到工作，并且他们往往也会这样做。而技能过时或技术薄弱的工程师则会留在公司，因为其他地方很难找到工作。如果这些工程师在公司里获得了加薪，他们会更愿意留在公司，因为在其他地方找到同等薪酬的工作会很有挑战性。

---

# 5、 呆伯特原则 (The Dilbert Principle)

〔呆伯特原则讽刺指出：企业倾向于系统性地将最无能的员工提拔到管理层，以便让他们离开真正产生价值的核心生产一线，从而尽量减少对实际业务的损害。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Dilbert_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%91%86%E4%BC%AF%E7%89%B9%E6%B3%95%E5%89%87)

> 公司会倾向于系统地将工作能力差的员工提升到管理层，以使他们脱离工作流程。
>
> _史考特·亚当斯 (Scott Adams)_

呆伯特原则是由史考特·亚当斯 (Dilbert 漫画连环画的创建者) 开发的一个管理概念，灵感来源于[彼得原理](#%e5%bd%bc%e5%be%97%e5%8e%9f%e7%90%86-the-peter-principle)。根据呆伯特原则，工作能力差的员工会被提升到管理层，从而限制他们所能造成的损害。亚当斯首先在 1995 年《华尔街日报》的一篇文章中解释了这一原则，随后在他 1996 年的商业书籍《呆伯特原则》中进行了扩展。

参见：

- [The Peter Principle](#the-peter-principle)
- [普特定律](#%e6%99%ae%e7%89%b9%e5%ae%9a%e5%be%8b-putts-law)

---

# 6、 帕累托法则 / 80-20 法则 (The Pareto Principle)

〔帕累托法则揭示：在大多数情境中，大约80%的结果由20%的关键原因所导致。在软件工程中，80%的运行时耗通常集中在20%的代码上，80%的崩溃往往由20%的核心缺陷引起。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Pareto_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%B8%95%E7%B4%AF%E6%89%98%E6%B3%95%E5%88%99)

> 生活中大多数事情不是均匀分布的。

帕累托法则可以帮你认识到大多数结果来自少数投入：

- 某个软件的 80％ 代码只占了总分配时间的 20％（相反，最难的 20％ 代码部分占用了 80％ 的时间）
- 20％ 的努力产生了 80％ 的结果
- 20％ 的工作创造了 80％ 的收入
- 20％ 的错误导致了 80％ 的崩溃
- 20％ 的功能导致了 80％ 的使用量

在 20 世纪 40 年代，公认为质量控制之父的美国罗马尼亚工程师约瑟夫·朱兰博士，[开始将帕累托法则应用于质量问题](https://en.wikipedia.org/wiki/Joseph_M._Juran)。

这个原则也被称为**二八法则**，**重要的少数法则**和**因素稀疏原则**。

现实的例子：

- 微软 2002 年的报告表明，修复最常出现的 20％ 错误，将消除 Windows 和 Office 中 80％ 的
  错误和崩溃。[报告地址](https://www.crn.com/news/security/18821726/microsofts-ceo-80-20-rule-applies-to-bugs-not-just-features.htm)

---

# 7、 舍基原则 (The Shirky Principle)

〔舍基原则指出：致力于解决某个问题的机构，往往会产生延长该问题存在的内在倾向，因为该问题的彻底解决往往意味着该机构自身生存合法性的终结。〕

[舍基原理解释](https://kk.org/thetechnium/the-shirky-prin/)

> Institutions will try to preserve the problem to which they are the solution.
> 各机构会努力保留他们能够解决的问题。
>
> _克莱·舍基 (Clay Shirky)_

舍基原理表明，复杂的解决方案——一家公司、一个行业或一项技术——可能会过于专注于他们正在解决的问题，以至于在无意中使问题本身变得永久化。这可能是有意为之（公司努力为问题找到新的细微差别，以证明继续开发解决方案是合理的），也可能是无意之举（不愿意接受和构建一个完全解决或避免问题的解决方案）。

相关内容:

- “当一个人不理解自己的工作就能够获得酬劳时，那么他就很难再去了解这份工作了！”——厄普顿·辛克莱
- 《创新者的困境》克莱·克里斯滕森

参见:

- [帕累托法则 (The Pareto Principle or The 80/20 Rule)](#帕累托法则-the-pareto-principle-or-the-8020-rule)

---

# 8、 随机鹦鹉理论 (The Stochastic Parrot)

〔随机鹦鹉理论批判指出：大型语言模型本质上是通过统计概率拼接庞大训练语料库中的语言形式，缺乏对外部世界真实语义、意图及逻辑的真正深层理解。〕

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

---

# 9、 彼得原理 (The Peter Principle)

〔彼得原理断言：在层级组织中，每个员工都会趋于晋升到他所不能胜任的职位。优秀程序员往往被提拔为糟糕的技术经理，直到他在该职位上停滞不前。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Peter_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%BD%BC%E5%BE%97%E5%8E%9F%E7%90%86)

> 在等级制度中，人往往会被提升到他们的“无法胜任的水平”。
>
> _劳伦斯·彼得 (Laurence J. Peter)_

这是由劳伦斯·彼得提出的一个管理概念。彼得原理认为，擅长工作的人会得到提升，直到他们达到不再成功的水平 (即他们所“无法胜任的水平”)。基于此，由于他们资历更高，被公司开除的可能性较小 (除非他们表现非常糟糕)。而且他们将继续担任几乎没有本职技能的职位，即使那些原本让他们成功的能力在新工作中并无必要。

有的工程师对此特别感兴趣，它们最初从事的是深度的技术工作，但走上了**管理**其他工程师的职业道路——这意味着需要一个完全不同的技能树。

参见：

- [呆伯特法则](#%e5%91%86%e4%bc%af%e7%89%b9%e6%b3%95%e5%88%99)
- [普特定律](#%e6%99%ae%e7%89%b9%e5%ae%9a%e5%be%8b-putts-law)

---

# 10、 稳健原则 / 波斯塔尔法则 (The Robustness Principle or Postel's Law)

〔波斯塔尔法则奉行：“对自己严格，对他人宽容”（Be conservative in what you send, be liberal in what you accept）。发送符合严格标准的数据，宽容接纳非严格标准的外部输入。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Robustness_principle)

> 在自己所做的事情上要保守, 在接受别人的事情上要自由。

通常应用于服务器应用程序开发中，该原则指出，你发送给其他人的内容应尽可能最小且符合要求，并且处理不符合要求的输入。

该原则的目标是构建稳健的系统。如果可以理解意图，它们可以处理不良的输入。但是，接受错误格式的输入可能存在安全隐患，特别是此类的输入未经过充分测试。

---

# 11、 SOLID 软件设计五大原则总论 (SOLID Principles)

〔SOLID是由罗伯特·C·马丁整理的五个面向对象设计核心原则缩写：单一职责（S）、开闭原则（O）、里氏替换（L）、接口隔离（I）、依赖反转（D）。它们是现代可维护软件架构的奠基支柱。〕

这是一个缩写，指的是：

- S：[单一功能原则 (The Single Responsibility Principle)](#%E5%8D%95%E4%B8%80%E5%8A%9F%E8%83%BD%E5%8E%9F%E5%88%99-the-single-responsibility-principle)
- O：[开闭原则 (The Open/Closed Principle)](#%E5%BC%80%E9%97%AD%E5%8E%9F%E5%88%99-the-openclosed-principle)
- L：[里氏替换原则 (The Liskov Substitution Principle)](#%E9%87%8C%E6%B0%8F%E6%9B%BF%E6%8D%A2%E5%8E%9F%E5%88%99-the-liskov-substitution-principle)
- I：[接口隔离原则 (The Interface Segregation Principle)](#%E6%8E%A5%E5%8F%A3%E9%9A%94%E7%A6%BB%E5%8E%9F%E5%88%99-the-interface-segregation-principle)
- D：[依赖反转原则 (The Dependency Inversion Principle)](#%E4%BE%9D%E8%B5%96%E5%8F%8D%E8%BD%AC%E5%8E%9F%E5%88%99-the-dependency-inversion-principle)

这些是 [Object-Oriented Programming](#todo) 的关键原则。诸如此类的设计原则能够帮助开发人员构建更易于维护的系统。

---

# 12、 单一职责原则 (The Single Responsibility Principle / SRP)

〔单一职责原则强调：一个模块或类应该且仅应该对某一类利益相关者（或某一个变更原因）负责。引起它变化的原因有且只有一个。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Single_responsibility_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%8D%95%E4%B8%80%E5%8A%9F%E8%83%BD%E5%8E%9F%E5%88%99)

> 每个模块或者类只应该有一项功能。

[SOLID](#solid) 的第一个原则。这个原则表明模块或者类只应该做一件事。实际上，这意味着对程序功能的单个小更改，应该只需要更改一个组件。例如，更改密码验证复杂性的方式应该只需要更改程序的一部分。

理论上讲，这使代码更健壮，更容易更改。知道正在更改的组件只有一个功能，这意味着测试更改更容易。使用前面的例子，更改密码复杂性组件应该只影响与密码复杂性相关的功能。变更具有许多功能的组件可能要困难得多。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)

---

# 13、 开闭原则 (The Open/Closed Principle / OCP)

〔开闭原则主张：软件实体（类、模块、函数等）应当对扩展开放，对修改关闭。当系统引入新需求时，应通过新增代码而非修改既有经测试代码来实现。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E5%BC%80%E9%97%AD%E5%8E%9F%E5%88%99)

> 实体应开放扩展并关闭修改。

[SOLID](#solid) 的第二个原则。这个原则指出实体（可以是类、模块、函数等）应该能够使它们的行为易于扩展，但是它们的扩展行为不应该被修改。

举一个假设的例子，想象一个能够将 Markdown 转换为 HTML 的模块。如果可以扩展模块，而不修改内部模块来处理新的 markdown 特征，而无需修改内部模块，则可以认为是开放扩展。如果用户不能修改处理现有 Markdown 特征的模块，那么它被认为是关闭修改。

这个原则与面向对象编程紧密相关，让我们可以设计对象以便于扩展，但是可以避免以意想不到的方式改变其现有对象的行为。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)

---

# 14、 里氏替换原则 (The Liskov Substitution Principle / LSP)

〔里氏替换原则指出：子类型必须能够完全替换掉它们的基类型，而不会改变程序的正确性与预期行为。继承不应破坏基类契约约束。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Liskov_substitution_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E9%87%8C%E6%B0%8F%E6%9B%BF%E6%8D%A2%E5%8E%9F%E5%88%99)

> 可以在不破坏系统的情况下，用子类型替换类型。

[SOLID](#solid) 的第三个原则。该原则指出，如果组件依赖于类型，那么它应该能够使用该类型的子类型，而不会导致系统失败或者必须知道该子类型的详细信息。

举个例子，假设我们有一个方法，读取 XML 文档。如果该方法使用基类型 **file**，则从 **file** 派生的任何内容，都能用在该方法中。 如果 **file** 支持反向查找，并且 xml 解析器使用该函数，但是派生类型 **network file** 尝试反向查找时失败，则 **network file** 将违反该原则。

该原则与面向对象编程紧密相关，必须仔细建模、层次结构，以避免让系统用户混淆。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)

---

# 15、 接口隔离原则 (The Interface Segregation Principle / ISP)

〔接口隔离原则要求：客户端不应该被迫依赖于它们不使用的方法。宁可定义多个专门细粒度的瘦接口，也不要定义单一臃肿的万能胖接口。〕

- [英文在线地址](http://www.hyrumslaw.com/)

> 当 API 有足够多的用户时，你在合同中的承诺已不重要：你系统的所有可观察行为都将被某些人所依赖。
>
> _海勒姆·赖特 (Hyrum Wright)_

隐式接口定律表明，当你的 API 有足够多的用户时，API 的所有行为（包括那些未囊括在公共说明中的一部分）最终都会被其他人所依赖。 一个简单的例子是 API 的响应时间这种非功能性因素；还有一个更微妙的例子是：用户使用正则表达式匹配错误提示来判断 API 的错误类型，即使 API 文档中没有任何关于错误提示的内容，而是指导用户应该使用相应的错误代码。一些用户依然会使用错误提示内容（而非错误代码），这种情况下变更 API 错误提示信息，实际上会破坏 API 的使用。

参见：

- [抽象泄漏定律](#%E6%8A%BD%E8%B1%A1%E6%B3%84%E6%BC%8F%E5%AE%9A%E5%BE%8B-the-law-of-leaky-abstractions)
- [XKCD 1172](https://xkcd.com/1172/)

---

# 16、 依赖倒置原则 (The Dependency Inversion Principle / DIP)

〔依赖倒置原则主张：高层模块不应该依赖低层模块，二者都应该依赖于抽象；抽象不应该依赖于细节，细节应该依赖于抽象。〕

- [英文维基百科](https://en.wikipedia.org/wiki/Dependency_inversion_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/%E4%BE%9D%E8%B5%96%E5%8F%8D%E8%BD%AC%E5%8E%9F%E5%88%99)

> 高级模块不应该依赖于低级实现。

[SOLID](#solid) 的第五个原则。该原则指出，更高级别的协调组件不应该知道其依赖项的详细信息。

举个例子，假设我们有一个从网站读取元数据的程序。我们假设主要组件必须知道下载网页内容的组件，以及可以读取元数据的组件。如果我们考虑依赖反转，主要组件将仅依赖于可以获取字节数据的抽象组件，然后是一个能够从字节流中读取元数据的抽象组件，主要组件不需要了解 TCP、IP、HTTP、HTML 等。

这个原则很复杂，因为它似乎可以反转系统的预期依赖性（因此得名）。实践中，这也意味着，单独的编排组件必须确保抽象类型的正确实现被使用（例如在前面的例子中，必须提供元数据读取器组件、HTTP 文件下载功能和 HTML 元标签读取器）。然后，这涉及诸如 [Inversion of Control](#todo) 和 [Dependency Injection](#todo) 之类的模式。

参见：

- [Object-Orientated Programming](#todo)
- [SOLID](#solid)
- [Inversion of Control](#todo)
- [Dependency Injection](#todo)

---

# 17、 DRY 原则 / 不要重复自己 (The DRY Principle)

〔DRY原则倡导：系统中的每一项知识或逻辑都必须在系统中具有单一、明确、权威且不可歧义的表述形式，杜绝多处复制粘贴导致的一致性维护噩梦。〕

[The DRY Principle on Wikipedia](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)

> Every piece of knowledge must have a single, unambiguous, authoritative representation within a system.

DRY is an acronym for _Don't Repeat Yourself_. This principle aims to help developers reducing the repetition of code and keep the information in a single place and was cited in 1999 by Andrew Hunt and Dave Thomas in the book [The Pragmatic Programmer](https://en.wikipedia.org/wiki/The_Pragmatic_Programmer)

> The opposite of DRY would be _WET_ (Write Everything Twice or We Enjoy Typing).

In practice, if you have the same piece of information in two (or more) different places, you can use DRY to merge them into a single one and reuse it wherever you want/need.

See also:

- [The Pragmatic Programmer](https://en.wikipedia.org/wiki/The_Pragmatic_Programmer)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

---

# 18、 KISS 原则 / 保持简单愚蠢 (The KISS Principle)

〔KISS原则指出：绝大多数系统在保持极简而非变得复杂时才能运转得最好。简单不仅是一种审美，更是系统可靠性、可调试性与长期生存能力的关键保障。〕

- [英文维基百科](https://en.wikipedia.org/wiki/KISS_principle)
- [中文维基百科](https://zh.wikipedia.org/wiki/KISS%E5%8E%9F%E5%88%99)

> 保持简单和直白。

KISS 原则指明了如果大多数的系统能够保持简单而非复杂化，那么他们便能够工作在最佳状态。因此，简单性应该是设计时的关键指标，同时也要避免不必要的复杂度。这个短语最初出自 1960 年的美国海军飞机工程师凯利 · 约翰逊 (Kelly Johnson)。

这一原则的最好例证便是约翰逊给设计工程师一些实用工具的故事。那时的他们正面临着一个挑战，即他们参与设计的喷气式飞机必须能够让普通的机械师在战场上仅仅用这些工具进行维修，因此，“直白”这个词应指的是损坏的事物本身和修复用工具的复杂度两者之间的关系，而非工程师们自身的能力水平。

参见：

- [盖尔定律](#%e7%9b%96%e5%b0%94%e5%ae%9a%e5%be%8b-galls-law)

---

# 19、 YAGNI 原则 / 你不会需要它 (You Aren't Gonna Need It)

〔YAGNI原则告诫：始终只实现你当前严格需要的功能，绝不要去实现你自以为将来可能需要的功能。过度未雨绸缪的抽象只会带来巨大的无效维护成本。〕

- [英文维基百科](https://en.wikipedia.org/wiki/You_aren%27t_gonna_need_it)

这是 _**Y**ou **A**ren't **G**onna **N**eed **I**t_ 的缩写。

> 只有当你需要某些东西的时候，才去实现它们，而不是在你预见的时候。
>
> [Ron Jeffries](https://twitter.com/RonJeffries) 是极限编程的创始人之一以及书籍《Extreme Programming Installed》的作者。

极限编程原则告诫开发人员，他们应该只实现当前所需的功能，并避免实现未来需要的功能，仅在必要时才实现。

遵守这一原则可以减小代码库大小，同时避免时间和生产力浪费在没有价值的功能上。

参见：

- [阅读清单《极限编程安装》](#%E9%98%85%E8%AF%BB%E6%B8%85%E5%8D%95)

---

# 20、 分布式计算的八大谬论 (The Fallacies of Distributed Computing)

〔分布式计算的八大谬论揭示了初学者在分布式系统中普遍持有的8个致命错误假定：网络可靠、延迟为零、带宽无限、网络安全、拓扑恒定、只有一名管理员、传输开销为零、网络同质。〕

[英文维基百科](https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing)

又称 _网络计算的谬误_，这是一系列关于分布式计算的猜想（或者看法），这些猜想可能会引起软件开发中的失败。这些假设是：

- 网络可靠
- 延迟为零
- 带宽无限
- 网络安全
- 拓扑恒定
- 单一管理员
- 运输成本为零
- 网络为同构的

前 4 个项目由 [Bill Joy](https://en.wikipedia.org/wiki/Bill_Joy) 和 [Tom Lyon](https://twitter.com/aka_pugs) 于 1991 左右提出。并被 [James Gosling](https://en.wikipedia.org/wiki/James_Gosling) 首次归类于“网络计算的谬误”；后 [L. Peter Deutsch](https://en.wikipedia.org/wiki/L._Peter_Deutsch) 添加了第 5、6、7 个谬误；90 年代末，Gosling 添加了最后一个谬误。

这些内容受到了 [太阳微系统 (Sun Microsystems)](https://en.wikipedia.org/wiki/Sun_Microsystems) 内部当时所发生的事情的启发。

在设计弹性代码的时候，应该仔细考虑这些谬误，并假设其中任何一个谬误都可能引起处理分布式系统的复杂性和现实性时的逻辑缺陷。

参见:

- [寻找分布式计算的谬误（第一部分） - Vaidehi Joshion Medium](https://medium.com/baseds/foraging-for-the-fallacies-of-distributed-computing-part-1-1b35c3b85b53)

---

# 21、 最小惊奇原则 (The Principle of Least Astonishment / POLA)

〔最小惊奇原则申明：在设计组件、接口或用户交互时，其行为应当符合用户的普遍心理预期，绝不应该展现出令经验丰富的使用者感到不可思议或惊愕的突兀怪异行为。〕

[The Principle of Least Astonishment on Wikipedia](https://en.wikipedia.org/wiki/Principle_of_least_astonishment)

> People are part of the system. The design should match the user's experience, expectations, and mental models.
>
> Frans Kaashoek

This principle proposes that systems and interfaces should be designed in a way that features and functionality is easily discovered and matches users expectations. Features that 'surprise' users should be discouraged in favour of features that can be intuitively reasoned about based on existing patterns and practices.

Many examples are present in user interfaces, such as a 'pull down' gesture on a mobile appliation to refresh content. Another example would be command line tools, where many standards exist for how parameters are named, common parameters that should be available and so on.

See also:

- [Convention Over Configuration](#todo)

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。

---
