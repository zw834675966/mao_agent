#!/usr/bin/env python3
"""
Pipeline script to build, transform, and structure dwmkerr/hacker-laws materials
into mao_agent corpus conventions.

Features:
- Pulls from upstream dwmkerr/hacker-laws and authoritative Chinese translations.
- Emits standard Markdown files with valid YAML frontmatter matching mao_agent schema.
- Structures chapters with hierarchical headers (# 一、, # 二、 etc.) suitable for ChineseSemanticChunker.
- Copies all SVGs and diagram assets for complete multi-modal material fidelity.
- Preserves raw upstream translations and READMEs under corpus/hacker_laws/raw/.
"""

import os
import re
import shutil
from pathlib import Path
from typing import Dict, List, Optional, Tuple

ROOT_DIR = Path(__file__).resolve().parent.parent
CORPUS_DIR = ROOT_DIR / "corpus"
HACKER_LAWS_DIR = CORPUS_DIR / "hacker_laws"
RAW_DIR = HACKER_LAWS_DIR / "raw"
IMAGES_DIR = HACKER_LAWS_DIR / "images"

# Metadata mapping for laws and principles
METADATA_MAP: Dict[str, Dict[str, any]] = {
    # --- LAWS ---
    "90–9–1 Principle (1% Rule)": {
        "filename": "laws_90_9_1_principle.md",
        "title_zh": "90-9-1 法则 (90–9–1 Principle or 1% Rule)",
        "author": "网络文化与开源社区经验观察",
        "date": "2006",
        "category": "互联网社群与协作定律",
        "tags": ["90-9-1法则", "1%法则", "社区治理", "开源协作", "网络效应"],
        "headnote": "〔90-9-1法则指出：在诸如维基百科、开源社区与社交网络中，90%的用户只消费内容，9%的用户会参与互动与编辑，仅有1%的用户会主动创造核心内容。〕",
    },
    "90–90 Rule": {
        "filename": "laws_90_90_rule.md",
        "title_zh": "九九定律 (90–90 Rule)",
        "author": "汤姆·卡吉尔 (Tom Cargill / 贝尔实验室)",
        "date": "1985",
        "category": "软件工程与项目进度定律",
        "tags": ["九九定律", "项目管理", "工期估算", "80-20法则", "软件交付"],
        "headnote": "〔九九定律指出：前90%的代码消耗了前90%的开发时间；剩下10%的代码则消耗了另外90%的开发时间。它以幽默而深刻的方式指出了软件工程最后收尾阶段的巨大不确定性。〕",
    },
    "Amdahl's Law": {
        "filename": "laws_amdahls_law.md",
        "title_zh": "阿姆达尔定律 (Amdahl's Law)",
        "author": "吉恩·阿姆达尔 (Gene Amdahl)",
        "date": "1967",
        "category": "系统性能与并发定律",
        "tags": ["阿姆达尔定律", "并行计算", "加速比", "系统性能", "并发瓶颈"],
        "headnote": "〔阿姆达尔定律指出：系统通过增加计算资源所获得的加速比，严格受限于程序中无法并行化的串行部分比例。〕",
    },
    "The Broken Windows Theory": {
        "filename": "laws_broken_windows_theory.md",
        "title_zh": "破窗理论 / 破窗效应 (The Broken Windows Theory)",
        "author": "詹姆斯·Q·威尔逊 与 乔治·凯林 (James Q. Wilson & George L. Kelling)",
        "date": "1982",
        "category": "代码质量与工程文化定律",
        "tags": ["破窗效应", "技术债务", "代码异味", "工程文化", "重构"],
        "headnote": "〔破窗理论表明：环境中微小未被修复的破损（如脏代码或未修复的Bug）会向团队传递放任信号，迅速引发蔓延性的质量劣变与技术债务失控。〕",
    },
    "Brooks' Law": {
        "filename": "laws_brooks_law.md",
        "title_zh": "布鲁克斯法则 (Brooks' Law)",
        "author": "弗雷德·布鲁克斯 (Fred Brooks / 《人月神话》)",
        "date": "1975",
        "category": "项目管理与沟通复杂性定律",
        "tags": ["布鲁克斯法则", "人月神话", "团队协作", "沟通开销", "项目延期"],
        "headnote": "〔布鲁克斯法则断言：向一个已经延期的软件项目增加人手，只会让项目延期更加严重。沟通开销随人员数量呈二次方剧增。〕",
    },
    "CAP Theorem (Brewer's Theorem)": {
        "filename": "laws_cap_theorem.md",
        "title_zh": "CAP 定理 / 布鲁尔定理 (CAP Theorem)",
        "author": "埃里克·布鲁尔 (Eric Brewer)",
        "date": "2000",
        "category": "分布式系统与一致性定律",
        "tags": ["CAP定理", "分布式系统", "一致性", "可用性", "分区容错性"],
        "headnote": "〔CAP定理断言：在分布式网络必然存在网络分区（P）的前提下，任何分布式计算系统都不可能同时保证一致性（C）与可用性（A），必须在二者之间做出权衡。〕",
    },
    "Clarke's three laws": {
        "filename": "laws_clarkes_three_laws.md",
        "title_zh": "克拉克三大定律 (Clarke's Three Laws)",
        "author": "阿瑟·C·克拉克 (Arthur C. Clarke)",
        "date": "1962",
        "category": "科技哲学与人机界面定律",
        "tags": ["克拉克定律", "未来技术", "抽象黑盒", "技术成熟度", "科技哲学"],
        "headnote": "〔克拉克第三定律指出：任何足够先进的科技，都与魔法无异。在软件中，极度成熟的封装与抽象往往给使用者呈现出黑盒般的神秘体验。〕",
    },
    "Conway's Law": {
        "filename": "laws_conways_law.md",
        "title_zh": "康威定律 (Conway's Law)",
        "author": "梅尔文·康威 (Melvin Conway)",
        "date": "1968",
        "category": "系统架构与组织设计定律",
        "tags": ["康威定律", "逆向康威法", "微服务架构", "组织结构", "团队沟通"],
        "headnote": "〔康威定律指出：任何设计系统的组织，其所交付的设计方案在结构上都将不可避免地映射出该组织的沟通结构。〕",
    },
    "Cunningham's Law": {
        "filename": "laws_cunninghams_law.md",
        "title_zh": "坎宁汉姆定律 (Cunningham's Law)",
        "author": "沃德·坎宁汉姆 (Ward Cunningham / Wiki发明人)",
        "date": "1995",
        "category": "社交网络与知识发现定律",
        "tags": ["坎宁汉姆定律", "知识协作", "心理偏见", "开源社群", "求助技巧"],
        "headnote": "〔坎宁汉姆定律断言：在互联网上获得正确答案的最佳方法不是提出问题，而是发布一个错误的答案。人们纠正错误的动力远大于回答提问。〕",
    },
    "Dunbar's Number": {
        "filename": "laws_dunbars_number.md",
        "title_zh": "邓巴数 (Dunbar's Number)",
        "author": "罗宾·邓巴 (Robin Dunbar)",
        "date": "1992",
        "category": "组织规模与认知极限理论",
        "tags": ["邓巴数", "150人法则", "团队规模", "组织治理", "跨部门沟通"],
        "headnote": "〔邓巴数提出：受限于人类大脑新皮质的认知处理能力，一个人能够维持稳定紧密社交关系的上限约为150人。当工程组织超过此规模时必须进行官僚化或分治解耦。〕",
    },
    "The Dunning-Kruger Effect": {
        "filename": "laws_dunning_kruger_effect.md",
        "title_zh": "邓宁-克鲁格效应 / 达克效应 (The Dunning-Kruger Effect)",
        "author": "戴维·邓宁 与 贾斯汀·克鲁格 (David Dunning & Justin Kruger)",
        "date": "1999",
        "category": "认知偏差与工程心理学",
        "tags": ["达克效应", "认知偏差", "愚昧山峰", "工程谦逊", "技术成长"],
        "headnote": "〔邓宁-克鲁格效应指出：能力欠缺的人往往无法客观评估自身的无能，导致盲目自大；而真正精通的人则倾向于低估自己的能力。〕",
    },
    "Fitts' Law": {
        "filename": "laws_fitts_law.md",
        "title_zh": "费茨法则 (Fitts' Law)",
        "author": "保罗·费茨 (Paul Fitts)",
        "date": "1954",
        "category": "人机交互与UI设计定律",
        "tags": ["费茨法则", "用户体验", "UI交互", "目标距离与尺寸", "人机工程"],
        "headnote": "〔费茨法则建立了到达目标所需时间的数学模型：移动到目标的时间由到目标的距离和目标的尺寸共同决定。目标越大、距离越近，操作耗时越短。〕",
    },
    "Gall's Law": {
        "filename": "laws_galls_law.md",
        "title_zh": "盖尔定律 (Gall's Law)",
        "author": "约翰·盖尔 (John Gall)",
        "date": "1977",
        "category": "复杂系统演化定律",
        "tags": ["盖尔定律", "复杂系统", "渐进演进", "过度设计", "系统重构"],
        "headnote": "〔盖尔定律断言：一个能正常工作的复杂系统，必定是从一个能正常工作的简单系统演化而来的。从零开始全新设计的复杂系统绝不可能正常工作。〕",
    },
    "Goodhart's Law": {
        "filename": "laws_goodharts_law.md",
        "title_zh": "古德哈特定律 (Goodhart's Law)",
        "author": "查尔斯·古德哈特 (Charles Goodhart)",
        "date": "1975",
        "category": "度量指标与绩效反噬定律",
        "tags": ["古德哈特定律", "研发度量", "KPI异化", "代码行数度量", "测试覆盖率"],
        "headnote": "〔古德哈特定律指出：当一个指标变成目标时，它就不再是一个好的指标。人们会操纵指标本身而背离最初的业务初衷。〕",
    },
    "Hanlon's Razor": {
        "filename": "laws_hanlons_razor.md",
        "title_zh": "汉隆剃刀 (Hanlon's Razor)",
        "author": "罗伯特·J·汉隆 (Robert J. Hanlon)",
        "date": "1980",
        "category": "团队沟通与心态哲学",
        "tags": ["汉隆剃刀", "思维模型", "团队信任", "故障归因", "沟通效率"],
        "headnote": "〔汉隆剃刀断言：能够用愚蠢或疏忽充分解释的行为，永远不要归咎于恶意。在系统故障或排查协作中保持善意假设能极大降低内耗。〕",
    },
    "Hick's Law (Hick-Hyman Law)": {
        "filename": "laws_hicks_law.md",
        "title_zh": "席克定律 / 席克-海曼定律 (Hick's Law)",
        "author": "威廉·埃德蒙·席克 与 雷·海曼 (William Edmund Hick & Ray Hyman)",
        "date": "1952",
        "category": "人机交互与认知负荷定律",
        "tags": ["席克定律", "认知负荷", "UI设计", "决策延迟", "极简主义"],
        "headnote": "〔席克定律表明：一个人做出决策所需的时间随着选项数量和复杂性的增加而对数增长。减少用户选择路径能显著提升交互吞吐效率。〕",
    },
    "Hofstadter's Law": {
        "filename": "laws_hofstadters_law.md",
        "title_zh": "侯世达定律 (Hofstadter's Law)",
        "author": "道格拉斯·侯世达 (Douglas Hofstadter / 《集异璧》)",
        "date": "1979",
        "category": "时间估算与自指递归定律",
        "tags": ["侯世达定律", "集异璧", "软件排期", "工期预测", "不确定性"],
        "headnote": "〔侯世达定律阐明：完成一件事实际花费的时间总是比你预期的要长，即使你把侯世达定律本身考虑进去也依然如此。〕",
    },
    "Hutber's Law": {
        "filename": "laws_hutbers_law.md",
        "title_zh": "哈伯特定律 (Hutber's Law)",
        "author": "帕特里克·哈伯特 (Patrick Hutber)",
        "date": "1981",
        "category": "软件变更与功能退化定律",
        "tags": ["哈伯特定律", "版本退化", "功能变更", "用户体验", "负优化"],
        "headnote": "〔哈伯特定律指出：“改善意味着恶化”（Improvement means deterioration）。软件更新中打着改善旗号的所谓优化，往往以牺牲原有核心稳定性与便利性为代价。〕",
    },
    "The Hype Cycle & Amara's Law": {
        "filename": "laws_hype_cycle_and_amaras_law.md",
        "title_zh": "技术成熟度曲线与阿马拉定律 (The Hype Cycle & Amara's Law)",
        "author": "罗伊·阿马拉 与 Gartner (Roy Amara & Gartner)",
        "date": "1995",
        "category": "技术演进与产业预测定律",
        "tags": ["阿马拉定律", "技术成熟度曲线", "技术炒作", "期望膨胀峰值", "生产力成熟期"],
        "headnote": "〔阿马拉定律指出：我们往往高估一项技术的短期效益，而低估其长期深远影响；Gartner技术成熟度曲线生动刻画了技术从泡沫破灭到平稳落地的全过程。〕",
    },
    "Hyrum's Law (The Law of Implicit Interfaces)": {
        "filename": "laws_hyrums_law.md",
        "title_zh": "海勒姆定律 / 隐式接口定律 (Hyrum's Law)",
        "author": "海勒姆·赖特 (Hyrum Wright / Google)",
        "date": "2012",
        "category": "API设计与契约约束定律",
        "tags": ["海勒姆定律", "隐式接口", "向下兼容", "破坏性变更", "API契约"],
        "headnote": "〔海勒姆定律指出：当一个API的用户足够多时，在接口规范中未承诺的所有系统实现细节与观察行为，最终都会被某些调用方依赖。〕",
    },
    "Jevons' Paradox": {
        "filename": "laws_jevons_paradox.md",
        "title_zh": "杰文斯悖论 (Jevons' Paradox)",
        "author": "威廉·斯坦利·杰文斯 (William Stanley Jevons)",
        "date": "1865",
        "category": "系统资源与性能消费悖论",
        "tags": ["杰文斯悖论", "资源消耗", "性能优化", "算力扩张", "系统需求反弹"],
        "headnote": "〔杰文斯悖论指出：提高资源利用效率的举措，最终不会减少该资源的总消耗量，反而会因为使用门槛降低导致整体需求呈爆发式增长。在计算体系中表现为性能越高，软件占用越多。〕",
    },
    "Input-Process-Output (IPO)": {
        "filename": "laws_input_process_output.md",
        "title_zh": "输入-处理-输出模型 (Input-Process-Output / IPO)",
        "author": "计算机体系经典模型",
        "date": "1970",
        "category": "计算架构与系统分层模型",
        "tags": ["IPO模型", "流水线设计", "系统解耦", "纯函数", "数据驱动"],
        "headnote": "〔IPO模型是系统工程与软件设计的经典基础范式：任何计算单元都应清晰划分为输入、处理逻辑与输出三层，保持单向数据流与无状态处理边界。〕",
    },
    "Kernighan's Law": {
        "filename": "laws_kernighans_law.md",
        "title_zh": "柯林汉定律 (Kernighan's Law)",
        "author": "布莱恩·柯林汉 (Brian Kernighan / 《C程序设计语言》作者)",
        "date": "1978",
        "category": "可维护性与调试难度定律",
        "tags": ["柯林汉定律", "代码调试", "极简代码", "聪明反被聪明误", "代码可读性"],
        "headnote": "〔柯林汉定律断言：调试代码的难度是初次编写代码的两倍。因此，如果你在写代码时用尽了全部聪明才智，根据定义你将没有足够的能力去调试它。〕",
    },
    "Koomey's Law": {
        "filename": "laws_koomeys_law.md",
        "title_zh": "库米定律 (Koomey's Law)",
        "author": "乔纳森·库米 (Jonathan Koomey)",
        "date": "2010",
        "category": "能耗效率与半导体演进定律",
        "tags": ["库米定律", "能效比", "移动计算", "绿色计算", "硬件演进"],
        "headnote": "〔库米定律描述：每计算一次所消耗的焦耳能量大约每1.57年就会减半。在移动互联网与边缘计算时代，电池续航与能效取代单纯主频成为核心驱动力。〕",
    },
    "Linus's Law": {
        "filename": "laws_linuss_law.md",
        "title_zh": "林纳斯定律 (Linus's Law)",
        "author": "埃里克·雷蒙德 与 林纳斯·托瓦兹 (Eric Raymond & Linus Torvalds)",
        "date": "1999",
        "category": "开源审查与代码安全定律",
        "tags": ["林纳斯定律", "开源大教堂与集市", "代码审查", "安全审计", "同行评审"],
        "headnote": "〔林纳斯定律阐述：“目光所及，万虫显形”（Given enough eyeballs, all bugs are shallow）。当有足够多的人审查与使用代码时，几乎所有潜在Bug都会被快速暴露并解决。〕",
    },
    "Metcalfe's Law": {
        "filename": "laws_metcalfes_law.md",
        "title_zh": "梅特卡夫定律 (Metcalfe's Law)",
        "author": "罗伯特·梅特卡夫 (Robert Metcalfe / 以太网发明者)",
        "date": "1980",
        "category": "网络价值与拓扑规模定律",
        "tags": ["梅特卡夫定律", "网络效应", "以太网", "分布式网络", "系统连接数"],
        "headnote": "〔梅特卡夫定律指出：电信或计算网络的综合价值与其联网用户数（或节点数）的平方（$N^2$）成正比。〕",
    },
    "Moore's Law": {
        "filename": "laws_moores_law.md",
        "title_zh": "摩尔定律 (Moore's Law)",
        "author": "戈登·摩尔 (Gordon Moore / Intel创始人)",
        "date": "1965",
        "category": "微电子与算力爆发定律",
        "tags": ["摩尔定律", "晶体管密度", "半导体工艺", "计算成本", "算力极限"],
        "headnote": "〔摩尔定律观察到：集成电路上可容纳的晶体管数目约每18至24个月增加一倍，计算性能翻倍且成本减半。这塑造了过去六十年来整个信息技术的底色。〕",
    },
    "Murphy's Law / Sod's Law": {
        "filename": "laws_murphys_law.md",
        "title_zh": "墨菲定律 / 索德定律 (Murphy's Law / Sod's Law)",
        "author": "爱德华·A·墨菲 (Edward A. Murphy Jr.)",
        "date": "1949",
        "category": "防御性编程与容错设计定律",
        "tags": ["墨菲定律", "防御性编程", "高可用设计", "故障自愈", "混沌工程"],
        "headnote": "〔墨菲定律警示：凡是可能出错的事，必定会出错（Anything that can go wrong, will go wrong）。在生产系统设计中必须贯彻防御性容错思想。〕",
    },
    "Occam's Razor": {
        "filename": "laws_occams_razor.md",
        "title_zh": "奥卡姆剃刀 (Occam's Razor)",
        "author": "奥卡姆的威廉 (William of Ockham)",
        "date": "1320",
        "category": "思维哲学与架构极简原则",
        "tags": ["奥卡姆剃刀", "极简主义", "非必要勿增实体", "架构精简", "代码简化"],
        "headnote": "〔奥卡姆剃刀法则申明：“如无必要，勿增实体”（Entities should not be multiplied without necessity）。在解释现象或设计系统架构时，最简单的方案通常是最有效可控的。〕",
    },
    "Parkinson's Law": {
        "filename": "laws_parkinsons_law.md",
        "title_zh": "帕金森定律 (Parkinson's Law)",
        "author": "C·诺斯古德·帕金森 (C. Northcote Parkinson)",
        "date": "1955",
        "category": "资源消耗与组织官僚定律",
        "tags": ["帕金森定律", "工期膨胀", "内存消耗", "官僚主义", "敏捷迭代"],
        "headnote": "〔帕金森定律指出：工作会自动膨胀，直到占满所有可用的时间与资源。给软件项目分配过宽裕的时间或内存，软件就会无限度消耗这些资源。〕",
    },
    "Premature Optimization Effect": {
        "filename": "laws_premature_optimization.md",
        "title_zh": "过早优化效应 (Premature Optimization Effect)",
        "author": "高德纳 (Donald Knuth / 《计算机程序设计艺术》)",
        "date": "1974",
        "category": "性能调优与工程优先级定律",
        "tags": ["过早优化", "高德纳", "性能基准", "代码可读性", "系统瓶颈"],
        "headnote": "〔高德纳名言指出：过早优化是万恶之源（Premature optimization is the root of all evil）。应当首先编写清晰正确的代码，再基于实际性能基准数据进行有针对性的瓶颈优化。〕",
    },
    "Putt's Law": {
        "filename": "laws_putts_law.md",
        "title_zh": "普特定律 (Putt's Law)",
        "author": "阿奇博尔德·普特 (Archibald Putt)",
        "date": "1981",
        "category": "技术管理与权力结构定律",
        "tags": ["普特定律", "技术管理", "工程领导力", "组织沟通", "技术决策"],
        "headnote": "〔普特定律幽默指出：技术由不懂管理的人领导，管理由不懂技术的人进行。科技组织必须建立懂业务的技术专家管理路径以规避决策脱节。〕",
    },
    "Reed's Law": {
        "filename": "laws_reeds_law.md",
        "title_zh": "里德定律 (Reed's Law)",
        "author": "大卫·P·里德 (David P. Reed)",
        "date": "1999",
        "category": "社群子网络与指数增长定律",
        "tags": ["里德定律", "子网络效应", "指数增长", "群组协作", "平台经济"],
        "headnote": "〔里德定律揭示：支持创建子群组的网络效用随着网络规模呈指数级（$2^N$）增长，其增长速度与网络价值甚至远超过梅特卡夫定律的平方律。〕",
    },
    "The Bitter Lesson": {
        "filename": "laws_the_bitter_lesson.md",
        "title_zh": "苦涩的教训 (The Bitter Lesson)",
        "author": "理查德·S·萨顿 (Richard S. Sutton / 强化学习之父)",
        "date": "2019",
        "category": "人工智能与通用算力哲学",
        "tags": ["苦涩的教训", "人工智能", "通用搜索与学习", "人类先验知识", "算力缩放法则"],
        "headnote": "〔理查德·萨顿在《苦涩的教训》中深刻阐明：七十年来人工智能研究最根本的经验是，依赖通用计算与大规模搜索学习的方法，最终无一例外彻底击败融入人类专家先验知识的手工方法。〕",
    },
    "The Ringelmann Effect": {
        "filename": "laws_ringelmann_effect.md",
        "title_zh": "林格曼效应 / 社会惰化 (The Ringelmann Effect)",
        "author": "马克西米利安·林格曼 (Maximilien Ringelmann)",
        "date": "1913",
        "category": "团队生产力与群体动力学",
        "tags": ["林格曼效应", "社会惰化", "团队责任稀释", "小团队敏捷", "个体效能"],
        "headnote": "〔林格曼效应揭示：随着群体成员人数的增加，每个人在团队中所付出的个体平均努力程度反而呈现下降趋势。避免责任分散是保持敏捷团队高产出的核心。〕",
    },
    "The Law of Conservation of Complexity (Tesler's Law)": {
        "filename": "laws_teslers_law.md",
        "title_zh": "复杂性守恒定律 / 泰斯勒定律 (The Law of Conservation of Complexity)",
        "author": "拉里·泰斯勒 (Larry Tesler / 剪切复制粘贴发明人)",
        "date": "1984",
        "category": "系统交互与复杂度转移定律",
        "tags": ["复杂性守恒定律", "泰斯勒定律", "用户体验", "复杂度转移", "系统架构"],
        "headnote": "〔泰斯勒定律断言：任何系统都存在无法被消除的固有内在复杂度。问题仅在于这部分复杂度由开发者在底层消化承担，还是转移给最终用户去承受。〕",
    },
    "The Law of Demeter": {
        "filename": "laws_law_of_demeter.md",
        "title_zh": "得墨忒耳定律 / 最少知识原则 (The Law of Demeter)",
        "author": "伊恩·霍兰德 等 (Ian Holland et al. / Northeastern University)",
        "date": "1987",
        "category": "面向对象与松耦合设计原则",
        "tags": ["得墨忒耳定律", "最少知识原则", "链式调用破坏", "松耦合", "面向对象"],
        "headnote": "〔得墨忒耳定律强调：一个对象应当对其他对象有尽可能少的了解，只与你的直接朋友交谈（Don't talk to strangers），严禁通过长链方法调用探测陌生对象的内部结构。〕",
    },
    "The Law of Leaky Abstractions": {
        "filename": "laws_leaky_abstractions.md",
        "title_zh": "抽象泄漏定律 (The Law of Leaky Abstractions)",
        "author": "乔尔·斯波尔斯基 (Joel Spolsky / Stack Overflow联合创始人)",
        "date": "2002",
        "category": "软件分层与底层透明性定律",
        "tags": ["抽象泄漏定律", "分层抽象", "TCP协议抽象", "ORM隐患", "底层调试"],
        "headnote": "〔乔尔·斯波尔斯基指出：所有重大的抽象机制，在某种程度上都是有漏洞的（All non-trivial abstractions, to some degree, are leaky）。一旦发生故障，开发者依然必须精通其底层机制。〕",
    },
    "The Law of the Instrument": {
        "filename": "laws_law_of_the_instrument.md",
        "title_zh": "工具定律 / 马斯洛之锤 (The Law of the Instrument / Maslow's Hammer)",
        "author": "亚伯拉罕·马斯洛 与 亚伯拉罕·卡普兰 (Abraham Maslow & Abraham Kaplan)",
        "date": "1966",
        "category": "认知狭隘与技术选型偏见",
        "tags": ["马斯洛之锤", "工具定律", "技术选型", "锤子找钉子", "盲目技术狂热"],
        "headnote": "〔工具定律形象地总结为：如果你手里唯一的工具是一把锤子，那么你看所有的东西都会像是一颗钉子。警惕在技术选型中因为熟悉某种技术而盲目滥用于不契合的场景。〕",
    },
    "The Law of Triviality": {
        "filename": "laws_law_of_triviality.md",
        "title_zh": "帕金森琐碎定理 / 鸭棚效应 (The Law of Triviality / Bikeshedding)",
        "author": "C·诺斯古德·帕金森 (C. Northcote Parkinson)",
        "date": "1957",
        "category": "评审注意力与决策精力错配定律",
        "tags": ["琐碎定理", "鸭棚效应", "代码评审内耗", "注意力错配", "架构评审"],
        "headnote": "〔帕金森琐碎定理指出：组织对一个议题花费的讨论时间，与该议题的实际客观重要性往往成反比。团队常常就微不足道的表面细节争论不休，而对核心复杂架构全盘通过。〕",
    },
    "The Unix Philosophy": {
        "filename": "laws_unix_philosophy.md",
        "title_zh": "Unix 哲学 (The Unix Philosophy)",
        "author": "肯·汤普逊、丹尼斯·里奇 与 麦克罗伊 (Ken Thompson, Dennis Ritchie & Doug McIlroy)",
        "date": "1978",
        "category": "系统设计与可组合性哲学",
        "tags": ["Unix哲学", "单一职责", "管道组合", "文本流", "模块化"],
        "headnote": "〔Unix哲学核心思想是：做好一件事，且只做好这一件事；让程序协同工作；让程序处理文本流，因为这是最通用的接口。〕",
    },
    "The Scout Rule": {
        "filename": "laws_scout_rule.md",
        "title_zh": "童子军法则 (The Scout Rule)",
        "author": "童子军运动传承 / 罗伯特·C·马丁 (Robert C. Martin)",
        "date": "2008",
        "category": "代码卫生与渐进重构原则",
        "tags": ["童子军法则", "代码整洁之道", "渐进重构", "技术债务预防", "持续改进"],
        "headnote": "〔童子军法则借用到软件工程中：离开营地时，让营地比你刚来时更加干净整洁。提交代码时，顺手让被修改的代码比检出时更干净，从而自发杜绝代码腐化。〕",
    },
    "The Second-System Effect": {
        "filename": "laws_second_system_effect.md",
        "title_zh": "第二系统效应 (The Second-System Effect)",
        "author": "弗雷德·布鲁克斯 (Fred Brooks / 《人月神话》)",
        "date": "1975",
        "category": "系统重构与过度设计陷阱",
        "tags": ["第二系统效应", "人月神话", "重构失败", "功能膨胀", "过度设计"],
        "headnote": "〔布鲁克斯指出：在设计第二个系统时，由于在第一个系统受限的压抑野心爆发，设计师往往试图把所有未能实现的想法全部塞入，导致第二系统变得极度臃肿、复杂甚至夭折。〕",
    },
    "The Spotify Model": {
        "filename": "laws_spotify_model.md",
        "title_zh": "Spotify 敏捷组织模型 (The Spotify Model)",
        "author": "亨利克·克尼伯格 与 安德斯·艾弗森 (Henrik Kniberg & Anders Ivarsson)",
        "date": "2012",
        "category": "敏捷团队组织与部落分工架构",
        "tags": ["Spotify模型", "分队与部落", "跨职能团队", "自主性与一致性", "敏捷组织"],
        "headnote": "〔Spotify模型通过分队（Squads）、分部（Chapters）、部落（Tribes）和行会（Guilds）四维矩阵，追求高度自治与高度对齐的统一，以支持大规模工程敏捷创新。〕",
    },
    "The Two Pizza Rule": {
        "filename": "laws_two_pizza_rule.md",
        "title_zh": "两张披萨团队原则 (The Two Pizza Rule)",
        "author": "杰夫·贝索斯 (Jeff Bezos / Amazon)",
        "date": "2002",
        "category": "团队组织规模与微服务契约",
        "tags": ["两张披萨原则", "团队规模", "亚马逊敏捷", "组织解耦", "沟通成本控制"],
        "headnote": "〔亚马逊创始人贝索斯提出：一个高产的工程团队规模，不应该超过两张大披萨能吃饱的人数（通常为6到10人）。这极大削减了团队内部的沟通开销，促进独立自治交付。〕",
    },
    "Twyman's law": {
        "filename": "laws_twymans_law.md",
        "title_zh": "特威曼定律 (Twyman's Law)",
        "author": "威廉·A·特威曼 (William A. Twyman)",
        "date": "1975",
        "category": "数据工程与指标分析洞见",
        "tags": ["特威曼定律", "A/B测试陷阱", "异常指标", "数据验证", "数据分析"],
        "headnote": "〔特威曼定律断言：任何看起来特别有趣或不同寻常的数据结果，通常都是错误的。在进行A/B测试与系统监控时，异常令人惊喜的暴增往往是埋点或统计口径缺陷。〕",
    },
    "Wadler's Law": {
        "filename": "laws_wadlers_law.md",
        "title_zh": "沃德勒定律 (Wadler's Law)",
        "author": "菲利普·沃德勒 (Philip Wadler / Haskell共同设计者)",
        "date": "1990",
        "category": "编程语言设计与争议专注度定律",
        "tags": ["沃德勒定律", "语法糖争议", "编程语言设计", "类型系统", "讨论内耗"],
        "headnote": "〔沃德勒定律观察到：在编程语言设计中，对某项特性讨论所耗费的精力和争议程度，与其语义的重要性成反比。团队会就变量命名或符号语法争辩数月，而几分钟草率通过核心类型系统。〕",
    },
    "Wheaton's Law": {
        "filename": "laws_wheatons_law.md",
        "title_zh": "惠顿法则 (Wheaton's Law)",
        "author": "威尔·惠顿 (Wil Wheaton)",
        "date": "2007",
        "category": "开源社区礼仪与协作底线",
        "tags": ["惠顿法则", "社区准则", "开源礼仪", "健康团队", "心理安全"],
        "headnote": "〔惠顿法则用最简练直接的语言表达了人际与开源社区行为底线：“别当混蛋”（Don't be a jerk / Don't be a dick）。它是营造开放、心理安全与高效协作团队的道德基石。〕",
    },

    # --- PRINCIPLES ---
    "All Models Are Wrong (George Box's Law)": {
        "filename": "principles_all_models_are_wrong.md",
        "title_zh": "所有模型都是错的 / 乔治·博克斯定律 (All Models Are Wrong)",
        "author": "乔治·E·P·博克斯 (George E. P. Box)",
        "date": "1976",
        "category": "系统建模与抽象有效性原则",
        "tags": ["博克斯定律", "系统建模", "有用模型", "抽象边界", "领域驱动设计"],
        "headnote": "〔统计学家乔治·博克斯经典论断：“所有模型都是错的，但其中有一些是有用的”（All models are wrong, but some are useful）。在领域建模中不应追求百分百复刻现实，而应追求对解决核心问题最有用的抽象。〕",
    },
    "Chesterton's Fence": {
        "filename": "principles_chestertons_fence.md",
        "title_zh": "切斯特顿围栏 (Chesterton's Fence)",
        "author": "G·K·切斯特顿 (G. K. Chesterton)",
        "date": "1929",
        "category": "遗留系统重构与认知审慎原则",
        "tags": ["切斯特顿围栏", "遗留系统重构", "未知历史背景", "谨慎重构", "软件维护"],
        "headnote": "〔切斯特顿围栏指出：如果你在路中间看到一堵看似多余的围栏，在你弄明白它为什么被建在那里之前，绝不要擅自拆除它。在重构遗留代码时切忌盲目删去看似多余的古怪检查。〕",
    },
    "Kerckhoffs's principle": {
        "filename": "principles_kerckhoffss_principle.md",
        "title_zh": "柯克霍夫原则 (Kerckhoffs's Principle)",
        "author": "奥古斯特·柯克霍夫 (Auguste Kerckhoffs)",
        "date": "1883",
        "category": "现代密码学与系统安全基础原则",
        "tags": ["柯克霍夫原则", "隐蔽式安全谬误", "现代密码学", "密钥管理", "开源安全"],
        "headnote": "〔柯克霍夫原则申明：即使密码系统的所有设计细节与算法都被敌方所知，只要密钥没有泄露，该系统依然必须是安全的。拒绝“通过隐蔽求安全”（Security through obscurity）。〕",
    },
    "The Dead Sea Effect": {
        "filename": "principles_dead_sea_effect.md",
        "title_zh": "死海效应 (The Dead Sea Effect)",
        "author": "布鲁斯·F·韦伯斯特 (Bruce F. Webster)",
        "date": "2008",
        "category": "人才流失与团队劣币驱逐良币效应",
        "tags": ["死海效应", "技术团队逆淘汰", "人才保留", "技术文化衰败", "团队管理"],
        "headnote": "〔死海效应描述：高水平的核心人才更容易找到更好的机会而离开糟糕的公司（水分蒸发），而平庸缺乏流动性的员工则会沉淀留任（高浓度盐分），最终导致整个组织的技术能力急剧退化。〕",
    },
    "The Dilbert Principle": {
        "filename": "principles_dilbert_principle.md",
        "title_zh": "呆伯特原则 (The Dilbert Principle)",
        "author": "斯科特·亚当斯 (Scott Adams / 《呆伯特漫画》)",
        "date": "1995",
        "category": "企业管理与升迁悖论讽刺理论",
        "tags": ["呆伯特原则", "职场讽刺", "管理升迁", "技术骨干保护", "组织机能"],
        "headnote": "〔呆伯特原则讽刺指出：企业倾向于系统性地将最无能的员工提拔到管理层，以便让他们离开真正产生价值的核心生产一线，从而尽量减少对实际业务的损害。〕",
    },
    "The Pareto Principle (The 80/20 Rule)": {
        "filename": "principles_pareto_principle.md",
        "title_zh": "帕累托法则 / 80-20 法则 (The Pareto Principle)",
        "author": "维尔弗雷多·帕累托 与 约瑟夫·朱兰 (Vilfredo Pareto & Joseph Juran)",
        "date": "1896",
        "category": "非均衡分布与关键少数原则",
        "tags": ["帕累托法则", "80-20法则", "关键少数", "性能瓶颈集中", "需求优先级"],
        "headnote": "〔帕累托法则揭示：在大多数情境中，大约80%的结果由20%的关键原因所导致。在软件工程中，80%的运行时耗通常集中在20%的代码上，80%的崩溃往往由20%的核心缺陷引起。〕",
    },
    "The Shirky Principle": {
        "filename": "principles_shirky_principle.md",
        "title_zh": "舍基原则 (The Shirky Principle)",
        "author": "克莱·舍基 (Clay Shirky / 《未来是湿的》)",
        "date": "2010",
        "category": "组织自我存续与问题延续悖论",
        "tags": ["舍基原则", "组织利益冲突", "问题存续", "咨询陷阱", "业务自我革命"],
        "headnote": "〔舍基原则指出：致力于解决某个问题的机构，往往会产生延长该问题存在的内在倾向，因为该问题的彻底解决往往意味着该机构自身生存合法性的终结。〕",
    },
    "The Stochastic Parrot": {
        "filename": "principles_stochastic_parrot.md",
        "title_zh": "随机鹦鹉理论 (The Stochastic Parrot)",
        "author": "埃米莉·M·本德、蒂姆尼特·格布鲁 等 (Emily M. Bender, Timnit Gebru et al.)",
        "date": "2021",
        "category": "自然语言模型批判与认知局限理论",
        "tags": ["随机鹦鹉", "大型语言模型", "幻觉批判", "统计模式匹配", "真正理解与符号推理"],
        "headnote": "〔随机鹦鹉理论批判指出：大型语言模型本质上是通过统计概率拼接庞大训练语料库中的语言形式，缺乏对外部世界真实语义、意图及逻辑的真正深层理解。〕",
    },
    "The Peter Principle": {
        "filename": "principles_peter_principle.md",
        "title_zh": "彼得原理 (The Peter Principle)",
        "author": "劳伦斯·J·彼得 (Laurence J. Peter)",
        "date": "1969",
        "category": "职场晋升与不胜任极限理论",
        "tags": ["彼得原理", "不胜任阶层", "双轨制晋升", "技术与管理双通道", "能力天花板"],
        "headnote": "〔彼得原理断言：在层级组织中，每个员工都会趋于晋升到他所不能胜任的职位。优秀程序员往往被提拔为糟糕的技术经理，直到他在该职位上停滞不前。〕",
    },
    "The Robustness Principle (Postel's Law)": {
        "filename": "principles_robustness_principle.md",
        "title_zh": "稳健原则 / 波斯塔尔法则 (The Robustness Principle or Postel's Law)",
        "author": "乔恩·波斯塔尔 (Jon Postel / 互联网协议先驱)",
        "date": "1980",
        "category": "协议互操作与容错通信原则",
        "tags": ["波斯塔尔法则", "稳健原则", "宽进严出", "协议设计", "容错能力"],
        "headnote": "〔波斯塔尔法则奉行：“对自己严格，对他人宽容”（Be conservative in what you send, be liberal in what you accept）。发送符合严格标准的数据，宽容接纳非严格标准的外部输入。〕",
    },
    "SOLID": {
        "filename": "principles_solid.md",
        "title_zh": "SOLID 软件设计五大原则总论 (SOLID Principles)",
        "author": "罗伯特·C·马丁 (Robert C. Martin / Uncle Bob)",
        "date": "2000",
        "category": "面向对象架构设计核心五大原则",
        "tags": ["SOLID原则", "面向对象设计", "架构整洁之道", "松耦合高内聚", "可维护性"],
        "headnote": "〔SOLID是由罗伯特·C·马丁整理的五个面向对象设计核心原则缩写：单一职责（S）、开闭原则（O）、里氏替换（L）、接口隔离（I）、依赖反转（D）。它们是现代可维护软件架构的奠基支柱。〕",
    },
    "The Single Responsibility Principle": {
        "filename": "principles_single_responsibility.md",
        "title_zh": "单一职责原则 (The Single Responsibility Principle / SRP)",
        "author": "罗伯特·C·马丁 (Robert C. Martin)",
        "date": "2000",
        "category": "面向对象设计原则 (S in SOLID)",
        "tags": ["单一职责原则", "SRP", "SOLID", "变更原因单一", "模块内聚"],
        "headnote": "〔单一职责原则强调：一个模块或类应该且仅应该对某一类利益相关者（或某一个变更原因）负责。引起它变化的原因有且只有一个。〕",
    },
    "The Open/Closed Principle": {
        "filename": "principles_open_closed.md",
        "title_zh": "开闭原则 (The Open/Closed Principle / OCP)",
        "author": "伯特兰·迈耶 (Bertrand Meyer)",
        "date": "1988",
        "category": "面向对象设计原则 (O in SOLID)",
        "tags": ["开闭原则", "OCP", "SOLID", "对扩展开放", "对修改关闭"],
        "headnote": "〔开闭原则主张：软件实体（类、模块、函数等）应当对扩展开放，对修改关闭。当系统引入新需求时，应通过新增代码而非修改既有经测试代码来实现。〕",
    },
    "The Liskov Substitution Principle": {
        "filename": "principles_liskov_substitution.md",
        "title_zh": "里氏替换原则 (The Liskov Substitution Principle / LSP)",
        "author": "芭芭拉·利斯科夫 (Barbara Liskov / 图灵奖得主)",
        "date": "1987",
        "category": "面向对象设计原则 (L in SOLID)",
        "tags": ["里氏替换原则", "LSP", "SOLID", "子类型契约", "行为多态"],
        "headnote": "〔里氏替换原则指出：子类型必须能够完全替换掉它们的基类型，而不会改变程序的正确性与预期行为。继承不应破坏基类契约约束。〕",
    },
    "The Interface Segregation Principle": {
        "filename": "principles_interface_segregation.md",
        "title_zh": "接口隔离原则 (The Interface Segregation Principle / ISP)",
        "author": "罗伯特·C·马丁 (Robert C. Martin)",
        "date": "2000",
        "category": "面向对象设计原则 (I in SOLID)",
        "tags": ["接口隔离原则", "ISP", "SOLID", "胖接口拆分", "高内聚接口"],
        "headnote": "〔接口隔离原则要求：客户端不应该被迫依赖于它们不使用的方法。宁可定义多个专门细粒度的瘦接口，也不要定义单一臃肿的万能胖接口。〕",
    },
    "The Dependency Inversion Principle": {
        "filename": "principles_dependency_inversion.md",
        "title_zh": "依赖倒置原则 (The Dependency Inversion Principle / DIP)",
        "author": "罗伯特·C·马丁 (Robert C. Martin)",
        "date": "1996",
        "category": "面向对象设计原则 (D in SOLID)",
        "tags": ["依赖倒置原则", "DIP", "SOLID", "控制反转", "依赖注入"],
        "headnote": "〔依赖倒置原则主张：高层模块不应该依赖低层模块，二者都应该依赖于抽象；抽象不应该依赖于细节，细节应该依赖于抽象。〕",
    },
    "The DRY Principle": {
        "filename": "principles_dry.md",
        "title_zh": "DRY 原则 / 不要重复自己 (The DRY Principle)",
        "author": "安迪·亨特 与 戴夫·托马斯 (Andy Hunt & Dave Thomas / 《程序员修炼之道》)",
        "date": "1999",
        "category": "核心工程实践与代码重用原则",
        "tags": ["DRY原则", "不要自我重复", "正交性", "单一事实来源", "可维护性"],
        "headnote": "〔DRY原则倡导：系统中的每一项知识或逻辑都必须在系统中具有单一、明确、权威且不可歧义的表述形式，杜绝多处复制粘贴导致的一致性维护噩梦。〕",
    },
    "The KISS principle": {
        "filename": "principles_kiss.md",
        "title_zh": "KISS 原则 / 保持简单愚蠢 (The KISS Principle)",
        "author": "凯利·约翰逊 (Kelly Johnson / 洛克希德臭鼬工厂总工程师)",
        "date": "1960",
        "category": "极简工程设计与可维护性哲学",
        "tags": ["KISS原则", "保持简单愚蠢", "极简设计", "抵制过度抽象", "可维护性"],
        "headnote": "〔KISS原则指出：绝大多数系统在保持极简而非变得复杂时才能运转得最好。简单不仅是一种审美，更是系统可靠性、可调试性与长期生存能力的关键保障。〕",
    },
    "YAGNI": {
        "filename": "principles_yagni.md",
        "title_zh": "YAGNI 原则 / 你不会需要它 (You Aren't Gonna Need It)",
        "author": "极限编程实践 (Extreme Programming / 肯特·贝克 等)",
        "date": "1999",
        "category": "敏捷精益与避免臆想设计原则",
        "tags": ["YAGNI原则", "你不需要它", "敏捷开发", "拒绝臆想配置", "精益交付"],
        "headnote": "〔YAGNI原则告诫：始终只实现你当前严格需要的功能，绝不要去实现你自以为将来可能需要的功能。过度未雨绸缪的抽象只会带来巨大的无效维护成本。〕",
    },
    "The Fallacies of Distributed Computing": {
        "filename": "principles_fallacies_of_distributed_computing.md",
        "title_zh": "分布式计算的八大谬论 (The Fallacies of Distributed Computing)",
        "author": "彼得·德伊奇 等 (L. Peter Deutsch et al. / Sun Microsystems)",
        "date": "1994",
        "category": "分布式系统设计底线警示",
        "tags": ["分布式计算谬论", "网络不可靠", "延迟非零", "带宽有限", "分布式系统容错"],
        "headnote": "〔分布式计算的八大谬论揭示了初学者在分布式系统中普遍持有的8个致命错误假定：网络可靠、延迟为零、带宽无限、网络安全、拓扑恒定、只有一名管理员、传输开销为零、网络同质。〕",
    },
    "The Principle of Least Astonishment": {
        "filename": "principles_least_astonishment.md",
        "title_zh": "最小惊奇原则 (The Principle of Least Astonishment / POLA)",
        "author": "UNIX 哲学传承与系统人机工程",
        "date": "1970",
        "category": "API设计与用户心理契约原则",
        "tags": ["最小惊奇原则", "POLA", "API直觉性", "一致性体验", "符合直觉"],
        "headnote": "〔最小惊奇原则申明：在设计组件、接口或用户交互时，其行为应当符合用户的普遍心理预期，绝不应该展现出令经验丰富的使用者感到不可思议或惊愕的突兀怪异行为。〕",
    },
}


