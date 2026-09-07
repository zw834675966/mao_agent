---
title: "经典论文导读: 无锁与非阻塞算法 (Non-blocking Algorithms)"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "无锁与非阻塞算法 (Non-blocking Algorithms)"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
  - "Papers We Love"
  - "计算机经典论文"
  - "无锁与非阻塞算法"
---

〔本篇为 Papers We Love 经典学术文献库关于“无锁与非阻塞算法 (Non-blocking Algorithms)”领域收录的传世奠基论文全景导读与本地文献档案。〕

# 一、 领域学术导读与背景

# Non-Blocking Algorithmics

An non-blocking algorithm is an algorithm in which failure or suspension of any thread cannot cause failure or suspension of another thread; for some operations, these algorithms provide a useful alternative to traditional blocking implementations. A non-blocking algorithm is lock-free if there is guaranteed system-wide progress, and wait-free if there is also guaranteed per-thread progress.

## Included Papers

* [:scroll:](a-wait-free-stack.pdf) [A Wait-Free Stack (2015)](https://arxiv.org/abs/1510.00116) (Seep Goel, Pooja Aggarwal, Smruti R. Sarangi)
* [:scroll:](a-wait-free-queue-as-fast-as-fetch-and-add.pdf) [A Wait-free Queue as Fast as Fetch-and-Add (2016)](http://chaoran.me/assets/pdf/wfq-ppopp16.pdf) (Chaoran Yang, John Mellor-Crummey)
* [:scroll:](efficient-lock-free-b+trees.pdf) [Efficient Lock-free B+trees (2014)](http://orbit.dtu.dk/files/102419168/abstract_ELB_trees.pdf) (Lars Frydendal Bonnichsen, Sven Karlsson, Christian W. Probst)

# 二、 核心学术论文与本地文献原件


### 本地归档核心学术论文（PDF）

- **[A Wait Free Queue As Fast As Fetch And Add](./raw/non_blocking_algorithms/a-wait-free-queue-as-fast-as-fetch-and-add.pdf)** `(0.87 MB)`
- **[A Wait Free Stack](./raw/non_blocking_algorithms/a-wait-free-stack.pdf)** `(0.46 MB)`
- **[Efficient Lock Free B+Trees](./raw/non_blocking_algorithms/efficient-lock-free-b+trees.pdf)** `(0.25 MB)`


---
_本地归档路径: `corpus/papers_we_love/raw/non_blocking_algorithms/`_
