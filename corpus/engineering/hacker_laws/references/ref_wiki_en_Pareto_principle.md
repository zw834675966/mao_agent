---
title: "引用文献: Pareto principle (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Pareto_principle"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Pareto principle (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Pareto_principle](https://en.wikipedia.org/wiki/Pareto_principle)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

The Pareto principle (also known as the 80:20 rule, the law of the vital few and the principle of factor sparsity) states that, for many outcomes, roughly 80% of consequences come from 20% of causes (the "vital few").
In 1941, management consultant Joseph M. Juran developed the concept in the context of quality control and improvement after reading the works of Italian sociologist and economist Vilfredo Pareto, who wrote in 1906 about the 80:20 connection while teaching at the University of Lausanne. In his first work, Cours d'économie politique, Pareto showed that approximately 80% of the land in the Kingdom of Italy was owned by 20% of the population. The Pareto principle is only tangentially related to the concept of Pareto efficiency.
Mathematically, the 80:20 rule is associated with a power law distribution (also known as a Pareto distribution). In many natural phenomena certain features are distributed according to power law statistics. It is an adage of business management that "80% of sales come from 20% of clients."


== History ==
In 1941, Joseph M. Juran, a Romanian-born American engineer, came across the work of Italian polymath Vilfredo Pareto. Pareto noted that approximately 80% of Italy's land was owned by 20% of the population. Juran applied the approximation that 80% of problems stem from 20% of the causes to the field of quality management. Later during his career, Juran preferred to describe this as "the vital few and the useful many", to dissuade from an interpretation of the principle as the contribution of the 80% being without value.


== Mathematical explanation ==
The demonstration of the Pareto principle is explained by a large proportion of process variation being associated with a small proportion of process variables. This is a special case of the wider phenomenon of Pareto distributions. If the Pareto index α, which is one of the parameters characterizing a Pareto distribution, is chosen as α = log45 ≈ 1.16, then one has 80% of effects coming from 20% of causes.
The term 80:20 is only a shorthand for the general principle at work. In individual cases, the distribution could be nearer to 90:10 or 70:30. Note that there is no need for the two numbers to add up to the number 100, as they are measures of different things. The Pareto principle is an illustration of a "power law" relationship, which also occurs in phenomena such as bush fires and earthquakes. Benoit Mandelbrot offered an explanation for this pattern in the field of economics and social science based on income dynamics in population. According to his reasoning, above a certain minimum income threshold, the probability of an individual's income increasing or decreasing by a fixed proportion (e.g., doubling) remains constant across all income levels. As a consequence, the ratio of individuals earning a given income x to those earning half that amount x/2 remains the same, regardless of the absolute value of x. This scale-invariant property is a defining feature of power-law distributions. Because it is self-similar over a wide range of magnitudes, it produces outcomes completely different from Normal or Gaussian distribution phenomena. The occurrence probability of rare extreme (or catastrophic) events showing power-law distribution may be of several orders of magnitude greater than that associated with other usual models, such as, e.g., Gaussian or exponential. This fact explains the frequent breakdowns of sophisticated financial instruments, which are modeled on the assumption that a Gaussian relationship is appropriate to something like stock price movements. 


=== Derivation of α for the 80:20 rule ===
As an example, consider the Pareto distribution of wealth. The (Type 1) Pareto distribution is defined as:

  
    
      
        p
        (
        x
        )
        =
        
          
            {
            
              
                
                  
                    
                      
                        α
                        
                        
                          x
                          
                            
                              m
                            
                          
                          
                            α
                          
                        
                      
                      
                        x
                        
                          α
                          +
                          1
                        
                      
                    
                  
                
                
                  x
                  ≥
                  
                    x
                    
                      
                        m
                      
                    
                  
                  ,
                
              
              
                
                  0
                
                
                  x
                  <
                  
                    x
                    
                      
                        m
                      
                    
                  
                  .
                
              
            
            
          
        
      
    
    {\displaystyle p(x)={\begin{cases}{\frac {\alpha \,x_{\mathrm {m} }^{\alpha }}{x^{\alpha +1}}}&x\geq x_{\mathrm {m} },\\0&x<x_{\mathrm {m} }.\end{cases}}}
  

where 
  
    
      
        
          x
          
            m
          
        
      
    
    {\displaystyle x_{m}}
  
 is the scale parameter and 
  
    
      
        α
      
    
    {\displaystyle \alpha }
  
 is the shape parameter. The x variable will represent wealth in (e.g.) dollars, while p(x)dx will represent the fraction of the population with wealth between x and x+dx dollars. Defining N as the total population, the number of people owning between x and x+dx dollars will be 
  
    
      
        N
        p
        (
        x
        )
        d
        x
      
    
    {\displaystyle Np(x)dx}
  
 and they will own a total of 
  
    
      
        N
        x
        
        p
        (
        x
        )
        d
        x
      
    
    {\displaystyle Nx\,p(x)dx}
  
 dollars.
The total number of people with wealth between 
  
    
      
        
          x
          
            a
          
        
      
    
    {\displaystyle x_{a}}
  
 and 
  
    
      
        
          x
          
            b
          
        
      
    
    {\displaystyle x_{b}}
  
 dollars will then be: 

  
    
      
        N
        
          ∫
          
            
              x
              
                a
              
            
          
          
            
              x
              
                b
              
            
          
        
        p
        (
        x
        )
        d
        x
      
    
    {\displaystyle N\int _{x_{a}}^{x_{b}}p(x)dx}
  

and they will be holding:

  
    
      
        N
        
          ∫
          
            
              x
              
                a
              
            
          
          
            
              x
              
                b
              
            
          
        
        x
        
        p
        (
        x
        )
        d
        x
      
    
    {\displaystyle N\int _{x_{a}}^{x_{b}}x\,p(x)dx}
  

dollars of the total wealth. The total wealth is:

  
    
      
        N
        
          ∫
          
            
              x
              
                m
              
            
          
          
            ∞
          
        
        x
        
        p
        (
        x
        )
        d
        x
      
    
    {\displaystyle N\int _{x_{m}}^{\infty }x\,p(x)dx}
  

dollars. The 80% of the population on the low end of the wealth scale will be those owning between 
  
    
      
        
          x
          
            m
          
        
      
    
    {\displaystyle x_{m}}
  
 and 
  
    
      
        
          x
          
            o
          
        
      
    
    {\displaystyle x_{o}}
  
 dollars so that:

  
    
      
        
          
            
              N
              
                ∫
                
                  
                    x
                    
                      m
                    
                  
                
                
                  
                    x
                    
                      o
                    
                  
                
              
              p
              (
              x
              )
              d
              x
            
            
              N
              
                ∫
                
                  
                    x
                    
                      m
                    
                  
                
                
                  ∞
                
              
              p
              (
              x
              )
              d
              x
            
          
        
        =
        1
        −
        
          
            (
            
              
                
                  x
                  
                    m
                  
                
                
                  x
                  
                    o
                  
                
              
            
            )
          
          
            α
          
        
        =
        0.8
      
    
    {\displaystyle {\frac {N\int _{x_{m}}^{x_{o}}p(x)dx}{N\int _{x_{m}}^{\infty }p(x)dx}}=1-\left({\frac {x_{m}}{x_{o}}}\right)^{\alpha }=0.8}
  

and if they hold 20% of the wealth then:

  
    
      
        
          
            
              N
              
                ∫
                
                  
                    x
                    
                      m
                    
                  
                
                
                  
                    x
                    
                      o
                    
                  
                
              
              x
              
              p
              (
              x
              )
              d
              x
            
            
              N
              
                ∫
                
                  
                    x
                    
                      m
                    
                  
                
                
                  ∞
                
              
              x
              
              p
              (
              x
              )
              d
              x
            
          
        
        =
        1
        −
        
          
            (
            
              
                
                  x
                  
                    m
                  
                
                
                  x
                  
                    o
                  
                
              
            
            )
          
          
            α
            −
            1
          
        
        =
        0.2
      
    
    {\displaystyle {\frac {N\int _{x_{m}}^{x_{o}}x\,p(x)dx}{N\int _{x_{m}}^{\infty }x\,p(x)dx}}=1-\left({\frac {x_{m}}{x_{o}}}\right)^{\alpha -1}=0.2}
  

Solving the above two equations for 
  
    
      
        α
      
    
    {\displaystyle \alpha }
  
 and 
  
    
      
        
          x
          
            o
          
        
      
    
    {\displaystyle x_{o}}
  
 yields 
  
    
      
        α
        =
        
          log
          
            4
          
        
        ⁡
        (
        5
        )
      
    
    {\displaystyle \alpha =\log _{4}(5)}
  
 and 
  
    
      
        
          x
          
            o
          
        
        =
        4
        
        
          x
          
            m
          
        
      
    
    {\displaystyle x_{o}=4\,x_{m}}
  
.


=== Gini coefficient and Hoover index ===
Using the "A:B" notation (for example, 0.8:0.2) and with A + B = 1, inequality measures like the Gini index (G) and the Hoover index (H) can be computed. In this case both are the same:

  
    
      
        H
        =
        G
        =
        
          |
        
        2
        A
        −
        1
        
          |
        
        =
        
          |
        
        1
        −
        2
        B
        
          |
        
        =
        
          
            1
            
              2
              α
              −
              1
            
          
        
      
    
    {\displaystyle H=G=|2A-1|=|1-2B|={\frac {1}{2\alpha -1}}}
  

which in the 80:20 case yields 
  
    
      
        
          G
        
        ≈
        0.756
      
    
    {\displaystyle \mathrm {G} \approx 0.756}
  

  
    
      
        A
        :
        B
        =
        
          (
          
            
              
                1
                +
                H
              
              2
            
          
          )
        
        :
        
          (
          
            
              
                1
                −
                H
              
              2
            
          
          )
        
      
    
    {\displaystyle A:B=\left({\frac {1+H}{2}}\right):\left({\frac {1-H}{2}}\right)}
  


== Analysis ==

Pareto analysis is a formal technique useful where many possible courses of action are competing for attention. In essence, the problem-solver estimates the benefit delivered by each action, then  selects a number of the most effective actions that deliver a total benefit reasonably close to the maximal possible one.
Pareto analysis is a creative way of looking at causes of problems because it helps stimulate thinking and organize thoughts. However, it can be limited by its exclusion of possibly important problems which may be small initially, but will grow with time. It should be combined with other analytical tools such as failure mode and effects analysis and fault tree analysis for example.
This technique helps to identify the top portion of causes that need to be addressed to resolve the majority of problems. Once the predominant causes are identified, then tools like the Ishikawa diagram (also called Fish-bone Analysis) can be used to identify the root causes of the problems. While it is common to refer to Pareto as "80:20" rule, under the assumption that, in all situations, 20% of causes determine 80% of problems, this ratio is merely a convenient rule of thumb and is not, nor should it be considered, an immutable law of nature.
The application of the Pareto analysis in risk management allows management to focus on those risks that have the most impact on the project.
Steps to identify the important causes using 80:20 rule:

Form a frequency of occurrences as a percentage
Arrange the rows in decreasing order of importance of the causes (i.e., the most important cause first)
Add a cumulative percentage column to the table, then plot the information
Plot (#1) a curve with causes on x- and cumulative percentage on y-axis
Plot (#2) a bar graph with causes on x- and percent frequency on y-axis
Draw a horizontal dotted line at 80% from the y-axis to intersect the curve. Then draw a vertical dotted line from the point of intersection to the x-axis. The vertical dotted line separates the important causes (on the left) and trivial causes (on the right)
Explicitly review the chart to ensure that causes for at least 80% of the problems are captured


== Applications ==


=== Economics ===
Pareto's observation was in connection with population and wealth. Pareto noticed that approximately 80% of Italy's land was owned by 20% of the population. He then carried out surveys on a variety of other countries and found to his surprise that a similar distribution applied.
A chart that demonstrated the effect appeared in the 1992 United Nations Development Program Report, which showed that the richest 20% of the world's population receives 82.7% of the world's income. However, among nations, the Gini index shows that wealth distributions vary substantially around this norm.
The principle also holds within the tails of the distribution. The physicist Victor Yakovenko of the University of Maryland, College Park and AC Silva analyzed income data from the US Internal Revenue Service from 1983 to 2001 and found that the income distribution of the richest 1–3% of the population also follows Pareto's principle.


=== Computing ===
In computer science the Pareto principle can be applied to optimization efforts. For example, Microsoft noted that by fixing the top 20% of the most-reported bugs, 80% of the related errors and crashes in a given system would be eliminated. 


=== Engineering and quality control ===
The Pareto principle provides the basis for the Pareto chart, one of the key tools used in total quality control and Six Sigma techniques. The Pareto principle serves as a baseline for ABC-analysis and XYZ-analysis, widely used in logistics and procurement for the purpose of optimizing stock of goods, as well as costs of keeping and replenishing that stock. In engineering control theory, such as for electromechanical energy converters, the 80:20 principle applies to optimization efforts.
The remarkable success of statistically based searches for root causes is based upon a combination of an empirical principle and mathematical logic. The empirical principle is usually known as the Pareto principle. With regard to variation causality, this principle states that there is a non-random distribution of the slopes of the numerous (theoretically infinite) terms in the general equation.
All of the terms are independent of each other by definition. Interdependent factors appear as multiplication terms. The Pareto principle states that the effect of the dominant term is very much greater than the second-largest effect term, which in turn is very much greater than the third, and so on. There is no explanation for this phenomenon; that is why we refer to it as an empirical principle.
The mathematical logic is known as the square-root-of-the-sum-of-the-squares axiom. This states that the variation caused by the steepest slope must be squared, and then the result added to the square of the variation caused by the second-steepest slope, and so on. The total observed variation is then the square root of the total sum of the variation caused by individual slopes squared. This derives from the probability density function for multiple variables or the multivariate distribution (we are treating each term as an independent variable).
The combination of the Pareto principle and the square-root-of-the-sum-of-the-squares axiom means that the strongest term in the general equation totally dominates the observed variation of effect. Thus, the strongest term will dominate the data collected for hypothesis testing.
In the systems science discipline, Joshua M. Epstein and Robert Axtell created an agent-based simulation model called Sugarscape, from a decentralized modeling approach, based on individual behavior rules defined for each agent in the economy. Wealth distribution and Pareto's 80:20 principle emerged in their results, which suggests the principle is a collective consequence of these individual rules.


=== Health and social outcomes ===
In 2009, the Agency for Healthcare Research and Quality said 20% of patients incurred 80% of healthcare expenses due to chronic conditions. A 2021 analysis showed unequal distribution of healthcare costs, with older patients and those with poorer health incurring more costs.
The 80:20 rule has been proposed as a rule of thumb for the infection distribution in superspreading events. However, the degree of infectiousness has been found to be distributed continuously in the population. In epidemics with super-spreading, the majority of individuals infect relatively few secondary contacts.


== See also ==
1% rule – Hypothesis that more people will lurk in a virtual community than will participate
10/90 gap – Health statistic
Ninety–ninety rule – Humorous aphorism in computer programming
Sturgeon's law – Adage stating that "ninety percent of everything is crap"


== References ==


== Further reading ==
Bookstein, Abraham (1990), "Informetric distributions, part I: Unified overview", Journal of the American Society for Information Science, 41 (5): 368–375, doi:10.1002/(SICI)1097-4571(199007)41:5<368::AID-ASI8>3.0.CO;2-C
Klass, O. S.; Biham, O.; Levy, M.; Malcai, O.; Soloman, S. (2006), "The Forbes 400 and the Pareto wealth distribution", Economics Letters, 90 (2): 290–295, doi:10.1016/j.econlet.2005.08.020
Koch, R. (2004), Living the 80/20 Way: Work Less, Worry Less, Succeed More, Enjoy More, London: Nicholas Brealey Publishing, ISBN 1-85788-331-4
Reed, W. J. (2001), "The Pareto, Zipf and other power laws", Economics Letters, 74 (1): 15–19, doi:10.1016/S0165-1765(01)00524-9
Rosen, K. T.; Resnick, M. (1980), "The size distribution of cities: an examination of the Pareto law and primacy", Journal of Urban Economics, 8 (2): 165–186, doi:10.1016/0094-1190(80)90043-1
Rushton, A.; Oxley, J.; Croucher, P. (2000), The handbook of logistics and distribution management (2nd ed.), London: Kogan Page, ISBN 978-0-7494-3365-9.


== External links ==

Pareto Principle: Rule of causes and consequences
ParetoRule.cf : Pareto Rule Archived December 2, 2018, at the Wayback Machine
ParetoRule.cf : The Pareto Rule Archived December 2, 2018, at the Wayback Machine
About.com: Pareto's Principle Archived February 13, 2009, at the Wayback Machine
Simply Psychology: Pareto Principle (The 80-20 Rule): Examples & More

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Pareto_principle_
