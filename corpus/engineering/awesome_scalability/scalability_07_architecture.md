---
title: "现代分布式系统架构风格与演进 (System Architecture)"
author: "Binh Nguyen 与全球架构师社区"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "软件架构模式与演进"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
  - "微服务架构"
  - "事件驱动架构"
  - "CQRS"
  - "六边形架构"
  - "Serverless"
---

〔本篇对比微服务、事件驱动、CQRS、领域驱动设计（DDD）与面向服务架构的适用边界与演进逻辑。〕

# 一、 架构模块全景阐述

* [Tech Stack at Medium](https://medium.engineering/the-stack-that-helped-medium-drive-2-6-millennia-of-reading-time-e56801f7c492)
* [Tech Stack at Shopify](https://engineering.shopify.com/blogs/engineering/e-commerce-at-scale-inside-shopifys-tech-stack)
* [Building Services (4 parts) at Airbnb](https://medium.com/airbnb-engineering/building-services-at-airbnb-part-4-23c95e428064)
* [Architecture of Evernote](https://evernote.com/blog/a-digest-of-evernotes-architecture/)
* [Architecture of Chat Service (3 parts) at Riot Games](https://engineering.riotgames.com/news/chat-service-architecture-persistence)
* [Architecture of League of Legends Client Update](https://technology.riotgames.com/news/architecture-league-client-update)
* [Architecture of Ad Platform at Twitter](https://blog.twitter.com/engineering/en_us/topics/infrastructure/2020/building-twitters-ad-platform-architecture-for-the-future.html)
* [Architecture of API Gateway at Uber](https://eng.uber.com/architecture-api-gateway/)
* [Architecture of API Gateway at Tinder](https://medium.com/tinder/how-we-built-the-tinder-api-gateway-831c6ca5ceca)
* [Basic Architecture of Slack](https://slack.engineering/how-slack-built-shared-channels-8d42c895b19f)
* [Lightweight Distributed Architecture to Handle Thousands of Library Releases at eBay](https://tech.ebayinc.com/engineering/a-lightweight-distributed-architecture-to-handle-thousands-of-library-releases-at-ebay/)
* [Back-end at LinkedIn](https://engineering.linkedin.com/architecture/brief-history-scaling-linkedin)
* [Back-end at Flickr](https://yahooeng.tumblr.com/post/157200523046/introducing-tripod-flickrs-backend-refactored)
* [Infrastructure (3 parts) at Zendesk](https://medium.com/zendesk-engineering/the-history-of-infrastructure-at-zendesk-part-3-foundation-team-forming-and-evolving-9859e40f5390)
* [Cloud Infrastructure at Grubhub](https://bytes.grubhub.com/cloud-infrastructure-at-grubhub-94db998a898a)
* [Real-time Presence Platform at LinkedIn](https://engineering.linkedin.com/blog/2018/01/now-you-see-me--now-you-dont--linkedins-real-time-presence-platf)
* [Settings Platform at LinkedIn](https://engineering.linkedin.com/blog/2019/05/building-member-trust-through-a-centralized-and-scalable-setting)
* [Nearline System for Scale and Performance (2 parts) at Glassdoor](https://medium.com/glassdoor-engineering/building-a-nearline-system-for-scale-and-performance-part-ii-9e01bf51b23d)
* [Real-time User Action Counting System for Ads at Pinterest](https://medium.com/@Pinterest_Engineering/building-a-real-time-user-action-counting-system-for-ads-88a60d9c9a)
* [API Platform at Riot Games](https://engineering.riotgames.com/news/riot-games-api-deep-dive)
* [Games Platform at The New York Times](https://open.nytimes.com/play-by-play-moving-the-nyt-games-platform-to-gcp-with-zero-downtime-cf425898d569)
* [Kabootar: Communication Platform at Swiggy](https://bytes.swiggy.com/kabootar-swiggys-communication-platform-e5a43cc25629)
* [Simone: Distributed Simulation Service at Netflix](https://medium.com/netflix-techblog/https-medium-com-netflix-techblog-simone-a-distributed-simulation-service-b2c85131ca1b)
* [Seagull: Distributed System that Helps Running > 20 Million Tests Per Day at Yelp](https://engineeringblog.yelp.com/2017/04/how-yelp-runs-millions-of-tests-every-day.html)
* [PriceAggregator: Intelligent System for Hotel Price Fetching (3 parts) at Agoda](https://medium.com/agoda-engineering/priceaggregator-an-intelligent-system-for-hotel-price-fetching-part-3-52acfc705081)
* [Phoenix: Testing Platform (3 parts) at Tinder](https://medium.com/tinder-engineering/phoenix-tinders-testing-platform-part-iii-520728b9537)
* [Hexagonal Architecture at Netflix](https://netflixtechblog.com/ready-for-changes-with-hexagonal-architecture-b315ec967749)
* [Architecture of Sticker Services at LINE](https://www.slideshare.net/linecorp/architecture-sustaining-line-sticker-services)
* [Stack Overflow Enterprise at Palantir](https://medium.com/@palantir/terraforming-stack-overflow-enterprise-in-aws-47ee431e6be7)
* [Architecture of Following Feed, Interest Feed, and Picked For You at Pinterest](https://medium.com/@Pinterest_Engineering/building-a-dynamic-and-responsive-pinterest-7d410e99f0a9)
* [API Specification Workflow at WeWork](https://engineering.wework.com/our-api-specification-workflow-9337448d6ee6)
* [Media Database at Netflix](https://medium.com/netflix-techblog/implementing-the-netflix-media-database-53b5a840b42a)
* [Member Transaction History Architecture at Walmart](https://medium.com/walmartlabs/member-transaction-history-architecture-8b6e34b87c21)
* [Sync Engine (2 parts) at Dropbox](https://dropbox.tech/infrastructure/-testing-our-new-sync-engine)
* [Ads Pacing Service at Twitter](https://blog.twitter.com/engineering/en_us/topics/infrastructure/2021/how-we-built-twitter-s-highly-reliable-ads-pacing-service)
* [Rapid Event Notification System at Netflix](https://netflixtechblog.com/rapid-event-notification-system-at-netflix-6deb1d2b57d1)
* [Architectures of Finance, Banking, and Payment Systems](https://www.redhat.com/architect/portfolio/detail/12-integrating-a-modern-payments-architecture)
	* [Bank Backend at Monzo](https://monzo.com/blog/2016/09/19/building-a-modern-bank-backend/)
	* [Trading Platform for Scale at Wealthsimple](https://medium.com/@Wealthsimple/engineering-at-wealthsimple-reinventing-our-trading-platform-for-scale-17e332241b6c)
	* [Core Banking System at Margo Bank](https://medium.com/margobank/choosing-an-architecture-85750e1e5a03)
	* [Architecture of Nubank](https://www.infoq.com/presentations/nubank-architecture)
	* [Tech Stack at TransferWise](http://tech.transferwise.com/the-transferwise-stack-heartbeat-of-our-little-revolution/)
	* [Tech Stack at Addepar](https://medium.com/build-addepar/our-tech-stack-a4f55dab4b0d)
	* [Avoiding Double Payments in a Distributed Payments System at Airbnb](https://medium.com/airbnb-engineering/avoiding-double-payments-in-a-distributed-payments-system-2981f6b070bb)
	* [Scaling Payments (3 parts) at Etsy](https://www.etsy.com/sg-en/codeascraft/scaling-etsy-payments-with-vitess-part-3--reducing-cutover-risk)
	* [Handles Millions of Digital Transactions Safely Everyday at Paytm](https://paytm.com/blog/engineering/how-paytm-handles-millions-of-digital-transactions-safely-everyday/)
	* [Billing and Payment Platform at Grammarly](https://www.grammarly.com/blog/engineering/billing-and-payments-platform/)

# 二、 文献引文与出处来源

- **知识库出处**: [binhnguyennus/awesome-scalability (GitHub)](https://github.com/binhnguyennus/awesome-scalability)
- **专题分类**: 分布式系统与高并发架构实战文库 · 软件架构模式与演进
