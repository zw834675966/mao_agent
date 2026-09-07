---
title: "分布式系统稳定性治理与容错防护 (System Stability)"
author: "Binh Nguyen 与全球架构师社区"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "系统稳定性与容灾保护"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
  - "稳定性治理"
  - "熔断器"
  - "限流降级"
  - "舱壁隔离"
  - "接口幂等性"
---

〔本篇介绍服务雪崩防护、超时重试、熔断器（Circuit Breaker）、速率限制与故障隔离技术方案。〕

# 一、 架构模块全景阐述

* [Circuit Breaker](https://martinfowler.com/bliki/CircuitBreaker.html)
	* [Circuit Breaking in Distributed Systems](https://www.infoq.com/presentations/circuit-breaking-distributed-systems)
	* [Circuit Breaker for Scaling Containers](https://f5.com/about-us/blog/articles/the-art-of-scaling-containers-circuit-breakers-28919)
	* [Lessons in Resilience at SoundCloud](https://developers.soundcloud.com/blog/lessons-in-resilience-at-SoundCloud)
	* [Protector: Circuit Breaker for Time Series Databases at Trivago](http://tech.trivago.com/2016/02/23/protector/)
	* [Improved Production Stability with Circuit Breakers at Heroku](https://blog.heroku.com/improved-production-stability-with-circuit-breakers)
	* [Circuit Breaker at Zendesk](https://medium.com/zendesk-engineering/the-joys-of-circuit-breaking-ee6584acd687)
	* [Circuit Breaker at Traveloka](https://medium.com/traveloka-engineering/circuit-breakers-dont-let-your-dependencies-bring-you-down-5ba1c5cf1eec)
	* [Circuit Breaker at Shopify](https://shopify.engineering/circuit-breaker-misconfigured)
* [Timeouts](https://www.javaworld.com/article/2824163/application-performance/stability-patterns-applied-in-a-restful-architecture.html)
	* [Fault Tolerance (Timeouts and Retries, Thread Separation, Semaphores, Circuit Breakers) at Netflix](https://medium.com/netflix-techblog/fault-tolerance-in-a-high-volume-distributed-system-91ab4faae74a)
	* [Enforce Timeout: A Reliability Methodology at DoorDash](https://doordash.engineering/2018/12/21/enforce-timeout-a-doordash-reliability-methodology/)
	* [Troubleshooting a Connection Timeout Issue with tcp_tw_recycle Enabled at eBay](https://www.ebayinc.com/stories/blogs/tech/a-vip-connection-timeout-issue-caused-by-snat-and-tcp-tw-recycle/)
* [Crash-safe Replication for MySQL at Booking.com](https://medium.com/booking-com-infrastructure/better-crash-safe-replication-for-mysql-a336a69b317f)
* [Bulkheads: Partition and Tolerate Failure in One Part](https://skife.org/architecture/fault-tolerance/2009/12/31/bulkheads.html)
* [Steady State: Always Put Logs on Separate Disk](https://docs.microsoft.com/en-us/sql/relational-databases/policy-based-management/place-data-and-log-files-on-separate-drives)
* [Throttling: Maintain a Steady Pace](http://www.sosp.org/2001/papers/welsh.pdf)
* [Multi-Clustering: Improving Resiliency and Stability of a Large-scale Monolithic API Service at LinkedIn](https://engineering.linkedin.com/blog/2017/11/improving-resiliency-and-stability-of-a-large-scale-api)
* [Determinism (4 parts) in League of Legends Server](https://engineering.riotgames.com/news/determinism-league-legends-fixing-divergences)

# 二、 文献引文与出处来源

- **知识库出处**: [binhnguyennus/awesome-scalability (GitHub)](https://github.com/binhnguyennus/awesome-scalability)
- **专题分类**: 分布式系统与高并发架构实战文库 · 系统稳定性与容灾保护
