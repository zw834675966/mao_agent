---
title: "引用文献: Wadler's Law - HaskellWiki"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://wiki.haskell.org/Wadler's_Law"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文在线地址》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Wadler's Law - HaskellWiki
- **原文链接**: [https://wiki.haskell.org/Wadler's_Law](https://wiki.haskell.org/Wadler's_Law)
- **引用锚文本**: 英文在线地址
- **抓取状态**: success

# 二、 文献正文内容

Jump to content
HaskellWiki
Search
Search
Log in
Personal tools
Log in
Wadler's Law
From HaskellWiki
Wadler's Law states that:
In any language design, the total time spent discussing
a feature in this list is proportional to two raised to
the power of its position.
0. Semantics
1. Syntax
2. Lexical syntax
3. Lexical syntax of comments
The origins of this law are found in two emails:
Haskell mailing list, 1992
Found here
Literate comments
Date: Wed, 5 Feb 1992 09:14:00 +0000
From: Philip Wadler <wadler@dcs.glasgow.ac.uk>
Sender: haskell-request@dcs.glasgow.ac.uk
To: haskell@dcs.glasgow.ac.uk
Cc: hudak@yale.edu, simonpj@dcs.glasgow.ac.uk, wadler@dcs.glasgow.ac.uk
Subject: Literate comments
Despite Paul having ruled on the matter, the debate still rages, with
about half saying that > is wonderful, and half saying it is awful.
Under these circumstances, Paul's ruling seems right: it is pointless
to legislate > as part of the language.
On the other hand, the current circumstance is that the main
implementations do all use the > convention.  It seems reasonable to
say this in the report -- we should at least tell other implementors
what the current implementors have settled upon.  This would also let
Joe use the convention in presenting the standard prelude, as he says
he would like to do.
To be precise: I propose an additional chapter of the report, labeled
`Literate comments' and no more than one page long, that states a
convention for providing literate comments, notes that it is NOT part
of Haskell but is supported by existing implementations, and mentions
that the Prelude is presented with this convention in the report.  I
volunteer to draft the page.
Paul, as syntax honcho you should rule on this.  Cheers,  -- P
PS:  Wadler's Law:
The emotional intensity of debate on a language feature
increases as one moves down the following scale:
Semantics,
Syntax,
Lexical syntax,
Comments.
Revised law, Thu, 19 Dec 1996
Found here
From: Philip Wadler <wadler_at_research.bell-labs.com>
Date: Thu, 19 Dec 1996 10:52:48 -0500
> > Good point, and well worth stressing. Nonetheless, it still seems
> > quite reasonable to say that Curry should contain all Haskell
> > that eschew type classes and non-uniform pattern matching.
>
> I think this is a good remark and I can agree to it.
I'm delighted to hear this.
> Concerning uppercase/lowercase: of course, we can or should
> use Haskell's *convention*, but I do not like to enforce it, ...
Oh dear. I don't think we've gotten our point across.
Yes, there are problems with enforcing a capitalisation convention.
There are also problems with not enforcing one. Nothing unique to
Curry here. The Haskell committee spent a lot of time hashing
over this ground. I strongly urge you to leave this can of worms
shut. Adopt the Haskell solution wherever possible, warts and all!
You might be interested in the following scientific law, which
has been verified by extensive emperical observation.
WADLER'S LAW OF LANGUAGE DESIGN
In any language design, the total time spent discussing
a feature in this list is proportional to two raised to
the power of its position.
0. Semantics
1. Syntax
2. Lexical syntax
3. Lexical syntax of comments
(That is, twice as much time is spent discussing syntax
than semantics, twice as much time is spent discussing
lexical syntax than syntax, and twice as much time is
spent discussing syntax of comments than lexical syntax.)
With the exception of comments, I would say discussion so far
conforms to this observation. -- P
Retrieved from "https://wiki.haskell.org/index.php?title=Wadler%27s_Law&oldid=62262"
Category:
Humor
Search
Search
Wadler's Law
Add topic

---
_本地归档时间: 2026-09-05 | 来源: https://wiki.haskell.org/Wadler's_Law_
