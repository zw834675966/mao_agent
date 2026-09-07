---
title: "经典论文导读: Time Series"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "Time Series"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
  - "Papers We Love"
  - "计算机经典论文"
  - "Time Series"
---

〔本篇为 Papers We Love 经典学术文献库关于“Time Series”领域收录的传世奠基论文全景导读与本地文献档案。〕

# 一、 领域学术导读与背景

Important papers relating to time-series data

Time-series data presents specific but very common problems for efficient 
analysis, resulting in the need for columnar data stores and iterative 
one-pass processing.

The included documents are:

* [:scroll:](https://github.com/papers-we-love/papers-we-love/blob/main/time_series/operators-on-inhomogeneous-time-series.pdf) [Operators on Inhomogeneous Time Series](http://papers.ssrn.com/sol3/papers.cfm?abstract_id=208278)  - Gilles O. Zumbach and Ulrich A. Müller

  We present a toolbox to compute and extract information from
  inhomogeneous (i.e. unequally spaced) time series. The toolbox
  contains a large set of operators, mapping from the space of
  inhomogeneous time series to itself.

  These operators are computationally efficient (time and memory-wise)
  and suitable for stochastic processes. This makes them attractive for
  processing high-frequency data in finance and other fields. Using a
  basic set of operators, we easily construct more powerful combined
  operators which cover a wide set of typical applications.

* [:scroll:](https://github.com/papers-we-love/papers-we-love/blob/main/time_series/ts-asap.pdf)
  [ASAP: Automatic Smoothing for Attention Prioritization in Streaming Time Series Visualization](https://arxiv.org/abs/1703.00983)  - Kexin Rong, Peter Bailis

  Time Series smoothing method to better prioritize attention in time series
  exploration and monitoring visualizations, smooth the time series as much as
  possible to remove noise while still retaining large-scale structure. We
  develop a new technique for automatically smoothing streaming time series
  that adaptively optimizes this trade-off between noise reduction (i.e.,
  variance) and outlier retention (i.e., kurtosis).

# 二、 核心学术论文与本地文献原件


### 本地归档核心学术论文（PDF）

- **[Operators On Inhomogeneous Time Series](./raw/time_series/operators-on-inhomogeneous-time-series.pdf)** `(0.35 MB)`
- **[Ts Asap](./raw/time_series/ts-asap.pdf)** `(1.58 MB)`


---
_本地归档路径: `corpus/papers_we_love/raw/time_series/`_
