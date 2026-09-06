---
title: "引用文献: Amdahl's law (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Amdahl%27s_law"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Amdahl's law (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Amdahl%27s_law](https://en.wikipedia.org/wiki/Amdahl%27s_law)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

In computer architecture, Amdahl's law (or Amdahl's argument) is a formula limiting the speedup of a task as resources are added to the system executing that task.
The law can be stated as:

the overall performance improvement gained by optimizing a single part of a system is limited by the fraction of time that the improved part is actually used.

It is named after computer scientist Gene Amdahl, and was presented at the American Federation of Information Processing Societies (AFIPS) Spring Joint Computer Conference in 1967.
Amdahl's law is often used in parallel computing to predict the theoretical speedup when using multiple processors.


== Definition ==
In the context of Amdahl's law, speedup can be defined as:

  
    
      
        
          Speedup
        
        =
        
          
            Performance for the entire task when enhancements are applied
            Performance for the same task without those enhancements
          
        
      
    
    {\displaystyle {\text{Speedup}}={\frac {\text{Performance for the entire task when enhancements are applied}}{\text{Performance for the same task without those enhancements}}}}
  

or

  
    
      
        
          Speedup
        
        =
        
          
            Execution time for the entire task without enhancements
            Execution time for the same task when enhancements are applied
          
        
      
    
    {\displaystyle {\text{Speedup}}={\frac {\text{Execution time for the entire task without enhancements}}{\text{Execution time for the same task when enhancements are applied}}}}
  

Amdahl's law can be formulated in the following way:

  
    
      
        
          
            Speedup
          
          
            overall
          
        
        =
        
          
            1
            
              (
              1
              −
              
                
                  time
                
                
                  optimized
                
              
              )
              +
              
                
                  
                    
                      time
                    
                    
                      optimized
                    
                  
                  
                    
                      speedup
                    
                    
                      optimized
                    
                  
                
              
            
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{overall}}={\frac {1}{(1-{\text{time}}_{\text{optimized}})+{\frac {{\text{time}}_{\text{optimized}}}{{\text{speedup}}_{\text{optimized}}}}}}}
  

where

  
    
      
        
          
            Speedup
          
          
            overall
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{overall}}}
  
 represents the total speedup of a program

  
    
      
        
          
            Time
          
          
            optimized
          
        
      
    
    {\displaystyle {\text{Time}}_{\text{optimized}}}
  
 represents the proportion of time spent on the portion of the code where improvements are made

  
    
      
        
          
            Speedup
          
          
            optimized
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{optimized}}}
  
 represents the extent of the improvement
The 
  
    
      
        
          
            Speedup
          
          
            overall
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{overall}}}
  
 is frequently much lower than one might expect. For instance, if a programmer enhances a part of the code that represents 10% of the total execution time (i.e. 
  
    
      
        
          
            Time
          
          
            optimized
          
        
      
    
    {\displaystyle {\text{Time}}_{\text{optimized}}}
  
 of 0.10) and achieves a  
  
    
      
        
          
            Speedup
          
          
            optimized
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{optimized}}}
  
 of 10,000, then  
  
    
      
        
          
            Speedup
          
          
            overall
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{overall}}}
  
 becomes 1.11 which means only 11% improvement in total speedup of the program. So, despite a massive improvement in one section, the overall benefit is quite small. In another example, if the programmer optimizes a section that accounts for 99% of the execution time (i.e. 
  
    
      
        
          
            Time
          
          
            optimized
          
        
      
    
    {\displaystyle {\text{Time}}_{\text{optimized}}}
  
 of 0.99) with a speedup factor of 100 (i.e. 
  
    
      
        
          
            Speedup
          
          
            optimized
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{optimized}}}
  
of 100), the 
  
    
      
        
          
            Speedup
          
          
            overall
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{overall}}}
  
 only reaches 50. This indicates that half of the potential performance gain (
  
    
      
        
          
            Speedup
          
          
            overall
          
        
      
    
    {\displaystyle {\text{Speedup}}_{\text{overall}}}
  
 will reach 100 if 100% of the execution time is covered) is lost due to the remaining 1% of execution time that was not improved.


