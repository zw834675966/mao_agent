---
title: "引用文献: Unix philosophy (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Unix_philosophy"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Unix philosophy (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Unix_philosophy](https://en.wikipedia.org/wiki/Unix_philosophy)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

The Unix philosophy, originated by Ken Thompson, is a set of cultural norms and philosophical approaches to minimalist, modular software development. It is based on the experience of leading developers of the Unix operating system. Early Unix developers were important in bringing the concepts of modularity and reusability into software engineering practice, spawning a "software tools" movement. Over time, the leading developers of Unix (and programs that ran on it) established a set of cultural norms for developing software; these norms became as important and influential as the technology of Unix itself, and have been termed the "Unix philosophy."
The Unix philosophy emphasizes building simple, compact, clear, modular, and extensible code that can be easily maintained and repurposed by developers other than its creators. The Unix philosophy favors composability as opposed to monolithic design.


== Origin ==
In their Unix paper of 1974, Ritchie and Thompson quote the following design considerations:

Make it easy to write, test, and run programs.
Interactive use instead of batch processing.
Economy and elegance of design due to size constraints ("salvation through suffering").
Self-supporting system: all Unix software is maintained under Unix.
In 1978, Doug McIlroy documented a set of principles encapsulating the "characteristic style" that had emerged among Unix system users and developers:

Make each program do one thing well. To do a new job, build afresh rather than complicate old programs by adding new "features".
Expect the output of every program to become the input to another, as yet unknown, program. Don't clutter output with extraneous information. Avoid stringently columnar or binary input formats. Don't insist on interactive input.
Design and build software, even operating systems, to be tried early, ideally within weeks. Don't hesitate to throw away the clumsy parts and rebuild them.
Use tools in preference to unskilled help to lighten a programming task, even if you have to detour to build the tools and expect to throw some of them out after you've finished using them.
Later, in 1994, it was summarized with the explicit name "The Unix philosophy" by Peter H. Salus, who credited it to McIlroy:

Write programs that do one thing and do it well.
Write programs to work together.
Write programs to handle text streams, because that is a universal interface.


== Parts ==


=== The UNIX Programming Environment ===
In their preface to the 1984 book, The UNIX Programming Environment, Brian Kernighan and Rob Pike, both from Bell Labs, give a brief description of the Unix design and the Unix philosophy:

Even though the UNIX system introduces a number of innovative programs and techniques, no single program or idea makes it work well. Instead, what makes it effective is the approach to programming, a philosophy of using the computer. Although that philosophy can't be written down in a single sentence, at its heart is the idea that the power of a system comes more from the relationships among programs than from the programs themselves. Many UNIX programs do quite trivial things in isolation, but, combined with other programs, become general and useful tools.
The authors further write that their goal for this book is "to communicate the UNIX programming philosophy."


=== Program Design in the UNIX Environment ===

In October 1984, Brian Kernighan and Rob Pike published a paper called Program Design in the UNIX Environment. In this paper, they criticize the accretion of program options and features found in some newer Unix systems such as 4.2BSD and System V, and explain the Unix philosophy of software tools, each performing one general function:

Much of the power of the UNIX operating system comes from a style of program design that makes programs easy to use and, more important, easy to combine with other programs. This style has been called the use of software tools, and depends more on how the programs fit into the programming environment and how they can be used with other programs than on how they are designed internally. [...] This style was based on the use of tools: using programs separately or in combination to get a job done, rather than doing it by hand, by monolithic self-sufficient subsystems, or by special-purpose, one-time programs.
The authors contrast Unix tools such as cat with larger program suites used by other systems.

The design of cat is typical of most UNIX programs: it implements one simple but general function that can be used in many different applications (including many not envisioned by the original author). Other commands are used for other functions. For example, there are separate commands for file system tasks like renaming files, deleting them, or telling how big they are. Other systems instead lump these into a single "file system" command with an internal structure and command language of its own. (The PIP file copy program found on operating systems like CP/M or RSX-11 is an example.) That approach is not necessarily worse or better, but it is certainly against the UNIX philosophy.


=== Doug McIlroy on Unix programming ===

McIlroy, then head of the Bell Labs Computing Sciences Research Center, and inventor of the Unix pipe, summarized the Unix philosophy as follows:

This is the Unix philosophy: Write programs that do one thing and do it well. Write programs to work together. Write programs to handle text streams, because that is a universal interface.
Beyond these statements, he has also emphasized simplicity and minimalism in Unix programming:

The notion of "intricate and beautiful complexities" is almost an oxymoron. Unix programmers vie with each other for "simple and beautiful" honors — a point that's implicit in these rules, but is well worth making overt.
Conversely, McIlroy has criticized modern Linux as having software bloat, remarking that, "adoring admirers have fed Linux goodies to a disheartening state of obesity." He contrasts this with the earlier approach taken at Bell Labs when developing and revising Research Unix:

Everything was small... and my heart sinks for Linux when I see the size of it. [...] The manual page, which really used to be a manual page, is now a small volume, with a thousand options... We used to sit around in the Unix Room saying, 'What can we throw out? Why is there this option?' It's often because there is some deficiency in the basic design — you didn't really hit the right design point. Instead of adding an option, think about what was forcing you to add that option.


=== Do One Thing and Do It Well ===
As stated by McIlroy, and generally accepted throughout the Unix community, Unix programs have always been expected to follow the concept of DOTADIW, or "Do One Thing And Do It Well." There are limited sources for the acronym DOTADIW on the Internet, but it is discussed at length during the development and packaging of new operating systems, especially in the Linux community.
Patrick Volkerding, the project lead of Slackware Linux, invoked this design principle in a criticism of the systemd architecture, stating that, "attempting to control services, sockets, devices, mounts, etc., all within one daemon flies in the face of the Unix concept of doing one thing and doing it well."


=== Eric Raymond's 17 Unix Rules ===
In his book The Art of Unix Programming that was first published in 2003, Eric S. Raymond (open source advocate and programmer) summarizes the Unix philosophy as "Keep it Simple, Stupid."  He provides a series of design rules:

Build modular programs
Write readable programs
Use composition
Separate mechanisms from policy
Write simple programs
Write small programs
Write transparent programs
Write robust programs
Make data complicated when required, not the program
Build on potential users' expected knowledge
Avoid unnecessary output
Write programs which fail in a way that is easy to diagnose
Value developer time over machine time
Write abstract programs that generate code instead of writing code by hand
Prototype software before polishing it
Write flexible and open programs
Make the program and protocols extensible.


=== Mike Gancarz: The UNIX Philosophy ===
In 1994, Mike Gancarz, a member of Digital Equipment Corporation's Unix Engineering Group (UEG), published The UNIX Philosophy based on his own Unix (Ultrix) port development at DEC in the 1980s and discussions with colleagues. He is also a member of the X Window System development team and author of Ultrix Window Manager (uwm). 
The book focuses on porting UNIX to different computers during the Unix wars of the 1980s and describes his philosophy that portability should be more important than the efficiency of using non-standard interfaces for hardware and graphics devices.
The nine basic "tenets" he claims to be important are

Small is beautiful.
Make each program do one thing well.
Build a prototype as soon as possible.
Choose portability over efficiency.
Store data in flat text files.
Use software leverage to your advantage.
Use shell scripts to increase leverage and portability.
Avoid captive user interfaces.
Make every program a filter.


=== "Worse is better" ===

Richard P. Gabriel suggests that a key advantage of Unix was that it embodied a design philosophy he termed "worse is better", in which simplicity of both the interface and the implementation are more important than any other attributes of the system—including correctness, consistency, and completeness. Gabriel argues that this design style has key evolutionary advantages, though he questions the quality of some results.
For example, in the early days Unix used a monolithic kernel (which means that user processes carried out kernel system calls all on the user stack). If a signal was delivered to a process while it was blocked on a long-term I/O in the kernel, the handling of the situation was unclear. The signal handler could not be executed when the process was in kernel mode, with sensitive kernel data on the stack.


== Criticism ==
In a 1981 article entitled "The truth about Unix: The user interface is horrid" published in Datamation, Don Norman criticized the design philosophy of Unix for its lack of concern for the user interface. Writing from his background in cognitive science and from the perspective of the then-current philosophy of cognitive engineering, he focused on how end-users comprehend and form a personal cognitive model of systems—or, in the case of Unix, fail to understand, with the result that disastrous mistakes (such as losing an hour's worth of work) are all too easy.
In the podcast On the Metal, game developer Jonathan Blow criticised UNIX philosophy as being outdated. He argued that tying together modular tools results in very inefficient programs. He says that UNIX philosophy suffers from similar problems to microservices: without overall supervision, big architectures end up ineffective and inefficient.


== See also ==
Cognitive engineering
Unix architecture
Minimalism (computing)
Software engineering
KISS principle
Hacker ethic
List of software development philosophies
Everything is a file
Worse is better


== Notes ==


== References ==
The Unix Programming EnvironmentArchived 2011-10-21 at the Wayback Machine by Brian Kernighan and Rob Pike, 1984
Program Design in the UNIX Environment – The paper by Pike and Kernighan that preceded the book.
Notes on Programming in C, Rob Pike, September 21, 1989
A Quarter Century of Unix, Peter H. Salus, Addison-Wesley, May 31, 1994 (ISBN 0-201-54777-5)
Philosophy Archived 2008-05-12 at the Wayback Machine — from The Art of Unix Programming, Eric S. Raymond, Addison-Wesley, September 17, 2003 (ISBN 0-13-142901-9)
Final Report of the Multics Kernel Design Project by M. D. Schroeder, D. D. Clark, J. H. Saltzer, and D. H. Wells, 1977.
The UNIX Philosophy, Mike Gancarz, ISBN 1-55558-123-4


== External links ==
Basics of the Unix Philosophy – by Catb.org
The Unix Philosophy: A Brief Introduction – by The Linux Information Project (LINFO)
Why the Unix Philosophy still matters
Fast food stand adopts the "UNIX philosophy"

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Unix_philosophy_
