---
title: "引用文献: Zawinski's Law and Software Bloat: Why Photoshop Slows Down (扎温斯基定律与软件膨胀：为什么程序启动越来越慢)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://forums.adobe.com/thread/376152"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
---

〔本文档为黑客定律与工程哲学文库中《Photoshop 启动缓慢》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Zawinski's Law and Software Bloat: Why Photoshop Slows Down (扎温斯基定律与软件膨胀：为什么程序启动越来越慢)
- **原文链接**: [https://forums.adobe.com/thread/376152](https://forums.adobe.com/thread/376152)
- **引用锚文本**: Photoshop 启动缓慢
- **抓取状态**: success

# 二、 文献正文内容

# 软件膨胀与扎温斯基定律案例研究 (Zawinski's Law & Software Bloat)

出处：Adobe 开发者论坛与 Jamie Zawinski (Netscape 早期核心工程师) 经典讨论。

### 扎温斯基定律 (Zawinski's Law)

> “Every program attempts to expand until it can read mail. Those programs which cannot so expand are replaced by ones which can.”
> （每个程序都会试图膨胀，直到它能够收发电子邮件。那些无法如此膨胀的程序最终会被那些能够收发邮件的程序所取代。）

### 案例剖析：大型商业软件的启动瓶颈

在 Adobe Photoshop 社区的经典技术讨论中，用户长期探讨为何随着版本升级，启动速度越来越慢：
1. **插件与字体扫描瀑布流**：冷启动时串行加载成百上千个第三方滤镜、扩展脚本和字体引擎。
2. **云服务通信阻塞**：试图在主界面展示前同步云端字体、订阅许可和协作状态，违反了“本地优先”（Local-First）原则。
3. **第二系统效应与功能蔓延**：每一代产品经理都试图向软件中塞入更多边缘功能，导致核心路径的启动性能被不可避免地稀释。

---
_本地归档时间: 2026-09-05 | 来源: https://forums.adobe.com/thread/376152_
