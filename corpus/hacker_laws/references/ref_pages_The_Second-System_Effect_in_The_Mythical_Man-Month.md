---
title: "引用文献: The Mythical Man-Month"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://pages.cs.wisc.edu/~param/quotes/man-month.html"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
---

〔本文档为黑客定律与工程哲学文库中《The Second-System Effect in _The Mythical Man-Month_》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: The Mythical Man-Month
- **原文链接**: [https://pages.cs.wisc.edu/~param/quotes/man-month.html](https://pages.cs.wisc.edu/~param/quotes/man-month.html)
- **引用锚文本**: The Second-System Effect in _The Mythical Man-Month_
- **抓取状态**: success

# 二、 文献正文内容

The Mythical Man-Month
Frederic P. Brooks
Why is programming Fun?
Why is programming fun? What delights may its
practitioner expect as his reward?
First is the sheer joy of making things. As the child
delights in his mud pie, so the adult enjoys building
things, especially things of his own design. I think this
delight must be an image of God's delight in making things,
a delight shown in the distinctness and newness of each
leaf and each snowflake.
Second is the pleasure of making things that are useful
to other people.  Deep within, we want others to use our
work and find it helpful. in this respect, the programming
system is not essentially different from the child's first
clay pencil holder "for Daddy's office."
Third is the fascination of fashioning complex
puzzle-like objects of interlocking moving parts and
watching them work in subtle cycles, playing out the
consequences of the principles built in from the beginning.
The programmed computer has all the fasciantion of the
pinball machine or the jukebox mechanism carried to the
ultimate.
Fourth is the joy of always learning, which springs from
the nonrepeating nature of the task. In one way or another,
the problem is always new, and its solver learns something:
sometimes practical, sometimes theoretical, and sometmes
both.
Finally there is the delight of working in such a
tractable medium.  The programmer, like the poet, works
only slightly removed from pure thought-stuff.  He builds
his castles in the air, from air, creating by exertion of
the imagination. Few media of creation are so flexible, so
easy to polish and rework, so readily capable of realizing
grand conceptual structures.
Yet the program construct, unlike the poet's words, is
real in the sense that in moves and works, producing
visible outputs seperate from the construct itself. It
prints results, draws pictures, produces sounds, moves
arms.  The magic of myth and legend has come true in our
time. One types the correct incantation on the keyboard,
and a display screen comes to life, showing things that
never were nor could be.
Programming then is fun because it gratifies creative
longings built deep within us and delights sensibilities we
have in common with all men.
Self-Discipline: The Second System Effect
An architect's first work is apt to be spare and clean. He
knows he dosen't know what he's doing, so he does it carefully
and with great restraint.
As he designs his first work, frill after frill,
embellishment after embillishment occour to him. These get
stored away to be used "next time." Sooner or later the first
system is finished, with firm confidence and a demonstrated
mastery of that class of systems, is ready to build a second
system.
This second is the most dangerous system a man ever designs.
...The general tendecy is to over-design the second system,
using all the ideas and frills that were cautiously sidetracked
on the first one. For example, consider the IMB 709
architecture, later embodied in the 7090. this is an upgrade, a
second system for the very successful and clean 704. The
operation set is so rich and profuse that only about half of it
was regularly used.
…
The second-system effect has another manifestation somewhat
different from pure functional embellishment. that is a
tendency to refine techniques whose very existence has been
made obsolete by changes in basic system assumptions. OS/360
has many examples of this.
Consider the linkage editor, designed to load
separately-compiled programs and resolve their cross references.
Beyond this basic function it also handles program overlays. It
is one of the finest overlay facilities ever built.  It allows
overlay structuring to be done externally, at linkage time,
without being designed into the source code. It allows the
overlay structure to be changes from run to run without
recompilation. It furnished a rich variety of useful options
and facilities. In a sense it is the culmination of years of
development of static overlay technique.
Yet it is also the last and finest of the dinosaurs, for it
belongs to a system in which multiprogramming is the normal
mode and dynamic core allocation the basic assumption. this is
in direct conflict with the notion of using static overlays.
how much better the system would work if if the efforts devoted
to overlay management had been spent on making the dynamic core
allocation and the dynamic cross-referencing facilities really
fast!
Home
|
FAQ
|
Photos
|
Misc
|
REC
Updated on 11 August 1999
XHTML-1.0
CSS

---
_本地归档时间: 2026-09-05 | 来源: https://pages.cs.wisc.edu/~param/quotes/man-month.html_
