---
title: "引用文献: Dependency inversion principle (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/Dependency_inversion_principle"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《英文维基百科》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: Dependency inversion principle (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/Dependency_inversion_principle](https://en.wikipedia.org/wiki/Dependency_inversion_principle)
- **引用锚文本**: 英文维基百科
- **抓取状态**: success

# 二、 文献正文内容

In object-oriented design, the dependency inversion principle is a specific methodology for loosely coupled software modules. When following this principle, the conventional dependency relationships established from high-level, policy-setting modules to low-level, dependency modules are reversed, thus rendering high-level modules independent of the low-level module implementation details. The principle states:

By dictating that both high-level and low-level objects must depend on the same abstraction, this design principle inverts the way some people may think about object-oriented programming.
The idea behind points A and B of this principle is that when designing the interaction between a high-level module and a low-level one, the interaction should be thought of as an abstract interaction between them. This has implications for the design of both the high-level and the low-level modules: the low-level one should be designed with the interaction in mind and it may be necessary to change its usage interface.
In many cases, thinking about the interaction itself as an abstract concept allows for reduction of the coupling between the components without introducing additional coding patterns and results in a lighter and less implementation-dependent interaction schema. When this abstract interaction schema is generic and clear, this design principle leads to the dependency inversion pattern described below.


== Traditional layers pattern ==
In conventional application architecture, lower-level components (e.g., Utility Layer) are designed to be consumed by higher-level components (e.g., Policy Layer) which enable highly composite systems to be built. Here higher-level components depend directly upon lower-level components to achieve some task. This dependency upon lower-level components limits the reuse opportunities of the higher-level components.

The goal of the dependency inversion pattern is to avoid this highly coupled distribution with the mediation of an abstract layer, and to  increase the reusability of higher and policy layers.


== Dependency inversion pattern ==
With the introduction of an abstract layer, both high- and lower-level layers reduce the traditional dependencies from top to bottom. Nevertheless, the "inversion" concept does not mean that lower-level layers directly depend on higher-level layers. Rather, both layers should depend on abstractions (interfaces) that expose the behavior needed by higher-level layers.

In a direct application of dependency inversion, the abstracts are owned by the higher/policy layers. This architecture groups the higher/policy components and the abstractions that define lower services together in the same package. The lower-level layers are created by inheritance/implementation of these abstract classes or interfaces.
The inversion of the dependencies and ownership encourages reuse of the higher and policy layers. Upper layers could use other implementations of the lower services. When the lower-level layer components are closed or when the application requires the reuse of existing services, it is common that an Adapter mediates between the services and the abstractions.


== Dependency inversion pattern generalization ==
In many projects the dependency inversion principle and pattern are considered as a single concept that should be generalized, i.e., applied to all interfaces between software modules. There are at least two reasons for that:

It is simpler to see a good thinking principle as a coding pattern. Once an abstract class or an interface has been coded, the programmer may say: "I have done the job of abstraction".
Because many unit testing tools rely on inheritance to accomplish mocking, the usage of generic interfaces between classes (not only between modules when it makes sense to use generality) became the rule.
If the mocking tool used relies only on inheritance, it may become necessary to widely apply the dependency inversion pattern. This has major drawbacks:

Merely implementing an interface over a class isn't sufficient to reduce coupling; only thinking about the potential abstraction of interactions can lead to a less coupled design.
Implementing generic interfaces everywhere in a project makes it harder to understand and maintain. At each step the reader will ask themself what are the other implementations of this interface and the response is generally: only mocks.
The interface generalization requires more plumbing code, in particular factories that generally rely on a dependency-injection framework.
Interface generalization also restricts the usage of the programming language.


=== Generalization restrictions ===
The presence of interfaces to accomplish the Dependency Inversion Pattern (DIP) has other design implications in an object-oriented program:

All member variables in a class must be in interfaces or abstracts.
All concrete class packages must connect only through interface or abstract class packages.
No class should derive from a concrete class.
No method should override an implemented method.
All variable instantiation requires the implementation of a creational pattern such as the factory method or the factory pattern, or the use of a dependency-injection framework.


=== Interface mocking restrictions ===
Using inheritance-based mocking tools also introduces restrictions:

Static externally visible members should systematically rely on dependency injection making them far harder to implement.
All testable methods should become an interface implementation or an override of an abstract definition.


== Implementations ==
Two common implementations of DIP use similar logical architecture but with different implications.
A direct implementation packages the policy classes with service abstracts classes in one library. In this implementation high-level components and low-level components are distributed into separate packages/libraries, where the interfaces defining the behavior/services required by the high-level component are owned by, and exist within the high-level component's library. The implementation of the high-level component's interface by the low-level component requires that the low-level component package depend upon the high-level component for compilation, thus inverting the conventional dependency relationship.

Figures 1 and 2 illustrate code with the same functionality, however in Figure 2, an interface has been used to invert the dependency. The direction of dependency can be chosen to maximize policy code reuse, and eliminate cyclic dependencies.
In this version of DIP, the lower layer component's dependency on the interfaces/abstracts in the higher-level layers makes reuse of the lower layer components difficult. This implementation instead "inverts" the traditional dependency from top-to-bottom to the opposite, bottom-to-top.
A more flexible solution extracts the abstract components into an independent set of packages/libraries:

The separation of each layer into its own package encourages reuse of any layer, providing robustness and mobility.


== Examples ==


=== Genealogical module ===
A genealogical system may represent relationships between people as a graph of direct relationships between them (father-son, father-daughter, mother-son, mother-daughter, husband-wife, wife-husband, etc.). This is very efficient and extensible, as it is easy to add an ex-husband or a legal guardian.
But some higher-level modules may require a simpler way to browse the system: any person may have children, parents, siblings (including half-brothers and -sisters or not), grandparents, cousins, and so on.
Depending on the usage of the genealogical module, presenting common relationships as distinct direct properties (hiding the graph) will make the coupling between a higher-level module and the genealogical module much lighter and allow changing the internal representation of the direct relationships completely without any effect on the modules using them. It also permits embedding exact definitions of siblings or uncles in the genealogical module, thus enforcing the single responsibility principle.
Finally, if the first extensible generalized graph approach seems the most extensible, the usage of the genealogical module may show that a more specialized and simpler relationship implementation is sufficient for the application(s) and helps create a more efficient system.
In this example, abstracting the interaction between the modules leads to a simplified interface of the lower-level module and may lead to a simpler implementation of it.


=== Remote file server client ===
A remote file server (FTP, cloud storage ...) client can be modeled as a set of abstract interfaces:

Connection/Disconnection (a connection persistence layer may be needed)
Folder/tags creation/rename/delete/list interface
File creation/replacement/rename/delete/read interface
File searching
Concurrent replacement or delete resolution
File history management ...
If local and remote files offers the same abstract interfaces, high-level modules that implement the dependency inversion pattern can use them indiscriminately.  The application will be able to save its documents locally or remotely transparently.
The level of service required by high level modules should be considered.
Designing a module as a set of abstract interfaces, and adapting other modules to it, can provide a common interface for many systems.


=== Model–view–controller ===

UI and ApplicationLayer packages contain mainly concrete classes. Controllers contains abstracts/interface types. UI has an instance of ICustomerHandler. All packages are physically separated.  In the ApplicationLayer there is a concrete implementation of CustomerHandler that Page class will use. Instances of the ICustomerHandler interface are created dynamically by a Factory (possibly in the same Controllers package). Concrete types Page and CustomerHandler depend on ICustomerHandler, not on each other.
Since the UI doesn't reference the ApplicationLayer or any other concrete package implementing ICustomerHandler, the concrete implementation of CustomerHandler can be replaced without changing the UI class. Also, the Page class implements interface IPageViewer which could be passed as an argument to ICustomerHandler methods, allowing the concrete implementation of CustomerHandler to communicate with UI without a concrete dependency. Again, both are linked by interfaces.


== Related patterns ==
Applying the dependency inversion principle can also be seen as an example of the adapter pattern. That is, the high-level class defines its own adapter interface which is the abstraction on which the other high-level classes depend. The adapted implementation also depends necessarily on the same adapter interface abstraction, while it can be implemented by using code from within its own low-level module. The high-level module does not depend on the low-level module, since it only uses the low-level functionality indirectly through the adapter interface by invoking polymorphic methods to the interface which are implemented by the adapted implementation and its low-level module.
Various patterns such as Plugin, Service Locator, or Dependency injection are employed to facilitate the run-time provisioning of the chosen low-level component implementation to the high-level component.


=== Implementation ===
This is an example of tightly-coupled classes (in Java:

Here, GasEngine is tightly coupled to the Car class, which violates the dependency inversion principle. To fix this, we could instead use a Engine abstraction for the Car to depend on:


== History ==
The dependency inversion principle was postulated by Robert C. Martin and described in several publications including the paper Object Oriented Design Quality Metrics: an analysis of dependencies, an article appearing in the C++ Report in June 1996 entitled The Dependency Inversion Principle, and the books Agile Software Development, Principles, Patterns, and Practices, and Agile Principles, Patterns, and Practices in C#.


== See also ==
Adapter pattern
Dependency injection
Design by contract
Interface
Inventor's Paradox
Inversion of control
Plug-in (computing)
Service locator pattern
SOLID – the "D" in "SOLID" stands for the dependency inversion principle


== References ==


== External links ==
Object Oriented Design Quality Metrics: an analysis of dependencies Robert C. Martin, C++ Report, Sept/Oct 1995
The Dependency Inversion Principle, Robert C. Martin, C++ Report, May 1996
Examining the Dependency Inversion Principle, Derek Greer
DIP: The Dependency-Inversion Principle
DIP in the Wild, Brett L. Schuchert, May 2013
Dependency Inversion Principle
A Little Architecture, Robert C. Martin (Uncle Bob), Jan 2016 - a blog on significance of Dependency Inversion Principle in software architecture

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/Dependency_inversion_principle_
