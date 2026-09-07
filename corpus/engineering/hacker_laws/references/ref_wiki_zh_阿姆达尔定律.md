---
title: "引用文献: 阿姆达尔定律 (ZH Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://zh.wikipedia.org/wiki/%E9%98%BF%E5%A7%86%E8%BE%BE%E5%B0%94%E5%AE%9A%E5%BE%8B"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《中文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: 阿姆达尔定律 (ZH Wikipedia)
- **原文链接**: [https://zh.wikipedia.org/wiki/%E9%98%BF%E5%A7%86%E8%BE%BE%E5%B0%94%E5%AE%9A%E5%BE%8B](https://zh.wikipedia.org/wiki/%E9%98%BF%E5%A7%86%E8%BE%BE%E5%B0%94%E5%AE%9A%E5%BE%8B)
- **引用锚文本**: 中文维基百科
- **抓取状态**: success

# 二、 文献正文内容

阿姆達爾定律（英語：Amdahl's law，Amdahl's argument），一個計算機科學界的經驗法則，因吉恩·阿姆達爾而得名。它代表了處理器并行運算之後效率提升的能力。


== 基本描述 ==
平行運算中的加速比是用并行前的执行速度和并行后的执行速度之比来表示的，它表示了在并行化之后的效率提升情况。
阿姆达尔定律是固定负载（计算总量不变时）时的量化标准。可用公式：
  
    
      
        
          
            
              
                W
                
                  s
                
              
              +
              
                W
                
                  p
                
              
            
            
              
                W
                
                  s
                
              
              +
              
                
                  
                    W
                    
                      p
                    
                  
                  p
                
              
            
          
        
      
    
    {\displaystyle {\frac {W_{s}+W_{p}}{W_{s}+{\frac {W_{p}}{p}}}}}
  
来表示。式中
  
    
      
        
          W
          
            s
          
        
        ,
        
          W
          
            p
          
        
      
    
    {\displaystyle W_{s},W_{p}}
  
分别表示问题规模的串行分量（问题中不能并行化的部分）和并行分量，p表示平行計算時節點数量。


== 讨论 ==
注意到当 
  
    
      
        p
        →
        ∞
      
    
    {\displaystyle p\to \infty }
  
时，上式的极限是
  
    
      
        
          
            W
            
              W
              
                s
              
            
          
        
      
    
    {\displaystyle {\frac {W}{W_{s}}}}
  
，其中，
  
    
      
        
          W
        
        =
        
          
            W
            
              s
            
          
        
        +
        
          
            W
            
              p
            
          
        
      
    
    {\displaystyle {W}={W_{s}}+{W_{p}}}
  
。
这意味着无论如何增加平行处理器的数量，受限無法被並行處理的部分，加速比无法高于这个上限。


== 参阅 ==
Gustafson定律
关键路径
摩尔定律


== 参考文献 ==

---
_本地归档时间: 2026-09-05 | 来源: https://zh.wikipedia.org/wiki/%E9%98%BF%E5%A7%86%E8%BE%BE%E5%B0%94%E5%AE%9A%E5%BE%8B_