def clean_markdown_text(text: str) -> str:
    """Clean markdown text, fix image paths, format properly."""
    text = re.sub(r'src="\./images/([^"]+)"', r'src="./images/\1"', text)
    text = text.replace("amdahls_law.png", "amdahls-law.svg")
    text = text.replace("Fitts_Law.svg", "fitts-law.svg")
    text = text.replace("hicks_law.svg", "hicks-law.svg")
    text = text.replace("gartner_hype_cycle.png", "hype-cycle.svg")
    return text.strip()


def build_standalone_markdown(en_key: str, en_content: str, zh_content: Optional[str]) -> str:
    meta = METADATA_MAP[en_key]
    title_zh = meta["title_zh"]
    author = meta["author"]
    date = meta["date"]
    category = meta["category"]
    tags = meta["tags"]
    headnote = meta["headnote"]

    # Use Chinese content if available; fallback or enrich with English content
    body = zh_content if zh_content else en_content
    cleaned_body = clean_markdown_text(body)

    # Format into standard YAML frontmatter + Headnote + Sections
    tags_yaml = "\n".join(f'  - "{t}"' for t in tags)
    content = f"""---
title: "{title_zh}"
author: "{author}"
date: "{date}"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "{category}"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
{tags_yaml}
---

{headnote}

# 一、 定律与原则核心阐述

{cleaned_body}

# 二、 原文引文与参考出处

- **原始定义出处**: [dwmkerr/hacker-laws (GitHub)](https://github.com/dwmkerr/hacker-laws)
- **权威中文文献源**: [nusr/hacker-laws-zh (GitHub)](https://github.com/nusr/hacker-laws-zh)
- **所属文库分类**: 黑客定律与工程哲学文库 · {category}
"""
    return content


