---
title: "经典论文导读: Memory Management"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "Memory Management"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
  - "Papers We Love"
  - "计算机经典论文"
  - "Memory Management"
---

〔本篇为 Papers We Love 经典学术文献库关于“Memory Management”领域收录的传世奠基论文全景导读与本地文献档案。〕

# 一、 领域学术导读与背景

## Memory Management

* [A Unified Theory of Garbage Collection](https://web.eecs.umich.edu/~weimerw/2012-4610/reading/bacon-garbage.pdf)

* [Teaching Garbage Collection without Implementing Compilers or Interpreters](https://cs.brown.edu/~sk/Publications/Papers/Published/cgkmf-teach-gc/paper.pdf)

* [Message Analysis Guided Allocation and Low Pause Incremental GC in a Concurrent Language](http://user.it.uu.se/~kostis/Papers/ismm04.pdf)

* [And Then There Were None: A Stall-Free Real-Time Garbage Collector for Reconfigurable Hardware](https://cacm.acm.org/magazines/2013/12/169948-and-then-there-were-none/fulltext)

* [The Slab Allocator: An Object-Caching Kernel Memory Allocator](https://www.usenix.org/legacy/publications/library/proceedings/bos94/bonwick.html)
  - [ASCII version](https://www.usenix.org/legacy/publications/library/proceedings/bos94/full_papers/bonwick.a)
  - [POSTSCRIPT version](https://www.usenix.org/legacy/publications/library/proceedings/bos94/full_papers/bonwick.ps)

* :scroll: [ScatterAlloc: Massively Parallel Dynamic Memory Allocation for the GPU](https://markussteinberger.net/papers/ScatterAlloc.pdf)

  Presents a useful algorithm as well as considerations relevant to designing algorithms for GPUs.


* [:scroll:](making-lockless-synchronization-fast.pdf) [Making Lockless Synchronization Fast: Performance Implications of Memory Reclamation](http://www.rdrop.com/users/paulmck/RCU/hart_ipdps06.pdf)
 
    Multicore systems are ubiquitous but modern concurrent programming
techniques still do not see wide-spread adoption. Most concurrent software
(developed in low-level languages) still relies on error-prone and unscalable
memory management techniques for correctness despite the introduction of
superior methods over 30 years ago. Safe memory reclamation allows for
performant and robust memory management that is also suitable for advanced
concurrent programming techniques such as non-blocking synchronization. If
properly used, safe memory reclamation techniques allow improved performance and
simplicity without the complexity of full-blown garbage collection. This paper
provides a terrific overview of common safe memory reclamation mechanisms and
then explores their performance implications.

# 二、 核心学术论文与本地文献原件


### 本地归档核心学术论文（PDF）

- **[Making Lockless Synchronization Fast](./raw/memory_management/making-lockless-synchronization-fast.pdf)** `(0.47 MB)`
- **[Scatteralloc Massively Parallel Dynamic Memory Allocation For The Gpu](./raw/memory_management/scatteralloc-massively-parallel-dynamic-memory-allocation-for-the-gpu.pdf)** `(1.19 MB)`


---
_本地归档路径: `corpus/papers_we_love/raw/memory_management/`_
