---
title: "低延迟高性能计算与系统调优 (System Performance)"
author: "Binh Nguyen 与全球架构师社区"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "性能工程与延迟优化"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
  - "性能调优"
  - "延迟与吞吐量"
  - "多级缓存"
  - "索引优化"
  - "网络协议栈"
---

〔本篇从吞吐量、P99延迟、网络通信、存储I/O到应用代码层，全方位梳理高性能系统的调优路径。〕

# 一、 架构模块全景阐述

* [Performance Optimization on OS, Storage, Database, Network](https://stackify.com/application-performance-metrics/)
	* [Improving Performance with Background Data Prefetching at Instagram](https://engineering.instagram.com/improving-performance-with-background-data-prefetching-b191acb39898)
	* [Fixing Linux filesystem performance regressions at LinkedIn](https://engineering.linkedin.com/blog/2020/fixing-linux-filesystem-performance-regressions)
	* [Compression Techniques to Solve Network I/O Bottlenecks at eBay](https://www.ebayinc.com/stories/blogs/tech/how-ebays-shopping-cart-used-compression-techniques-to-solve-network-io-bottlenecks/)
	* [Optimizing Web Servers for High Throughput and Low Latency at Dropbox](https://dropbox.tech/infrastructure/optimizing-web-servers-for-high-throughput-and-low-latency)
	* [Linux Performance Analysis in 60.000 Milliseconds at Netflix](https://medium.com/netflix-techblog/linux-performance-analysis-in-60-000-milliseconds-accc10403c55)
	* [Live Downsizing Google Cloud Persistent Disks (PD-SSD) at Mixpanel](https://engineering.mixpanel.com/2018/07/31/live-downsizing-google-cloud-pds-for-fun-and-profit/)
	* [Decreasing RAM Usage by 40% Using jemalloc with Python & Celery at Zapier](https://zapier.com/engineering/celery-python-jemalloc/)
	* [Reducing Memory Footprint at Slack](https://slack.engineering/reducing-slacks-memory-footprint-4480fec7e8eb)
	* [Continuous Load Testing at Slack](https://slack.engineering/continuous-load-testing/)
	* [Performance Improvements at Pinterest](https://medium.com/@Pinterest_Engineering/driving-user-growth-with-performance-improvements-cfc50dafadd7)
	* [Server Side Rendering at Wix](https://www.youtube.com/watch?v=f9xI2jR71Ms)
	* [30x Performance Improvements on MySQLStreamer at Yelp](https://engineeringblog.yelp.com/2018/02/making-30x-performance-improvements-on-yelps-mysqlstreamer.html)
	* [Optimizing APIs at Netflix](https://medium.com/netflix-techblog/optimizing-the-netflix-api-5c9ac715cf19)
	* [Performance Monitoring with Riemann and Clojure at Walmart](https://medium.com/walmartlabs/performance-monitoring-with-riemann-and-clojure-eafc07fcd375)
	* [Performance Tracking Dashboard for Live Games at Zynga](https://www.zynga.com/blogs/engineering/live-games-have-evolving-performance)
	* [Optimizing CAL Report Hadoop MapReduce Jobs at eBay](https://www.ebayinc.com/stories/blogs/tech/optimization-of-cal-report-hadoop-mapreduce-job/)
	* [Performance Tuning on Quartz Scheduler at eBay](https://www.ebayinc.com/stories/blogs/tech/performance-tuning-on-quartz-scheduler/)
	* [Profiling C++ (Part 1: Optimization, Part 2: Measurement and Analysis) at Riot Games](https://engineering.riotgames.com/news/profiling-optimisation)
	* [Profiling React Server-Side Rendering at HomeAway](https://medium.com/homeaway-tech-blog/profiling-react-server-side-rendering-to-free-the-node-js-event-loop-7f0fe455a901)
	* [Hardware-Assisted Video Transcoding at Dailymotion](https://medium.com/dailymotion-engineering/hardware-assisted-video-transcoding-at-dailymotion-66cd2db448ae)
	* [Cross Shard Transactions at 10 Million RPS at Dropbox](https://dropbox.tech/infrastructure/cross-shard-transactions-at-10-million-requests-per-second)
	* [API Profiling at Pinterest](https://medium.com/@Pinterest_Engineering/api-profiling-at-pinterest-6fa9333b4961)
	* [Pagelets Parallelize Server-side Processing at Yelp](https://engineeringblog.yelp.com/2017/07/generating-web-pages-in-parallel-with-pagelets.html)
	* [Improving key expiration in Redis at Twitter](https://blog.twitter.com/engineering/en_us/topics/infrastructure/2019/improving-key-expiration-in-redis.html)
	* [Ad Delivery Network Performance Optimization with Flame Graphs at MindGeek](https://medium.com/mindgeek-engineering-blog/ad-delivery-network-performance-optimization-with-flame-graphs-bc550cf59cf7)
	* [Predictive CPU isolation of containers at Netflix](https://medium.com/netflix-techblog/predictive-cpu-isolation-of-containers-at-netflix-91f014d856c7)
	* [Improving HDFS I/O Utilization for Efficiency at Uber](https://eng.uber.com/improving-hdfs-i-o-utilization-for-efficiency/)
	* [Cloud Jewels: Estimating kWh in the Cloud at Etsy](https://codeascraft.com/2020/04/23/cloud-jewels-estimating-kwh-in-the-cloud/)
	* [Unthrottled: Fixing CPU Limits in the Cloud (2 parts) at Indeed](https://engineering.indeedblog.com/blog/2019/12/unthrottled-fixing-cpu-limits-in-the-cloud/)
* [Performance Optimization by Tuning Garbage Collection](https://confluence.atlassian.com/enterprise/garbage-collection-gc-tuning-guide-461504616.html)
	* [Garbage Collection in Java Applications at LinkedIn](https://engineering.linkedin.com/garbage-collection/garbage-collection-optimization-high-throughput-and-low-latency-java-applications)
	* [Garbage Collection in High-Throughput, Low-Latency Machine Learning Services at Adobe](https://medium.com/adobetech/engineering-high-throughput-low-latency-machine-learning-services-7d45edac0271)
	* [Garbage Collection in Redux Applications at SoundCloud](https://developers.soundcloud.com/blog/garbage-collection-in-redux-applications)
	* [Garbage Collection in Go Application at Twitch](https://blog.twitch.tv/go-memory-ballast-how-i-learnt-to-stop-worrying-and-love-the-heap-26c2462549a2)
	* [Analyzing V8 Garbage Collection Logs at Alibaba](https://www.linux.com/blog/can-nodejs-scale-ask-team-alibaba)
	* [Python Garbage Collection for Dropping 50% Memory Growth Per Request at Instagram](https://instagram-engineering.com/copy-on-write-friendly-python-garbage-collection-ad6ed5233ddf)
	* [Performance Impact of Removing Out of Band Garbage Collector (OOBGC) at Github](https://githubengineering.com/removing-oobgc/)
	* [Debugging Java Memory Leaks at Allegro](https://allegro.tech/2018/05/a-comedy-of-errors-debugging-java-memory-leaks.html)
	* [Optimizing JVM at Alibaba](https://www.youtube.com/watch?v=X4tmr3nhZRg)
	* [Tuning JVM Memory for Large-scale Services at Uber](https://eng.uber.com/jvm-tuning-garbage-collection/)
	* [Solr Performance Tuning at Walmart](https://medium.com/walmartglobaltech/solr-performance-tuning-beb7d0d0f8d9)
	* [Memory Tuning a High Throughput Microservice at Flipkart](https://blog.flipkart.tech/memory-tuning-a-high-throughput-microservice-ed57b3e60997)
* [Performance Optimization on Image, Video, Page Load](https://developers.google.com/web/fundamentals/performance/why-performance-matters/)
	* [Optimizing 360 Photos at Scale at Facebook](https://code.facebook.com/posts/129055711052260/optimizing-360-photos-at-scale/)
	* [Reducing Image File Size in the Photos Infrastructure at Etsy](https://codeascraft.com/2017/05/30/reducing-image-file-size-at-etsy/)
	* [Improving GIF Performance at Pinterest](https://medium.com/@Pinterest_Engineering/improving-gif-performance-on-pinterest-8dad74bf92f1)
	* [Optimizing Video Playback Performance at Pinterest](https://medium.com/@Pinterest_Engineering/optimizing-video-playback-performance-caf55ce310d1)
	* [Optimizing Video Stream for Low Bandwidth with Dynamic Optimizer at Netflix](https://medium.com/netflix-techblog/optimized-shot-based-encodes-now-streaming-4b9464204830)
	* [Adaptive Video Streaming at YouTube](https://youtube-eng.googleblog.com/2018/04/making-high-quality-video-efficient.html)
    * [Reducing Video Loading Time at Dailymotion](https://medium.com/dailymotion/reducing-video-loading-time-fa9c997a2294)
	* [Improving Homepage Performance at Zillow](https://www.zillow.com/engineering/improving-homepage-performance/)
	* [The Process of Optimizing for Client Performance at Expedia](https://medium.com/expedia-engineering/go-fast-or-go-home-the-process-of-optimizing-for-client-performance-57bb497402e)
	* [Web Performance at BBC](https://medium.com/bbc-design-engineering/bbc-world-service-web-performance-26b08f7abfcc)
* [Performance Optimization by Brotli Compression](https://blogs.akamai.com/2016/02/understanding-brotlis-potential.html)
	* [Boosting Site Speed Using Brotli Compression at LinkedIn](https://engineering.linkedin.com/blog/2017/05/boosting-site-speed-using-brotli-compression)	
	* [Brotli at Booking.com](https://medium.com/booking-com-development/bookings-journey-with-brotli-978b249d34f3)
	* [Brotli at Treebo](https://tech.treebo.com/a-tale-of-brotli-compression-bcb071d9780a)
	* [Deploying Brotli for Static Content at Dropbox](https://dropbox.tech/infrastructure/deploying-brotli-for-static-content)
	* [Progressive Enhancement with Brotli at Yelp](https://engineeringblog.yelp.com/2017/07/progressive-enhancement-with-brotli.html)
	* [Speeding Up Redis with Compression at DoorDash](https://doordash.engineering/2019/01/02/speeding-up-redis-with-compression/)
* [Performance Optimization on Languages and Frameworks](https://www.techempower.com/benchmarks/)
	* [Python at Netflix](https://netflixtechblog.com/python-at-netflix-bba45dae649e)
	* [Python at scale (3 parts) at Instagram](https://instagram-engineering.com/python-at-scale-strict-modules-c0bb9245c834)
	* [OCaml best practices (2 parts) at Issuu](https://engineering.issuu.com/2018/12/10/our-current-ocaml-best-practices-part-2)
	* [PHP at Slack](https://slack.engineering/taking-php-seriously-cf7a60065329)
	* [Go at Trivago](https://tech.trivago.com/2020/03/02/why-we-chose-go/)
	* [TypeScript at Etsy](https://codeascraft.com/2021/11/08/etsys-journey-to-typescript/)
	* [Kotlin for taming state at Etsy](https://www.etsy.com/sg-en/codeascraft/sealed-classes-opened-my-mind)
	* [Kotlin at DoorDash](https://doordash.engineering/2022/03/22/how-to-leverage-functional-programming-in-kotlin-to-write-better-cleaner-code/)
	* [BPF and Go at Bumble](https://medium.com/bumble-tech/bpf-and-go-modern-forms-of-introspection-in-linux-6b9802682223)
	* [Ruby on Rails at GitLab](https://medium.com/gitlab-magazine/why-we-use-ruby-on-rails-to-build-gitlab-601dce4a7a38)
	* [Rust in production at Figma](https://medium.com/figma-design/rust-in-production-at-figma-e10a0ec31929)
	* [Choosing a Language Stack at WeWork](https://engineering.wework.com/choosing-a-language-stack-cac3726928f6)
	* [Switching from Go to Rust at Discord](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f)
	* [ASP.NET Core Performance Optimization at Agoda](https://medium.com/agoda-engineering/happy-asp-net-core-performance-optimization-4e21a383d299)
	* [Data Race Patterns in Go at Uber](https://eng.uber.com/data-race-patterns-in-go/)
	* [Java 21 Virtual Threads at Netflix](https://netflixtechblog.com/java-21-virtual-threads-dude-wheres-my-lock-3052540e231d)

# 二、 文献引文与出处来源

- **知识库出处**: [binhnguyennus/awesome-scalability (GitHub)](https://github.com/binhnguyennus/awesome-scalability)
- **专题分类**: 分布式系统与高并发架构实战文库 · 性能工程与延迟优化
