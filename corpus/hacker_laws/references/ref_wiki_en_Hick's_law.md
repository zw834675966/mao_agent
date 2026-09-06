---
title: "引用文献: Hick's law (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Hick%27s_law"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Hick's law (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Hick%27s_law](https://en.wikipedia.org/wiki/Hick%27s_law)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

Hick's law, or the Hick–Hyman law, named after British and American psychologists William Edmund Hick and Ray Hyman, describes the time it takes for a person to make a decision as a result of the possible choices: increasing the number of choices will increase the decision time logarithmically. The Hick–Hyman law assesses cognitive information capacity in choice reaction experiments. The amount of time taken to process a certain amount of bits in the Hick–Hyman law is known as the "rate of gain of information". The plain language implication of the finding is that increasing the number of choices does not directly increase the time to choose. In other words, twice as many choices does not result in twice as long to choose. Also, because the relationship is logarithmic, the increase in time it takes to choose becomes less and less as the number of choices increases.


== Background ==
In 1868, Franciscus Donders reported the relationship between having multiple stimuli and choice reaction time. In 1885, J. Merkel discovered that the response time is longer when a stimulus belongs to a larger set of stimuli. Psychologists began to see similarities between this phenomenon and information theory.
Hick first began experimenting with this theory in 1951. In his first experiment, 10 lamps were arranged in a circle around the subject, each paired with a Morse key operated by a different finger. A pre-punched tape activated a random lamp every 5 seconds. Four electric pens recorded each lamp activation on a moving paper strip as a 4-bit binary code; when the subject pressed the corresponding key, the same pens recorded the response in the same format. The distance between the two marks on the paper gave the reaction time. Although the 4-bit encoding could represent up to 16 states (15 lamp positions plus "all clear"), Hick used only 10.
Hick performed a second experiment using the same task, while keeping the number of alternatives at 10. The participant performed the task the first two times with the instruction to perform the task as accurately as possible. For the last task, the participant was asked to perform the task as quickly as possible.
While Hick was stating that the relationship between reaction time and the number of choices was logarithmic, Hyman wanted to better understand the relationship between the reaction time and the mean number of choices. In Hyman's experiment, he had eight different lights arranged in a 6x6 matrix. Each of these different lights was given a name, so the participant was timed in the time it took to say the name of the light after it was lit. Further experiments changed the number of each different type of light. Hyman was responsible for determining a linear relation between reaction time and the information transmitted.


== Law ==

Given n equally probable choices, the average reaction time T required to choose among the choices is approximately:

  
    
      
        T
        =
        b
        ⋅
        
          log
          
            2
          
        
        ⁡
        (
        n
        +
        1
        )
      
    
    {\displaystyle T=b\cdot \log _{2}(n+1)}
  

where b is a constant that can be determined empirically by fitting a line to measured data. The logarithm expresses depth of "choice tree" hierarchy – log2 indicates binary search was performed. Addition of 1 to n takes into account the "uncertainty about whether to respond or not, as well as about which response to make."
In the case of choices with unequal probabilities, the law can be generalized as:

  
    
      
        T
        =
        b
        H
      
    
    {\displaystyle T=bH}
  

where H is strongly related to the information-theoretic entropy of the decision, defined as

  
    
      
        H
        =
        
          ∑
          
            i
          
          
            n
          
        
        
          p
          
            i
          
        
        
          log
          
            2
          
        
        ⁡
        (
        1
        
          /
        
        
          p
          
            i
          
        
        +
        1
        )
      
    
    {\displaystyle H=\sum _{i}^{n}p_{i}\log _{2}(1/p_{i}+1)}
  

where pi refers to the probability of the ith alternative yielding the information-theoretic entropy.
Hick's law is similar in form to Fitts's law. Hick's law has a logarithmic form because people subdivide the total collection of choices into categories, eliminating about half of the remaining choices at each step, rather than considering each and every choice one-by-one, which would require linear time.


=== Relation to IQ ===

E. Roth (1964) demonstrated a correlation between IQ and information processing speed, which is the reciprocal of the slope of the function:

  
    
      
        
          Reaction Time
        
        =
        
          Movement Time
        
        +
        
          
            
              
                log
                
                  2
                
              
              ⁡
              (
              n
              )
            
            Processing Speed
          
        
      
    
    {\displaystyle {\text{Reaction Time}}={\text{Movement Time}}+{\frac {\log _{2}(n)}{\text{Processing Speed}}}}
  

