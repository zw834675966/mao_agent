---
title: "高并发系统设计基本原则与权衡模型 (Scalability Principles)"
author: "Binh Nguyen 与全球架构师社区"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "架构设计原则与理论模型"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
  - "高并发原则"
  - "CAP定理"
  - "PACELC"
  - "设计权衡"
  - "可扩展性"
---

〔本篇梳理构建可扩展、高可靠与高性能大规模分布式系统的核心理论原则与设计权衡模式。〕

# 一、 架构模块全景阐述

* [Lessons from Giant-Scale Services - Eric Brewer, UC Berkeley & Google](https://people.eecs.berkeley.edu/~brewer/papers/GiantScale-IEEE.pdf)
* [Designs, Lessons and Advice from Building Large Distributed Systems - Jeff Dean, Google](https://www.cs.cornell.edu/projects/ladis2009/talks/dean-keynote-ladis2009.pdf)
* [How to Design a Good API & Why it Matters - Joshua Bloch, CMU & Google](https://www.infoq.com/presentations/effective-api-design)
* [On Efficiency, Reliability, Scaling - James Hamilton, VP at AWS](http://mvdirona.com/jrh/work/)
* [Principles of Chaos Engineering](https://www.usenix.org/conference/srecon17americas/program/presentation/rosenthal)
* [Finding the Order in Chaos](https://www.usenix.org/conference/srecon16/program/presentation/lueder)
* [The Twelve-Factor App](https://12factor.net/)
* [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
* [High Cohesion and Low Coupling](http://www.math-cs.gordon.edu/courses/cs211/lectures-2009/Cohesion,Coupling,MVC.pdf)
* [Monoliths and Microservices](https://medium.com/@SkyscannerEng/monoliths-and-microservices-8c65708c3dbf)
* [CAP Theorem and Trade-offs](http://robertgreiner.com/2014/08/cap-theorem-revisited/)
* [CP Databases and AP Databases](https://blog.andyet.com/2014/10/01/right-database)
* [Stateless vs Stateful Scalability](http://ithare.com/scaling-stateful-objects/)	
* [Scale Up vs Scale Out: Hidden Costs](https://blog.codinghorror.com/scaling-up-vs-scaling-out-hidden-costs/)
* [ACID and BASE](https://neo4j.com/blog/acid-vs-base-consistency-models-explained/)
* [Blocking/Non-Blocking and Sync/Async](https://blogs.msdn.microsoft.com/csliu/2009/08/27/io-concept-blockingnon-blocking-vs-syncasync/)
* [Performance and Scalability of Databases](https://use-the-index-luke.com/sql/testing-scalability)
* [Database Isolation Levels and Effects on Performance and Scalability](http://highscalability.com/blog/2011/2/10/database-isolation-levels-and-their-effects-on-performance-a.html)
* [The Probability of Data Loss in Large Clusters](https://martin.kleppmann.com/2017/01/26/data-loss-in-large-clusters.html)
* [Data Access for Highly-Scalable Solutions: Using SQL, NoSQL, and Polyglot Persistence](https://docs.microsoft.com/en-us/previous-versions/msp-n-p/dn271399(v=pandp.10))
* [SQL vs NoSQL](https://www.upwork.com/hiring/data/sql-vs-nosql-databases-whats-the-difference/)
* [SQL vs NoSQL - Lesson Learned at Salesforce](https://engineering.salesforce.com/sql-or-nosql-9eaf1d92545b)
* [NoSQL Databases: Survey and Decision Guidance](https://medium.baqend.com/nosql-databases-a-survey-and-decision-guidance-ea7823a822d)
* [How Sharding Works](https://medium.com/@jeeyoungk/how-sharding-works-b4dec46b3f6)
* [Consistent Hashing](http://www.tom-e-white.com/2007/11/consistent-hashing.html)
* [Consistent Hashing: Algorithmic Tradeoffs](https://medium.com/@dgryski/consistent-hashing-algorithmic-tradeoffs-ef6b8e2fcae8)
* [Don’t be tricked by the Hashing Trick](https://booking.ai/dont-be-tricked-by-the-hashing-trick-192a6aae3087)
* [Uniform Consistent Hashing at Netflix](https://medium.com/netflix-techblog/distributing-content-to-open-connect-3e3e391d4dc9)
* [Eventually Consistent - Werner Vogels, CTO at Amazon](https://www.allthingsdistributed.com/2008/12/eventually_consistent.html)
* [Cache is King](https://www.stevesouders.com/blog/2012/10/11/cache-is-king/)
* [Anti-Caching](https://www.the-paper-trail.org/post/2014-06-06-paper-notes-anti-caching/)
* [Understand Latency](http://highscalability.com/latency-everywhere-and-it-costs-you-sales-how-crush-it)
* [Latency Numbers Every Programmer Should Know](http://norvig.com/21-days.html#answers)
* [The Calculus of Service Availability](https://queue.acm.org/detail.cfm?id=3096459&__s=dnkxuaws9pogqdnxmx8i)
* [Architecture Issues When Scaling Web Applications: Bottlenecks, Database, CPU, IO](http://highscalability.com/blog/2014/5/12/4-architecture-issues-when-scaling-web-applications-bottlene.html)	
* [Common Bottlenecks](http://highscalability.com/blog/2012/5/16/big-list-of-20-common-bottlenecks.html)
* [Life Beyond Distributed Transactions](https://queue.acm.org/detail.cfm?id=3025012)
* [Relying on Software to Redirect Traffic Reliably at Various Layers](https://www.usenix.org/conference/srecon15/program/presentation/taveira)
* [Breaking Things on Purpose](https://www.usenix.org/conference/srecon17americas/program/presentation/andrus)
* [Avoid Over Engineering](https://medium.com/@rdsubhas/10-modern-software-engineering-mistakes-bc67fbef4fc8)
* [Scalability Worst Practices](https://www.infoq.com/articles/scalability-worst-practices)
* [Use Solid Technologies - Don’t Re-invent the Wheel - Keep It Simple!](https://medium.com/@DataStax/instagram-engineerings-3-rules-to-a-scalable-cloud-application-architecture-c44afed31406)
* [Simplicity by Distributing Complexity](https://engineering.zalando.com/posts/2018/01/simplicity-by-distributing-complexity.html)
* [Why Over-Reusing is Bad](http://tech.transferwise.com/why-over-reusing-is-bad/)
* [Performance is a Feature](https://blog.codinghorror.com/performance-is-a-feature/)
* [Make Performance Part of Your Workflow](https://codeascraft.com/2014/12/11/make-performance-part-of-your-workflow/)
* [The Benefits of Server Side Rendering over Client Side Rendering](https://medium.com/walmartlabs/the-benefits-of-server-side-rendering-over-client-side-rendering-5d07ff2cefe8)
* [Automate and Abstract: Lessons at Facebook](https://architecht.io/lessons-from-facebook-on-engineering-for-scale-f5716f0afc7a)
* [AWS Do's and Don'ts](https://8thlight.com/blog/sarah-sunday/2017/09/15/aws-dos-and-donts.html)
* [(UI) Design Doesn’t Scale - Stanley Wood, Design Director at Spotify](https://medium.com/@hellostanley/design-doesnt-scale-4d81e12cbc3e)
* [Linux Performance](http://www.brendangregg.com/linuxperf.html)
* [Building Fast and Resilient Web Applications - Ilya Grigorik](https://www.igvita.com/2016/05/20/building-fast-and-resilient-web-applications/)
* [Accept Partial Failures, Minimize Service Loss](https://www.usenix.org/conference/srecon17asia/program/presentation/wang_daxin)
* [Design for Resiliency](http://highscalability.com/blog/2012/12/31/designing-for-resiliency-will-be-so-2013.html)
* [Design for Self-healing](https://docs.microsoft.com/en-us/azure/architecture/guide/design-principles/self-healing)
* [Design for Scaling Out](https://docs.microsoft.com/en-us/azure/architecture/guide/design-principles/scale-out)	
* [Design for Evolution](https://docs.microsoft.com/en-us/azure/architecture/guide/design-principles/design-for-evolution)
* [Learn from Mistakes](http://highscalability.com/blog/2013/8/26/reddit-lessons-learned-from-mistakes-made-scaling-to-1-billi.html)

# 二、 文献引文与出处来源

- **知识库出处**: [binhnguyennus/awesome-scalability (GitHub)](https://github.com/binhnguyennus/awesome-scalability)
- **专题分类**: 分布式系统与高并发架构实战文库 · 架构设计原则与理论模型
