---
title: "高可用架构、故障转移与数据复制 (System Availability)"
author: "Binh Nguyen 与全球架构师社区"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "高可用与容灾架构"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
  - "高可用性"
  - "故障转移"
  - "主从复制"
  - "最终一致性"
  - "共识算法"
---

〔本篇阐述高可用系统的关键指标、故障自动转移（Fail-over）、冗余备份与复制一致性权衡。〕

# 一、 架构模块全景阐述

* [Resilience Engineering: Learning to Embrace Failure](https://queue.acm.org/detail.cfm?id=2371297)	
	* [Resilience Engineering with Project Waterbear at LinkedIn](https://engineering.linkedin.com/blog/2017/11/resilience-engineering-at-linkedin-with-project-waterbear)
	* [Resiliency against Traffic Oversaturation at iHeartRadio](https://tech.iheart.com/resiliency-against-traffic-oversaturation-77c5ed92a5fb)
	* [Resiliency in Distributed Systems at GO-JEK](https://blog.gojekengineering.com/resiliency-in-distributed-systems-efd30f74baf4)
	* [Practical NoSQL Resilience Design Pattern for the Enterprise at eBay](https://www.ebayinc.com/stories/blogs/tech/practical-nosql-resilience-design-pattern-for-the-enterprise/)
	* [Ensuring Resilience to Disaster at Quora](https://quoraengineering.quora.com/Ensuring-Quoras-Resilience-to-Disaster)
	* [Site Resiliency at Expedia](https://www.infoq.com/presentations/expedia-website-resiliency?utm_source=presentations_about_Case_Study&utm_medium=link&utm_campaign=Case_Study)
	* [Resiliency and Disaster Recovery with Kafka at eBay](https://tech.ebayinc.com/engineering/resiliency-and-disaster-recovery-with-kafka/)
	* [Disaster Recovery for Multi-Region Kafka at Uber](https://eng.uber.com/kafka/)
* [Failover](http://cloudpatterns.org/mechanisms/failover_system)
	* [The Evolution of Global Traffic Routing and Failover](https://www.usenix.org/conference/srecon16/program/presentation/heady)
	* [Testing for Disaster Recovery Failover Testing](https://www.usenix.org/conference/srecon17asia/program/presentation/liu_zehua)
	* [Designing a Microservices Architecture for Failure](https://blog.risingstack.com/designing-microservices-architecture-for-failure/)
	* [ELB for Automatic Failover at GoSquared](https://engineering.gosquared.com/use-elb-automatic-failover)
	* [Eliminate the Database for Higher Availability at American Express](http://americanexpress.io/eliminate-the-database-for-higher-availability/)
	* [Failover with Redis Sentinel at Vinted](http://engineering.vinted.com/2015/09/03/failover-with-redis-sentinel/)
	* [High-availability SaaS Infrastructure at FreeAgent](http://engineering.freeagent.com/2017/02/06/ha-infrastructure-without-breaking-the-bank/)
	* [MySQL High Availability at GitHub](https://github.blog/2018-06-20-mysql-high-availability-at-github/)
	* [MySQL High Availability at Eventbrite](https://www.eventbrite.com/engineering/mysql-high-availability-at-eventbrite/)
	* [Business Continuity & Disaster Recovery at Walmart](https://medium.com/walmartlabs/business-continuity-disaster-recovery-in-the-microservices-world-ef2adca363df)
* [Load Balancing](https://blog.vivekpanyam.com/scaling-a-web-service-load-balancing/)
	* [Introduction to Modern Network Load Balancing and Proxying](https://blog.envoyproxy.io/introduction-to-modern-network-load-balancing-and-proxying-a57f6ff80236)
	* [Top Five (Load Balancing) Scalability Patterns](https://www.f5.com/company/blog/top-five-scalability-patterns)
	* [Load Balancing infrastructure to support more than 1.3 billion users at Facebook](https://www.usenix.org/conference/srecon15europe/program/presentation/shuff)
	* [DHCPLB: DHCP Load Balancer at Facebook](https://code.facebook.com/posts/1734309626831603/dhcplb-an-open-source-load-balancer/)
	* [Katran: Scalable Network Load Balancer at Facebook](https://code.facebook.com/posts/1906146702752923/open-sourcing-katran-a-scalable-network-load-balancer/)
	* [Deterministic Aperture: A Distributed, Load Balancing Algorithm at Twitter](https://blog.twitter.com/engineering/en_us/topics/infrastructure/2019/daperture-load-balancer.html)	
	* [Load Balancing with Eureka at Netflix](https://medium.com/netflix-techblog/netflix-shares-cloud-load-balancing-and-failover-tool-eureka-c10647ef95e5)
	* [Edge Load Balancing at Netflix](https://medium.com/netflix-techblog/netflix-edge-load-balancing-695308b5548c)
	* [Zuul 2: Cloud Gateway at Netflix](https://medium.com/netflix-techblog/open-sourcing-zuul-2-82ea476cb2b3)
	* [Load Balancing at Yelp](https://engineeringblog.yelp.com/2017/05/taking-zero-downtime-load-balancing-even-further.html)
	* [Load Balancing at Github](https://githubengineering.com/introducing-glb/)
	* [Consistent Hashing to Improve Load Balancing at Vimeo](https://medium.com/vimeo-engineering-blog/improving-load-balancing-with-a-new-consistent-hashing-algorithm-9f1bd75709ed)
	* [UDP Load Balancing at 500 pixel](https://developers.500px.com/udp-load-balancing-with-keepalived-167382d7ad08)
	* [QALM: QoS Load Management Framework at Uber](https://eng.uber.com/qalm/)	
	* [Traffic Steering using Rum DNS at LinkedIn](https://www.usenix.org/conference/srecon17europe/program/presentation/rastogi)
	* [Traffic Infrastructure (Edge Network) at Dropbox](https://dropbox.tech/infrastructure/dropbox-traffic-infrastructure-edge-network)
	* [Intelligent DNS based load balancing at Dropbox](https://dropbox.tech/infrastructure/intelligent-dns-based-load-balancing-at-dropbox)
	* [Monitor DNS systems at Stripe](https://stripe.com/en-sg/blog/secret-life-of-dns)
	* [Multi-DNS Architecture (3 parts) at Monday](https://medium.com/monday-engineering/how-and-why-we-migrated-our-dns-from-cloudflare-to-a-multi-dns-architecture-part-3-584a470f4062)
	* [Dynamic Anycast DNS Infrastructure at Hulu](https://medium.com/hulu-tech-blog/building-hulus-dynamic-anycast-dns-infrastructure-985a7a11fd30)
* [Rate Limiting](https://www.keycdn.com/support/rate-limiting/)
	* [Rate Limiting for Scaling to Millions of Domains at Cloudflare](https://blog.cloudflare.com/counting-things-a-lot-of-different-things/)
	* [Cloud Bouncer: Distributed Rate Limiting at Yahoo](https://yahooeng.tumblr.com/post/111288877956/cloud-bouncer-distributed-rate-limiting-at-yahoo)
	* [Scaling API with Rate Limiters at Stripe](https://stripe.com/blog/rate-limiters)
	* [Distributed Rate Limiting at Allegro](https://allegro.tech/2017/04/hermes-max-rate.html)
	* [Ratequeue: Core Queueing-And-Rate-Limiting System at Twilio](https://www.twilio.com/blog/2017/11/chaos-engineering-ratequeue-ha.html)
	* [Quotas Service at Grab](https://engineering.grab.com/quotas-service)
	* [Rate Limiting at Figma](https://medium.com/figma-design/an-alternative-approach-to-rate-limiting-f8a06cf7c94c)	
* [Autoscaling](https://medium.com/@BotmetricHQ/top-11-hard-won-lessons-learned-about-aws-auto-scaling-5bfe56da755f)
	* [Autoscaling Pinterest](https://medium.com/@Pinterest_Engineering/auto-scaling-pinterest-df1d2beb4d64)
	* [Autoscaling Based on Request Queuing at Square](https://medium.com/square-corner-blog/autoscaling-based-on-request-queuing-c4c0f57f860f)
	* [Autoscaling Jenkins at Trivago](http://tech.trivago.com/2017/02/17/your-definite-guide-for-autoscaling-jenkins/)
	* [Autoscaling Pub-Sub Consumers at Spotify](https://labs.spotify.com/2017/11/20/autoscaling-pub-sub-consumers/)
	* [Autoscaling Bigtable Clusters based on CPU Load at Spotify](https://labs.spotify.com/2018/12/18/bigtable-autoscaler-saving-money-and-time-using-managed-storage/)
	* [Autoscaling AWS Step Functions Activities at Yelp](https://engineeringblog.yelp.com/2019/06/autoscaling-aws-step-functions-activities.html)
	* [Scryer: Predictive Auto Scaling Engine at Netflix](https://medium.com/netflix-techblog/scryer-netflixs-predictive-auto-scaling-engine-a3f8fc922270)	
	* [Bouncer: Simple AWS Auto Scaling Rollovers at Palantir](https://medium.com/palantir/bouncer-simple-aws-auto-scaling-rollovers-c5af601d65d4)
	* [Clusterman: Autoscaling Mesos Clusters at Yelp](https://engineeringblog.yelp.com/2019/02/autoscaling-mesos-clusters-with-clusterman.html)
* [Availability in Globally Distributed Storage Systems at Google](http://static.googleusercontent.com/media/research.google.com/en/us/pubs/archive/36737.pdf)	
* [NodeJS High Availability at Yahoo](https://yahooeng.tumblr.com/post/68823943185/nodejs-high-availability)
* [Operations (11 parts) at LinkedIn](https://www.linkedin.com/pulse/introduction-every-day-monday-operations-benjamin-purgason)
* [Monitoring Powers High Availability for LinkedIn Feed](https://www.usenix.org/conference/srecon17americas/program/presentation/barot)
* [Supporting Global Events at Facebook](https://code.facebook.com/posts/166966743929963/how-production-engineers-support-global-events-on-facebook/)
* [High Availability at BlaBlaCar](https://medium.com/blablacar-tech/the-expendables-backends-high-availability-at-blablacar-8cea3b95b26b)
* [High Availability at Netflix](https://medium.com/@NetflixTechBlog/tips-for-high-availability-be0472f2599c)
* [High Availability Cloud Infrastructure at Twilio](https://www.twilio.com/engineering/2011/12/12/scaling-high-availablity-infrastructure-in-cloud)
* [High Availability with Distributed DB on Kubernetes at Airbnb](https://airbnb.tech/infrastructure/achieving-high-availability-with-distributed-database-on-kubernetes-at-airbnb/)
* [Automating Datacenter Operations at Dropbox](https://dropbox.tech/infrastructure/automating-datacenter-operations-at-dropbox)
* [Globalizing Player Accounts at Riot Games](https://technology.riotgames.com/news/globalizing-player-accounts)

# 二、 文献引文与出处来源

- **知识库出处**: [binhnguyennus/awesome-scalability (GitHub)](https://github.com/binhnguyennus/awesome-scalability)
- **专题分类**: 分布式系统与高并发架构实战文库 · 高可用与容灾架构