== Derivation ==
A task executed by a system whose resources are improved compared to an initial similar system can be split up into two parts:

a part that does not benefit from the improvement of the resources of the system;
a part that benefits from the improvement of the resources of the system.
An example is a computer program that processes files. A part of that program may scan the directory of the disk and create a list of files internally in memory. After that, another part of the program passes each file to a separate thread for processing. The part that scans the directory and creates the file list cannot be sped up on a parallel computer, but the part that processes the files can.
The execution time of the whole task before the improvement of the resources of the system is denoted as 
  
    
      
        T
      
    
    {\displaystyle T}
  
. It includes the execution time of the part that would not benefit from the improvement of the resources and the execution time of the one that would benefit from it. The fraction of the execution time of the task that would benefit from the improvement of the resources is denoted by 
  
    
      
        p
      
    
    {\displaystyle p}
  
. The one concerning the part that would not benefit from it is therefore 
  
    
      
        1
        −
        p
      
    
    {\displaystyle 1-p}
  
. Then:

  
    
      
        T
        =
        (
        1
        −
        p
        )
        T
        +
        p
        T
        .
      
    
    {\displaystyle T=(1-p)T+pT.}
  

It is the execution of the part that benefits from the improvement of the resources that is accelerated by the factor 
  
    
      
        s
      
    
    {\displaystyle s}
  
 after the improvement of the resources. Consequently, the execution time of the part that does not benefit from it remains the same, while the part that benefits from it becomes:

  
    
      
        
          
            p
            s
          
        
        T
        .
      
    
    {\displaystyle {\frac {p}{s}}T.}
  

The theoretical execution time 
  
    
      
        T
        (
        s
        )
      
    
    {\displaystyle T(s)}
  
 of the whole task after the improvement of the resources is then:

  
    
      
        T
        (
        s
        )
        =
        (
        1
        −
        p
        )
        T
        +
        
          
            p
            s
          
        
        T
        .
      
    
    {\displaystyle T(s)=(1-p)T+{\frac {p}{s}}T.}
  

Amdahl's law gives the theoretical speedup in latency of the execution of the whole task at fixed workload 
  
    
      
        W
      
    
    {\displaystyle W}
  
, which yields

  
    
      
        
          S
          
            latency
          
        
        (
        s
        )
        =
        
          
            
              T
              W
            
            
              T
              (
              s
              )
              W
            
          
        
        =
        
          
            T
            
              T
              (
              s
              )
            
          
        
        =
        
          
            1
            
              1
              −
              p
              +
              
                
                  p
                  s
                
              
            
          
        
        .
      
    
    {\displaystyle S_{\text{latency}}(s)={\frac {TW}{T(s)W}}={\frac {T}{T(s)}}={\frac {1}{1-p+{\frac {p}{s}}}}.}
  


=== Parallel programs ===
If 30% of the execution time may be the subject of a speedup, p will be 0.3; if the improvement makes the affected part twice as fast, s will be 2. Amdahl's law states that the overall speedup of applying the improvement will be:

  
    
      
        
          S
          
            latency
          
        
        =
        
          
            1
            
              1
              −
              p
              +
              
                
                  p
                  s
                
              
            
          
        
        =
        
          
            1
            
              1
              −
              0.3
              +
              
                
                  0.3
                  2
                
              
            
          
        
        =
        1.18.
      
    
    {\displaystyle S_{\text{latency}}={\frac {1}{1-p+{\frac {p}{s}}}}={\frac {1}{1-0.3+{\frac {0.3}{2}}}}=1.18.}
  

