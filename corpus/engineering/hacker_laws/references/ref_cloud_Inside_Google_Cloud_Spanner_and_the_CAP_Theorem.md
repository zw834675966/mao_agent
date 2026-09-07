---
title: "引用文献: Inside Cloud Spanner and the CAP Theorem | Google Cloud Blog"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://cloud.google.com/blog/products/gcp/inside-cloud-spanner-and-the-cap-theorem"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
---

〔本文档为黑客定律与工程哲学文库中《Inside Google Cloud Spanner and the CAP Theorem》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Inside Cloud Spanner and the CAP Theorem | Google Cloud Blog
- **原文链接**: [https://cloud.google.com/blog/products/gcp/inside-cloud-spanner-and-the-cap-theorem](https://cloud.google.com/blog/products/gcp/inside-cloud-spanner-and-the-cap-theorem)
- **引用锚文本**: Inside Google Cloud Spanner and the CAP Theorem
- **抓取状态**: success

# 二、 文献正文内容

CloudBlogContact sales Get started for free CloudBlog
AI & Machine Learning
API Management
Application Development
Application Modernization
Chrome Enterprise
Compute
Containers & Kubernetes
Data Analytics
Databases
DevOps & SRE
Maps & Geospatial
Security
Security & Identity
Threat Intelligence
Infrastructure
Infrastructure Modernization
Networking
Productivity & Collaboration
SAP on Google Cloud
Storage & Data Transfer
Sustainability
Ecosystem
IT Leaders
Industries
Financial Services
Healthcare & Life Sciences
Manufacturing
Media & Entertainment
Public Sector
Retail
Supply Chain
Telecommunications
Partners
Startups & SMB
Training & Certifications
Inside Google Cloud
Google Cloud Next & Events
Google Cloud Consulting
Google Maps Platform
Google Workspace
Developers & Practitioners
Transform with Google CloudContact sales Get started for free Google Cloud
Inside Cloud Spanner and the CAP TheoremFebruary 15, 2017
Eric Brewer
VP of Infrastructure & Google Fellow
Building systems that manage globally distributed data, provide data consistency and are also highly available is really hard. The beauty of the cloud is that someone else can build that for you.
The CAP theorem says that a database can only have two of the three following desirable properties:
C: consistency, which implies a single value for shared data
A: 100% availability, for both reads and updates
P: tolerance to network partitions
This leads to three kinds of systems: CA, CP and AP, based on what letter you leave out. Designers are not entitled to two of the three, and many systems have zero or one of the properties.
For distributed systems over a “wide area,” it's generally viewed that partitions are inevitable, although not necessarily common. If you believe that partitions are inevitable, any distributed system must be prepared to forfeit either consistency (AP) or availability (CP), which is not a choice anyone wants to make. In fact, the original point of the CAP theorem was to get designers to take this tradeoff seriously. But there are two important caveats: First, you only need to forfeit consistency or availability during an actual partition, and even then there are many mitigations. Second, the actual theorem is about 100% availability; a more interesting discussion is about the tradeoffs involved to achieve realistic high availability.
Spanner joins Google CloudToday, Google is releasing Cloud Spanner for use by Google Cloud Platform (GCP) customers. Spanner is Google’s highly available, global SQL database. It manages replicated data at great scale, both in terms of size of data and volume of transactions. It assigns globally consistent real-time timestamps to every datum written to it, and clients can do globally consistent reads across the entire database without locking.
In terms of CAP, Spanner claims to be both consistent and highly available despite operating over a wide area, which many find surprising or even unlikely. The claim thus merits some discussion. Does this mean that Spanner is a CA system as defined by CAP? The short answer is “no” technically, but “yes” in effect and its users can and do assume CA.
The purist answer is “no” because partitions can happen and in fact have happened at Google, and during some partitions, Spanner chooses C and forfeits A. It is technically a CP system.
However, no system provides 100% availability, so the pragmatic question is whether or not Spanner delivers availability that is so high that most users don't worry about its outages. For example, given there are many sources of outages for an application, if Spanner is an insignificant contributor to its downtime, then users are correct to not worry about it.
In practice, we find that Spanner does meet this bar, with more than five 9s of availability (less than one failure in 105). Given this, the target for multi-region Cloud Spanner will be right at five 9s, as it has some additional new pieces that will be higher risk for a while.
Inside Spanner
The next question is, how is Spanner able to achieve this?
There are several factors, but the most important one is that Spanner runs on Google’s private network. Unlike most wide-area networks, and especially the public internet, Google controls the entire network and thus can ensure redundancy of hardware and paths, and can also control upgrades and operations in general. Fibers will still be cut, and equipment will fail, but the overall system remains quite robust.
It also took years of operational improvements to get to this point. For much of the last decade, Google has improved its redundancy, its fault containment and, above all, its processes for evolution. We found that the network contributed less than 10% of Spanner’s already rare outages.
Building systems that can manage data that spans the globe, provide data consistency and are also highly available is possible; it’s just really hard. The beauty of the cloud is that someone else can build that for you, and you can focus on innovation core to your service or application.
Next steps
For a significantly deeper dive into the details, see the white paper also released today. It covers Spanner, consistency and availability in depth (including new data). It also looks at the role played by Google’s TrueTime system, which provides a globally synchronized clock.  We intend to release TrueTime for direct use by Cloud customers in the future.
Furthermore, look for the addition of new Cloud Spanner-related sessions at Google Cloud Next ‘17 in San Francisco next month. Register soon, because seats are limited.Posted in
Google Cloud
Databases
Spanner
Related articlesInside Google Cloud
What’s new with Google Cloud
By Google Cloud Content & Editorial  • 36-minute readAI & Machine Learning
What Google Cloud announced in AI this month
By Andrea Morange • 27-minute readAI & Machine Learning
What Google Cloud announced in AI this month - 2025
By Andrea Morange • 43-minute readInside Google Cloud
What’s new with Google Cloud - 2025
By Google Cloud Content & Editorial  • 39-minute read
Google Cloud
Google Cloud Products
Privacy
Terms
Cookies management controls
Help
Language‪English‬‪Deutsch‬‪Français‬‪한국어‬‪日本語‬

---
_本地归档时间: 2026-09-05 | 来源: https://cloud.google.com/blog/products/gcp/inside-cloud-spanner-and-the-cap-theorem_