where n is the number of choices. The time it takes to come to a decision is proportional to:

  
    
      
        
          
            
              
                log
                
                  2
                
              
              ⁡
              (
              n
              )
            
            Processing Speed
          
        
      
    
    {\displaystyle {\frac {\log _{2}(n)}{\text{Processing Speed}}}}
  


== Stimulus–response compatibility ==
The stimulus–response compatibility is known to also affect the choice reaction time for the Hick–Hyman law. This means that the response should be similar to the stimulus itself (such as turning a steering wheel to turn the wheels of the car). The action the user performs is similar to the response the driver receives from the car.


== Exceptions ==

Studies suggest that the search for a word within a randomly ordered list—in which the reaction time increases linearly according to the number of items—does not allow for the generalization of the scientific law, considering that, in other conditions, the reaction time may not be linearly associated to the logarithm of the number of elements or even show other variations of the basic plane.
Exceptions to Hick's law have been identified in studies of verbal response to familiar stimuli, where there is no relationship or only a subtle increase in the reaction time associated with an increased number of elements, and saccade responses, where it was shown that there is either no relationship, or a decrease in the saccadic time with the increase of the number of elements, thus an antagonistic effect to that postulated by Hick's law.
The generalization of Hick's law was also tested in studies on the predictability of transitions associated with the reaction time of elements that appeared in a structured sequence. This process was first described as being in accordance to Hick's law, but more recently it was shown that the relationship between predictability and reaction time is sigmoid, not linear associated with different modes of action.
Hick's law is sometimes cited to justify menu design decisions. For example, to find a given word (e.g. the name of a command) in a randomly ordered word list (e.g. a menu), scanning of each word in the list is required, consuming linear time, so Hick's law does not apply. However, if the list is alphabetical and the user knows the name of the command, he or she may be able to use a subdividing strategy that works in logarithmic time.


== See also ==
Power law of practice
The Paradox of Choice
Fitts's Law


== Notes ==


== References ==
Cockburn, Andy; Gutwin, Carl; Greenberg, Saul (April 28 – May 3, 2007). "A predictive model of menu performance" (PDF). Proceedings of the SIGCHI Conference on Human Factors in Computing Systems. San Jose, California. pp. 627–636. doi:10.1145/1240624.1240723. hdl:10092/662. ISBN 978-1-59593-593-9. S2CID 7340315.{{cite book}}:  CS1 maint: location missing publisher (link)
Hick, W. E. (1 March 1952). "On the rate of gain of information" (PDF). Quarterly Journal of Experimental Psychology. 4 (1): 11–26. doi:10.1080/17470215208416600. S2CID 39060506.
Hyman, R (March 1953). "Stimulus information as a determinant of reaction time". Journal of Experimental Psychology. 45 (3): 188–96. doi:10.1037/h0056940. PMID 13052851. S2CID 17559281.
Rosati, L. (October 24–25, 2013). "How to design interfaces for choice: Hick-Hyman law and classification for information architecture". In Slavic, A.; Salah, A.; Davies, C. (eds.). Classification and visualization: interfaces to knowledge: proceedings of the International UDC Seminar. The Hague, The Netherlands. pp. 125–138.{{cite book}}:  CS1 maint: location missing publisher (link)
Roy, Q.; Malacria, S.; Lecolinet, E.; Guiard, Y.; Eagan, J. (April 27 – May 2, 2013). "Augmented letters: Mnemonic gesture-based shortcuts". Proceedings of the SIGCHI Conference on Human Factors in Computing Systems (PDF). Paris, France. pp. 2325–2328. doi:10.1145/2470654.2481321. ISBN 978-1-4503-1899-0. S2CID 15928158.{{cite book}}:  CS1 maint: location missing publisher (link)
Seow, Steven C. (2005). "Information Theoretic Models of HCI: A Comparison of the Hick–Hyman Law and Fitts' Law". Human-Computer Interaction. 20 (3): 315–352. CiteSeerX 10.1.1.86.4509. doi:10.1207/s15327051hci2003_3. S2CID 14436546. {{cite journal}}: Cite uses deprecated parameter |citeseerx= (help)
Welford, Alan T. (1968). Fundamentals of Skill. Methuen, Massachusetts. pp. 61–65.{{cite book}}:  CS1 maint: location missing publisher (link)


== External links ==
Usability Glossary: Hick's Law

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Hick%27s_law_