For example, assume that we are given a serial task which is split into four consecutive parts, whose percentages of execution time are p1 = 0.11, p2 = 0.18, p3 = 0.23, and p4 = 0.48 respectively. Then we are told that the 1st part is not sped up, so s1 = 1, while the 2nd part is sped up 5 times, so s2 = 5, the 3rd part is sped up 20 times, so s3 = 20, and the 4th part is sped up 1.6 times, so s4 = 1.6. By using Amdahl's law, the overall speedup is

  
    
      
        
          S
          
            latency
          
        
        =
        
          
            1
            
              
                
                  
                    p
                    1
                  
                  
                    s
                    1
                  
                
              
              +
              
                
                  
                    p
                    2
                  
                  
                    s
                    2
                  
                
              
              +
              
                
                  
                    p
                    3
                  
                  
                    s
                    3
                  
                
              
              +
              
                
                  
                    p
                    4
                  
                  
                    s
                    4
                  
                
              
            
          
        
        =
        
          
            1
            
              
                
                  0.11
                  1
                
              
              +
              
                
                  0.18
                  5
                
              
              +
              
                
                  0.23
                  20
                
              
              +
              
                
                  0.48
                  1.6
                
              
            
          
        
        =
        2.19.
      
    
    {\displaystyle S_{\text{latency}}={\frac {1}{{\frac {p1}{s1}}+{\frac {p2}{s2}}+{\frac {p3}{s3}}+{\frac {p4}{s4}}}}={\frac {1}{{\frac {0.11}{1}}+{\frac {0.18}{5}}+{\frac {0.23}{20}}+{\frac {0.48}{1.6}}}}=2.19.}
  

Notice how the 5 times and 20 times speedup on the 2nd and 3rd parts respectively don't have much effect on the overall speedup when the 4th part (48% of the execution time) is accelerated by only 1.6 times.


=== Serial programs ===

For example, with a serial program in two parts A and B for which TA = 3 s and TB = 1 s,

if part B is made to run 5 times faster, that is s = 5 and p = TB/(TA + TB) = 0.25, then 
  
    
      
        
          S
          
            latency
          
        
        =
        
          
            1
            
              1
              −
              0.25
              +
              
                
                  0.25
                  5
                
              
            
          
        
        =
        1.25
        ;
      
    
    {\displaystyle S_{\text{latency}}={\frac {1}{1-0.25+{\frac {0.25}{5}}}}=1.25;}
  

if part A is made to run 2 times faster, that is s = 2 and p = TA/(TA + TB) = 0.75, then 
  
    
      
        
          S
          
            latency
          
        
        =
        
          
            1
            
              1
              −
              0.75
              +
              
                
                  0.75
                  2
                
              
            
          
        
        =
        1.60.
      
    
    {\displaystyle S_{\text{latency}}={\frac {1}{1-0.75+{\frac {0.75}{2}}}}=1.60.}
  

Therefore, making part 
  
    
      
        A
      
    
    {\displaystyle A}
  
 to run 2 times faster is better than making part 
  
    
      
        B
      
    
    {\displaystyle B}
  
 to run 5 times faster. The percentage improvement in speed can be calculated as

  
    
      
        
          percentage improvement
        
        =
        100
        
          (
          
            1
            −
            
              
                1
                
                  S
                  
                    latency
                  
                
              
            
          
          )
        
        .
      
    
    {\displaystyle {\text{percentage improvement}}=100\left(1-{\frac {1}{S_{\text{latency}}}}\right).}
  

Improving part A by a factor of 2 will increase overall program speed by a factor of 1.60, which makes it 37.5% faster than the original computation.
However, improving part B by a factor of 5, which presumably requires more effort, will achieve an overall speedup factor of 1.25 only, which makes it 20% faster.


=== Optimizing the sequential part of parallel programs ===
If the non-parallelizable part is optimized by a factor of 
  
    
      
        O
      
    
    {\displaystyle O}
  
, then

  
    
      
        T
        (
        O
        ,
        s
        )
        =
        (
        1
        −
        p
        )
        
          
            T
            O
          
        
        +
        
          
            p
            s
          
        
        T
        .
      
    
    {\displaystyle T(O,s)=(1-p){\frac {T}{O}}+{\frac {p}{s}}T.}
  

