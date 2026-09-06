---
title: "引用文献: Principle of least astonishment (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Principle_of_least_astonishment"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《The Principle of Least Astonishment on Wikipedia》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Principle of least astonishment (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Principle_of_least_astonishment](https://en.wikipedia.org/wiki/Principle_of_least_astonishment)
- **引用锚文本**: The Principle of Least Astonishment on Wikipedia
- **抓取状态**: success

# 二、 文献正文内容

In user interface design and software design, 
the principle of least astonishment (POLA), also known as principle of least surprise (POLS), proposes that a component of a system should behave in a way that most users will expect it to behave, and therefore not astonish or surprise users. The following is a corollary of the principle: "If a necessary feature has a high astonishment factor, it may be necessary to redesign the feature."
The principle has been in use in relation to computer interaction since at least the 1970s. Although first formalized in the field of computer technology, the principle can be applied broadly in other fields. For example, in writing, a cross-reference to another part of the work or a hyperlink should be phrased in a way that accurately tells the reader what to expect.


== Origin ==
An early reference to the "Law of Least Astonishment" appeared in the PL/I Bulletin in 1967 (PL/I is a programming language released by IBM in 1966).  By the late 1960s, PL/I had become infamous for violating this law because, due to PL/I's precision conversion rules, the expressions 25 + 1/3 and 1/3 + 25 would either produce a fatal error, or, if errors were suppressed, 5.33333333333 instead of the correct 25.33333333333.
The law first appeared in print in 1972:

For those parts of the system which cannot be adjusted to the peculiarities of the user, the designers of a systems programming language should obey the "Law of Least Astonishment." In short, this law states that every construct in the system should behave exactly as its syntax suggests. Widely accepted conventions should be followed whenever possible, and exceptions to previously established rules of the language should be minimal.


== Formulation ==
A textbook formulation is: "People are part of the system. The design should match the user's experience, expectations, and mental models."
The principle aims to leverage the existing knowledge of users to minimize the learning curve, for instance by designing interfaces that borrow heavily from "functionally similar or analogous programs with which your users are likely to be familiar". User expectations in this respect may be closely related to a particular computing platform or tradition. For example, Unix command line programs are expected to follow certain conventions with respect to switches, and widgets of Microsoft Windows programs are expected to follow certain conventions with respect to keyboard shortcuts. In more abstract settings like an API, the expectation that function or method names intuitively match their behavior is another example. This practice also involves the application of sensible defaults.
When two elements of an interface conflict, or are ambiguous, the behavior should be that which will least surprise the user; in particular a programmer should try to think of the behavior that will least surprise someone who uses the program, rather than that behavior that is natural from knowing the inner workings of the program.
The choice of "least surprising" behavior can depend on the expected audience (for example, end users, programmers, or system administrators).


== Examples ==
Websites offering keyboard shortcuts often allow pressing ? to see the available shortcuts.  Examples include Gmail, YouTube, and Jira.
In Windows operating systems and some desktop environments for Linux, the F1 function key typically opens the help program for an application.  A similar keyboard shortcut in macOS is ⌘ Command+⇧ Shift+/. Users expect a help window or context menu when they press the usual help shortcut key(s). Software that instead uses this shortcut for another feature is likely to cause astonishment if no help appears.
A programming language's standard library usually provides a function similar to the pseudocode ParseInteger(string, radix), which creates a machine-readable integer from a string of human-readable digits. The radix conventionally defaults to 10, meaning the string is interpreted as decimal (base 10). This function usually supports other bases, like binary (base 2) and octal (base 8), but only when they are specified explicitly. In a departure from this convention, JavaScript originally defaulted to base 8 for strings beginning with "0", causing developer confusion and software bugs. This was discouraged in ECMAScript 3 and dropped in ECMAScript 5.
Some development communities like FreeBSD use POLA as one of the guidelines for what makes an unsurprising user experience.


== See also ==
DWIM (do what I mean)
Convention over configuration
Human interface guidelines
Look and feel
Occam's razor
WYSIWYG
List of software development philosophies
User experience design


== Notes ==


== References ==


== External links ==
Principle of Least Astonishment at Portland Pattern Repository

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Principle_of_least_astonishment_