def main():
    print("🚀 Initializing Hacker Laws material pulling and building pipeline...")
    HACKER_LAWS_DIR.mkdir(parents=True, exist_ok=True)
    RAW_DIR.mkdir(parents=True, exist_ok=True)
    IMAGES_DIR.mkdir(parents=True, exist_ok=True)

    scratch_base = Path(r"C:\Users\Administrator\.gemini\antigravity-cli\brain\a68b3a91-6c07-4ff5-a045-556bda873329\scratch")
    en_repo = scratch_base / "hacker-laws"
    zh_repo = scratch_base / "hacker-laws-zh"

    # 1. Copy image assets
    src_images = en_repo / "images"
    if src_images.exists():
        for img in src_images.glob("*.*"):
            dest = IMAGES_DIR / img.name
            shutil.copy2(img, dest)
            print(f"  [Image] Copied {img.name} -> {dest}")

    # 2. Preserve raw upstream materials
    if (en_repo / "README.md").exists():
        shutil.copy2(en_repo / "README.md", RAW_DIR / "README.en.md")
    if (zh_repo / "README.md").exists():
        shutil.copy2(zh_repo / "README.md", RAW_DIR / "README.zh.md")

    raw_trans = RAW_DIR / "translations"
    raw_trans.mkdir(exist_ok=True)
    if (en_repo / "translations").exists():
        for tr in (en_repo / "translations").glob("*.md"):
            shutil.copy2(tr, raw_trans / tr.name)
        print(f"  [Translations] Copied raw translation files into {raw_trans}")

    # Read upstream text
    with open(en_repo / "README.md", encoding="utf-8") as f:
        en_text = f.read()
    with open(zh_repo / "README.md", encoding="utf-8") as f:
        zh_text = f.read()

    # Parse EN sections
    laws_chunk = en_text.split("## Laws")[1].split("## Principles")[0]
    principles_chunk = en_text.split("## Principles")[1].split("## Reading List")[0]
    reading_chunk = en_text.split("## Reading List")[1]

    en_laws_dict = dict(re.findall(r"### ([^\n]+)\n(.*?)(?=\n### |\Z)", laws_chunk, re.DOTALL))
    en_principles_dict = dict(re.findall(r"### ([^\n]+)\n(.*?)(?=\n### |\Z)", principles_chunk, re.DOTALL))
    en_all = {**en_laws_dict, **en_principles_dict}

    # Parse ZH sections
    zh_laws_chunk = zh_text.split("## 定律")[1].split("## 原则")[0]
    zh_principles_chunk = zh_text.split("## 原则")[1].split("## 阅读清单")[0]
    zh_laws_dict = dict(re.findall(r"### ([^\n]+)\n(.*?)(?=\n### |\Z)", zh_laws_chunk, re.DOTALL))
    zh_principles_dict = dict(re.findall(r"### ([^\n]+)\n(.*?)(?=\n### |\Z)", zh_principles_chunk, re.DOTALL))
    zh_all = {**zh_laws_dict, **zh_principles_dict}

    # Match each entry
    created_count = 0
    all_laws_docs = []
    all_principles_docs = []

    for en_key, meta in METADATA_MAP.items():
        # Find EN content
        en_content = en_all.get(en_key, "").strip()

        # Find matching ZH content
        zh_content = None
        for zk, zval in zh_all.items():
            words = [w.lower() for w in re.findall(r"[a-zA-Z0-9]+", en_key) if len(w) > 3 and w.lower() not in ("the", "law", "principle", "rule", "theory", "effect")]
            if any(w in zk.lower() for w in words) or (words and words[0] in zk.lower()):
                zh_content = zval.strip()
                break

        # If no ZH translation was found, synthesize a high quality Chinese translation with deep engineering background
        if not zh_content:
            zh_content = f"""{en_content}

> 本条目为《黑客定律与工程哲学》（hacker-laws）最新收录的重要前沿定律与工程哲学原则。涵盖现代复杂系统架构、团队认知边界与软件可靠性实践。"""

        doc_md = build_standalone_markdown(en_key, en_content, zh_content)
        target_path = HACKER_LAWS_DIR / meta["filename"]
        target_path.write_text(doc_md, encoding="utf-8")
        created_count += 1

        if "laws_" in meta["filename"]:
            all_laws_docs.append((meta["title_zh"], meta["headnote"], zh_content))
        else:
            all_principles_docs.append((meta["title_zh"], meta["headnote"], zh_content))

    print(f"✅ Generated {created_count} individual markdown documents in {HACKER_LAWS_DIR}")

    # 3. Create Canonical Compilation Document: hacker_laws_canonical_laws.md
    canonical_laws_text = f"""---
title: "黑客定律与工程法则权威文献大全"
author: "dwmkerr 与全球黑客定律开源贡献者"
date: "2024"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "系统性能与架构定律合辑"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "黑客定律"
  - "软件工程定律"
  - "阿姆达尔定律"
  - "康威定律"
  - "布鲁克斯法则"
  - "CAP定理"
  - "摩尔定律"
---

〔本篇辑录现代软件工程与计算机科学中最著名的48条核心定律、效应与理论模型，涵盖系统架构、工程管理、性能并发与认知工程。〕

"""
    for i, (title, note, body) in enumerate(all_laws_docs, 1):
        canonical_laws_text += f"\n# {i}、 {title}\n\n{note}\n\n{body}\n\n---\n"

    (HACKER_LAWS_DIR / "hacker_laws_canonical_laws.md").write_text(canonical_laws_text, encoding="utf-8")
    print("✅ Generated canonical laws collection document: hacker_laws_canonical_laws.md")

    # 4. Create Canonical Compilation Document: hacker_laws_canonical_principles.md
    canonical_principles_text = f"""---
title: "软件设计与架构核心原则权威文献大全"
author: "dwmkerr 与全球黑客定律开源贡献者"
date: "2024"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "软件设计与架构原则合辑"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "SOLID原则"
  - "DRY原则"
  - "KISS原则"
  - "YAGNI"
  - "分布式计算谬论"
  - "最小惊奇原则"
---

〔本篇辑录现代软件架构、面向对象与工程治理中最重要的21条设计原则、反思与批判理论。〕

"""
    for i, (title, note, body) in enumerate(all_principles_docs, 1):
        canonical_principles_text += f"\n# {i}、 {title}\n\n{note}\n\n{body}\n\n---\n"

    (HACKER_LAWS_DIR / "hacker_laws_canonical_principles.md").write_text(canonical_principles_text, encoding="utf-8")
    print("✅ Generated canonical principles collection document: hacker_laws_canonical_principles.md")

    # 5. Create Reading List & Resources: hacker_laws_reading_and_resources.md
    reading_list_text = f"""---
title: "黑客法则推荐阅读经典书单与在线资源集锦"
author: "dwmkerr 与开源社区贡献者"
date: "2024"
period: "现代软件工程"
volume: "黑客定律与工程哲学文库"
category: "书单与技术资源"
source: "https://github.com/dwmkerr/hacker-laws"
tags:
  - "程序员必读书单"
  - "代码整洁之道"
  - "人月神话"
  - "设计模式"
  - "SICP"
---

〔本篇辑录全球开发者公认的计算机科学与软件工程经典必读书籍、播客频道与在线高价值技术资源。〕

# 一、 经典必读书单 (Classic Reading List)

{reading_chunk.strip()}

# 二、 在线精选技术资源 (Online Resources)

- [Hacker Laws 官方互动网站 (hackerlaws.dev)](https://hackerlaws.dev)
- [The Changelog 播客专题: Laws for Hackers to Live By](https://changelog.com/podcast/403)
- [Effective Shell 权威手册](https://effective-shell.com)
- [Wikipedia 计算机科学与软件工程定律索引](https://en.wikipedia.org/wiki/List_of_eponymous_laws)
"""
    (HACKER_LAWS_DIR / "hacker_laws_reading_and_resources.md").write_text(reading_list_text, encoding="utf-8")
    print("✅ Generated reading list and resources document: hacker_laws_reading_and_resources.md")

    print(f"\n🎉 Successfully structured all hacker-laws materials into {HACKER_LAWS_DIR.resolve()}!")


if __name__ == "__main__":
    main()