It follows from Amdahl's law that the speedup due to parallelism is given by

  
    
      
        
          S
          
            latency
          
        
        (
        O
        ,
        s
        )
        =
        
          
            
              T
              (
              O
              )
            
            
              T
              (
              O
              ,
              s
              )
            
          
        
        =
        
          
            
              (
              1
              −
              p
              )
              
                
                  1
                  O
                
              
              +
              
                p
              
            
            
              
                
                  
                    1
                    −
                    p
                  
                  O
                
              
              +
              
                
                  p
                  s
                
              
            
          
        
        .
      
    
    {\displaystyle S_{\text{latency}}(O,s)={\frac {T(O)}{T(O,s)}}={\frac {(1-p){\frac {1}{O}}+{p}}{{\frac {1-p}{O}}+{\frac {p}{s}}}}.}
  

When 
  
    
      
        s
        =
        1
      
    
    {\displaystyle s=1}
  
, we have 
  
    
      
        
          S
          
            latency
          
        
        (
        O
        ,
        s
        )
        =
        1
      
    
    {\displaystyle S_{\text{latency}}(O,s)=1}
  
, meaning that the speedup is
measured with respect to the execution time after the non-parallelizable part is optimized.
When  
  
    
      
        s
        =
        ∞
      
    
    {\displaystyle s=\infty }
  
, 

  
    
      
        
          S
          
            latency
          
        
        (
        O
        ,
        ∞
        )
        =
        
          
            
              T
              (
              O
              )
            
            
              T
              (
              O
              ,
              s
              )
            
          
        
        =
        
          
            
              (
              1
              −
              p
              )
              
                
                  1
                  O
                
              
              +
              
                p
              
            
            
              
                
                  
                    1
                    −
                    p
                  
                  O
                
              
              +
              
                
                  p
                  s
                
              
            
          
        
        =
        1
        +
        
          
            p
            
              1
              −
              p
            
          
        
        O
        .
      
    
    {\displaystyle S_{\text{latency}}(O,\infty )={\frac {T(O)}{T(O,s)}}={\frac {(1-p){\frac {1}{O}}+{p}}{{\frac {1-p}{O}}+{\frac {p}{s}}}}=1+{\frac {p}{1-p}}O.}
  

If  
  
    
      
        1
        −
        p
        =
        0.4
      
    
    {\displaystyle 1-p=0.4}
  
, 
  
    
      
        O
        =
        2
      
    
    {\displaystyle O=2}
  
 and 
  
    
      
        s
        =
        5
      
    
    {\displaystyle s=5}
  
, then:

  
    
      
        
          S
          
            latency
          
        
        (
        O
        ,
        s
        )
        =
        
          
            
              T
              (
              O
              )
            
            
              T
              (
              O
              ,
              s
              )
            
          
        
        =
        
          
            
              
                0.4
              
              
                
                  1
                  2
                
              
              +
              0.6
            
            
              
                
                  0.4
                  2
                
              
              +
              
                
                  0.6
                  5
                
              
            
          
        
        =
        2.5.
      
    
    {\displaystyle S_{\text{latency}}(O,s)={\frac {T(O)}{T(O,s)}}={\frac {{0.4}{\frac {1}{2}}+0.6}{{\frac {0.4}{2}}+{\frac {0.6}{5}}}}=2.5.}
  


=== Transforming sequential parts of parallel programs into parallelizable ===
Next, we consider the case wherein the non-parallelizable part is reduced by a factor of 
  
    
      
        
          O
          ′
        
      
    
    {\displaystyle O'}
  
, and the parallelizable part is correspondingly increased. Then

  
    
      
        
          T
          ′
        
        (
        
          O
          ′
        
        ,
        s
        )
        =
        
          
            
              1
              −
              p
            
            
              O
              ′
            
          
        
        T
        +
        
          (
          
            1
            −
            
              
                
                  1
                  −
                  p
                
                
                  O
                  ′
                
              
            
          
          )
        
        
          
            T
            s
          
        
        .
      
    
    {\displaystyle T'(O',s)={\frac {1-p}{O'}}T+\left(1-{\frac {1-p}{O'}}\right){\frac {T}{s}}.}
  

It follows from Amdahl's law that the speedup due to parallelism is given by

  
    
      
        
          S
          
            latency
          
          ′
        
        (
        
          O
          ′
        
        ,
        s
        )
        =
        
          
            
              
                T
                ′
              
              (
              
                O
                ′
              
              )
            
            
              
                T
                ′
              
              (
              
                O
                ′
              
              ,
              s
              )
            
          
        
        =
        
          
            1
            
              
                
                  
                    1
                    −
                    p
                  
                  
                    O
                    ′
                  
                
              
              +
              
                (
                
                  1
                  −
                  
                    
                      
                        1
                        −
                        p
                      
                      
                        O
                        ′
                      
                    
                  
                
                )
              
              
                
                  1
                  s
                
              
            
          
        
        .
      
    
    {\displaystyle S'_{\text{latency}}(O',s)={\frac {T'(O')}{T'(O',s)}}={\frac {1}{{\frac {1-p}{O'}}+\left(1-{\frac {1-p}{O'}}\right){\frac {1}{s}}}}.}
  


== Relation to the law of diminishing returns ==
Amdahl's law is often conflated with the law of diminishing returns, whereas only a special case of applying Amdahl's law demonstrates the law of diminishing returns. If one picks optimally (in terms of the achieved speedup) what is to be improved, then one will see monotonically decreasing improvements as one improves. If, however, one picks non-optimally, after improving a sub-optimal component and moving on to improve a more optimal component, one can see an increase in the return. Note that it is often rational to improve a system in an order that is "non-optimal" in this sense, given that some improvements are more difficult or require larger development time than others.
Amdahl's law does represent the law of diminishing returns if one is considering what sort of return one gets by adding more processors to a machine, if one is running a fixed-size computation that will use all available processors to their capacity. Each new processor added to the system will add less usable power than the previous one. Each time one doubles the number of processors the speedup ratio will diminish, as the total throughput heads toward the limit of 
  
    
      
        
          
            1
            
              1
              −
              p
            
          
        
      
    
    {\displaystyle {\frac {1}{1-p}}}
  
.
This analysis neglects other potential bottlenecks such as memory bandwidth and I/O bandwidth. If these resources do not scale with the number of processors, then merely adding processors provides even lower returns.
An implication of Amdahl's law is that to speed up real applications which have both serial and parallel portions, heterogeneous computing techniques are required. There are novel speedup and energy consumption models based on a more general representation of heterogeneity, referred to as the normal form heterogeneity, that support a wide range of heterogeneous many-core architectures. These modelling methods aim to predict system power efficiency and performance ranges, and facilitates research and development at the hardware and system software levels.


== See also ==
Gustafson's law
Universal Scalability Law
Analysis of parallel algorithms
Critical path method
Brooks's law
Koomey's law – Electronic hardware trend
Moore's law
List of eponymous laws


== References ==


== Further reading ==
Amdahl, Gene M. (1967). "Validity of the single processor approach to achieving large scale computing capabilities" (PDF). Proceedings of the April 18-20, 1967, spring joint computer conference on - AFIPS '67 (Spring). pp. 483–485. doi:10.1145/1465482.1465560. S2CID 195607370.


== External links ==

Gene M. Amdahl (1989), Oral history interview with Gene M. Amdahl, Charles Babbage Institute, University of Minnesota, hdl:11299/104341. Amdahl discusses his graduate work at the University of Wisconsin and his design of WISC. Discusses his role in the design of several computers for IBM including the STRETCH, IBM 701, and IBM 704. He discusses his work with Nathaniel Rochester and IBM's management of the design process. Mentions work with Ramo-Wooldridge, Aeronutronic, and Computer Sciences Corporation
"Amdahl's Law" by Joel F. Klein, Wolfram Demonstrations Project (2007)
Amdahl's Law in the Multicore Era (July 2008)

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Amdahl%27s_law_
