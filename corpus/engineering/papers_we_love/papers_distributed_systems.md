---
title: "经典论文导读: 分布式系统 (Distributed Systems)"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "分布式系统 (Distributed Systems)"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
  - "Papers We Love"
  - "计算机经典论文"
  - "分布式系统"
---

〔本篇为 Papers We Love 经典学术文献库关于“分布式系统 (Distributed Systems)”领域收录的传世奠基论文全景导读与本地文献档案。〕

# 一、 领域学术导读与背景

# Distributed Systems

* General Papers
* Topics
  * [Datastores](#datastores)
  * [Physics](#physics)
  * [Testing, Verification, and Correctness](#testing-verification-and-correctness)


## External Papers

* [:scroll:](a-note-on-distributed-computing.pdf) [A Note on Distributed Computing](https://www.researchgate.net/profile/Ellen-Isaacs/publication/220168963_Why_do_users_like_video/links/02e7e5186b67219c70000000/Why-do-users-like-video.pdf#page=89)

* [A simple totally ordered broadcast protocol](http://diyhpl.us/~bryan/papers2/distributed/distributed-systems/zab.totally-ordered-broadcast-protocol.2008.pdf)

* [Above the Clouds: A Berkeley View of Cloud Computing](http://www.eecs.berkeley.edu/Pubs/TechRpts/2009/EECS-2009-28.pdf)

* [Chord: A Scalable Peer-to-peer Lookup Service for Internet Applications](http://pdos.csail.mit.edu/papers/chord:sigcomm01/chord_sigcomm.pdf)

* [Kafka: a Distributed Messaging System for Log Processing](http://notes.stephenholiday.com/Kafka.pdf)

* [Large-scale cluster management at Google with Borg](http://static.googleusercontent.com/media/research.google.com/en/us/pubs/archive/43438.pdf)

* [Linearizability: A Correctness Condition for Concurrent Objects](http://cs.brown.edu/~mph/HerlihyW90/p463-herlihy.pdf)

* [Implementing Fault-Tolerant Services Using the State Machine Approach: A Tutorial](https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf)

* [Hoard: A Scalable Memory Allocator for Multithreaded Applications](http://people.cs.umass.edu/~emery/pubs/berger-asplos2000.pdf)

* [MillWheel: Fault-Tolerant Stream Processing at Internet Scale](http://static.googleusercontent.com/media/research.google.com/en/us/pubs/archive/41378.pdf)

* [Omega: flexible, scalable schedulers for large compute clusters](http://research.google.com/pubs/archive/41684.pdf)

* [Orleans: Distributed Virtual Actors for Programmability and Scalability](http://research.microsoft.com/apps/pubs/default.aspx?id=210931)

* [Paxos Made Live - An Engineering Perspective](http://www.cs.utexas.edu/users/lorenzo/corsi/cs380d/papers/paper2-1.pdf)

* [Practical Byzantine Fault Tolerance and Proactive Recovery](http://www.microsoft.com/research/wp-content/uploads/2017/01/p398-castro-bft-tocs.pdf)

* [Pregel: A System for Large-Scale Graph Processing](http://kowshik.github.io/JPregel/pregel_paper.pdf)

* [Replication, History, and Grafting in the Ori File System](http://sigops.org/sosp/sosp13/papers/p151-mashtizadeh.pdf)

* [Resilient Overlay Networks](http://nms.lcs.mit.edu/papers/ron-sosp2001.pdf)

* [Sinfonia: A New Paradigm for Building Scalable Distributed Systems](http://www.mshah.org/papers/sosp_2007_aguilera.pdf)

* [Sparrow: Distributed, Low Latency Scheduling](http://people.csail.mit.edu/matei/papers/2013/sosp_sparrow.pdf)

* [The Byzantine Generals Problem](http://www.andrew.cmu.edu/course/15-749/READINGS/required/resilience/lamport82.pdf)

* [Hashgraph Consensus: Fair, Fast, Byzantine Fault Tolerance](https://swirlds.com/downloads/SWIRLDS-TR-2016-01.pdf)

* [:scroll:](the-chubby-lock-service-for-loosely-coupled-distributed-systems.pdf) [The Chubby Lock Service for Loosely-Coupled Distributed Systems](http://static.googleusercontent.com/media/research.google.com/en/us/archive/chubby-osdi06.pdf)

* [:scroll:](join-calculus.pdf) [The Join Calculus: a Language for Distributed Mobile Programming](http://research.microsoft.com/en-us/um/people/fournet/papers/join-tutorial.pdf)

* [The Part-Time Parliament](http://research.microsoft.com/en-us/um/people/lamport/pubs/lamport-paxos.pdf)

* [There Is More Consensus in Egalitarian Parliaments](https://www.cs.cmu.edu/~dga/papers/epaxos-sosp2013.pdf)

* [Transactional Client-Server Cache Consistency: Alternatives and Performance](http://drum.lib.umd.edu/bitstream/handle/1903/751/CS-TR-3511.pdf)

* [Unicorn: A System for Searching the Social Graph](http://db.disi.unitn.eu/pages/VLDBProgram/pdf/industry/p871-curtiss.pdf)

* [Unikernels: Library Operating Systems for the Cloud](http://unikernel.org/files/2013-asplos-mirage.pdf)

* [Untraceable Electronic Mail, Return Addresses, and Digital Pseudonyms](http://www.cs.utexas.edu/~shmat/courses/cs395t_fall04/chaum81.pdf)

* [Viewstamped Replication: A New Primary Copy Method to Support Highly-Available Distributed Systems](http://www.pmg.csail.mit.edu/papers/vr.pdf)

* [VL2: A Scalable and Flexible Data Center Network](http://research.microsoft.com/pubs/80693/vl2-sigcomm09-final.pdf)

## Other Hosted Papers

* :scroll: [A History of the Virtual Synchrony Replication Model](a-history-of-the-virtual-synchrony-replication-model.pdf)

* :scroll: [A Hundred Impossibility Proofs for Distributed Systems](a-hundred-impossibility-proofs-for-distributed-computing.pdf)

* :scroll: [A response to Cheriton and Skeen's Criticism of Causal and Totally Ordered Communication](a-response-to-cheriton-and-skeens-criticism-of-causal-and-totally-ordered-communication.pdf)

* :scroll: [A Universal Modular ACTOR Formalism for Artificial Intelligence](a-universal-modular-actor-formalism-for-artificial-intelligence.pdf)

* :scroll: [A Versatile Scheme for Routing Highly Variable Traffic in Service Overlays and IP Backbones](a-versatile-scheme-for-routing-highly-variable-traffic-in-service-overlays-and-ip.pdf)

* :scroll: [Beehive: O(1) Lookup Performance for Power-Law Query Distributions in Peer-to-Peer Overlays](beehive-lookup-performance-for-power-law-query-distributions-in-peer-to-peer-overlays.pdf)

* :scroll: [Byzantine Chain Replication](byzantine-chain-replication.pdf)

* :scroll: [A Byzantine Fault Tolerant Distributed Commit Protocol](byzantine-fault-tolerant-distributed-commit-protocol.pdf)

* :scroll: [Brewer’s Conjecture and the Feasibility of Consistent, Available, Partition-Tolerant Web Services](brewers-conjecture.pdf)

* :scroll: [Chain Replication for Supporting High Throughput and Availability](chain-replication-for-supporting-high-throughput-and-availability.pdf)

* :scroll: [Commodifying Replicated State Machines with OpenReplica](commodifying-replicated-state-machines-with-openreplica.pdf)

* :scroll: [￼Consensusin the Presenceof Partial Synchrony](consensus-in-presence-of-partial-synchrony.pdf)

* :scroll: [Consistent Global States of Distributed Systems: Fundamental Concepts and Mechanisms](consistent-global-states-of-distributed-systems-fundamental-concepts-and-mechanisms.pdf)

* :scroll: [Consistent Hashing and Random Trees:
Distributed Caching Protocols for Relieving Hot Spots on the World Wide Web](consistent-hashing-and-random-trees.pdf)

* :scroll: [Copysets: Reducing the Frequency of Data Loss in Cloud Storage](copysets-reducing-the-frequency-of-data-loss-in-cloud-storage.pdf)

* :scroll: [Dapper, a Large-Scale Distributed Systems Tracing Infrastructure](dapper-a-large-scale-distributed-tracing-infrastructure.pdf)

* :scroll: [￼Distributed Snapshots: Determining Global States of Distributed Systems](distributed-snapshots-determining-global-states-of-distributed-systems.pdf)

* :scroll: [Eluding Carnivores: File Sharing with Strong Anonymity](eluding-carnivores-file-sharing-with-strong-anonymity.pdf)

* :scroll: [End-to-end arguments in system design](end-to-end-arguments-in-system-design.pdf)

* :scroll: [Epidemic Algorithms for Replicated Database Maintenance](epidemic-algorithms-for-replicated-database-maintenance.pdf)

* :scroll: [Harvest, Yield, and Scalable Tolerant Systems](harvest-yield-and-scalable-tolerant-systems.pdf)

* :scroll: [Herbivore: A Scalable and Efficient Protocol for Anonymous Communication](herbivore-a-scalable-and-efficient-protocol-for-anonymous.pdf)

* :scroll: [High-Level Specifications: Lessons from Industry](high-level-specifications--lessons-from-industry.pdf)

* :scroll: [How the Hidden Hand Shapes the Market for Software Reliability](how-the-hidden-hand-shapes-the-market-for-software-reliability.pdf)

* :scroll: [Implementing the Omega failure detector in the crash-recovery failure model](implementing-the-omega-failure-detector-in-crash-recovery-failure-model.pdf)

* :scroll: [Impossibility of Distributed Consensus with One Faulty Process](impossibility-of-consensus-with-one-faulty-process.pdf)

* :scroll: [In Search of an Understandable Consensus Algorithm](in-search-of-an-understandable-consensus-algorithm.pdf)

* :scroll: [Kelips*: Building an Efficient and Stable P2P DHT Through Increased Memory and Background Overhead](kelips-building-an-efficient-and-stable-p2p-dht-through-increased-memory-and-background-overhead.pdf)

* :scroll: [Large-scale Incremental Processing Using Distributed Transactions and Notifications](large-scale-incremental-processing-using-distributed-transactions-and-notifications.pdf)

* :scroll: [Life beyond Distributed Transactions: an Apostate’s Opinion](life-beyond-distributed-transactions-an-apostates-opinion.pdf)

* :scroll: [MapReduce: Simplified Data Processing on Large Clusters](mapreduce-simplified-data-processing-on-large-clusters.pdf)

* :scroll: [Mesos: A Platform for Fine-Grained Resource Sharing in the Data Center](mesos-a-platform-for-fine-grained-resource-sharing-in-the-data-center.pdf)

* :scroll: [Oblivious routing of highly variable traffic in service overlays and IP backbones](oblivious-routing-of-highly-variable-traffic-in-service-overlays-and-ip-backbones.pdf)

* :scroll: [On proof and progress in mathematics](on-proof-and-progress-in-mathematics.pdf)

* :scroll: [P5: A Protocol for Scalable Anonymous Communication](p5-a-protocal-for-scalable-anonymous-communication.pdf)

* :scroll: [Pastry: Scalable, decentralized object location and routing for large-scale peer-to-peer systems](pastry-scalable-decentralized-object-location-and-routing-for-large-scale-peer-to-peer-systems.pdf)

* :scroll: [Paxos Made Moderately Complex](paxos-made-moderately-complex.pdf)

* :scroll: [Paxos Made Simple](paxos-made-simple.pdf)

* :scroll: [Self-stabilizing Systems in Spite of Distributed Control](self-stabilizing-systems-in-spite-of-distributed-control.pdf)

* :scroll: [SIFT: Design and Analysis of a Fault-Tolerant Computer for Aircraft Control](sift-design-and-analysis-of-a-fault-tolerant-computer-for-aircraft-contro.pdf)

* :scroll: [Signal/Collect: Graph Algorithms for the (Semantic) Web](signal-%26-collect-graph-algorithms-for-the-\(semantic\)-web.pdf)

* :scroll: [Solution of a Problem in
Concurrent Programming Control](solution-of-a-problem-in-concurrent-programming-control.pdf)

* :scroll: [Sparse Partitions](sparse-partitions.pdf)

* :scroll: [Stronger Semantics for Low-Latency Geo-Replicated Storage](stronger-semantics-for-low-latency-geo-replicated-storage.pdf)

* :scroll: [The Akamai Network: A Platform for High-Performance Internet Applications](the-akamai-network.pdf)

* :scroll: [The Dining Cryptographers Problem:
Unconditional Sender and Recipient Untraceability](the-dining-cryptographers-problem.pdf)

* :scroll: [Tor: The Second-Generation Onion Router](tor-the-second-generation-onion-router.pdf)

* :scroll: [Towards a cloud computing research agenda](towards-a-cloud-computing-research-agenda.pdf)

* :scroll: [Understanding the Limitations of Causally and Totally Ordered Communication](understanding-the-limitations-of-causally-and-totally-ordered-communication.pdf)

* :scroll: [￼￼￼￼￼￼￼￼￼￼Viewing Control Structures as Patterns of Passing Messages](viewing-control-structures-as-patterns-of-passing-messages.pdf)

* :scroll: [Warp: Multi-Key Transactions for Key-Value Stores](../datastores/warp-multi-key-transactions-for-key-value-stores.pdf)

* :scroll: [Zab: High-performance broadcast for primary-backup systems](zab-high-performance-broadcast-for-primary-backup-systems.pdf)

* :scroll: [ZooKeeper: Wait-free coordination for Internet-scale systems](zookeeper-wait-free-coordination-for-internet-scale-systems.pdf)

* :scroll: [Tiered Replication: A Cost-effective Alternative to
Full Cluster Geo-replication](tiered-replication-a-cost-effective-alternative-to-full-cluster-geo-replication.pdf)

## Topics

### Datastores

* [Calvin: Fast Distributed Transactions for Partitioned Database Systems](http://cs.yale.edu/homes/thomson/publications/calvin-sigmod12.pdf)

* [f4: Facebook’s Warm BLOB Storage System](http://www.usenix.org/system/files/conference/osdi14/osdi14-paper-muralidhar.pdf)

* [The Case for Determinism in Database Systems](http://cs-www.cs.yale.edu/homes/dna/papers/determinism-vldb10.pdf)

* [Consistency Tradeoffs in Modern Distributed Database System Design](http://cs-www.cs.yale.edu/homes/dna/papers/abadi-pacelc.pdf)

* [Modularity and Scalability in Calvin](http://sites.computer.org/debull/A13june/calvin1.pdf)

* [Lightweight Locking for Main Memory Database Systems](http://cs-www.cs.yale.edu/homes/dna/papers/vll-vldb13.pdf)

* [Cassandra - A Decentralized Structured Storage System](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.161.6751&rep=rep1&type=pdf)

* [CRUSH: Controlled, Scalable, Decentralized Placement of Replicated Data](http://www.ssrc.ucsc.edu/Papers/weil-sc06.pdf)

* [Don’t Settle for Eventual: Scalable Causal Consistency for Wide-Area Storage with COPS](http://www.cs.cmu.edu/~dga/papers/cops-sosp2011.pdf)

* [Dremel: Interactive Analysis of Web-Scale Datasets](http://static.googleusercontent.com/media/research.google.com/en/us/pubs/archive/36632.pdf)

* [F1: A Distributed SQL Database That Scales](http://static.googleusercontent.com/media/research.google.com/en/us/pubs/archive/41344.pdf)

* [HaLoop: Efficient Iterative Data Processing on Large Clusters](http://homes.cs.washington.edu/~billhowe/pubs/HaLoop.pdf)

* [HyperDex: A Distributed, Searchable Key-Value Store](https://cs.uwaterloo.ca/~bernard/hyperdex.pdf)

* [Introduction to a System for Distributed Databases SDD-1](http://people.eecs.berkeley.edu/~wong/wong_pubs/wong73.pdf)

* [Making Reliable Distributed Systems in the Presence of Software Errors](http://www.erlang.org/download/armstrong_thesis_2003.pdf)

* [Managing Update Conflicts in Bayou, a Weakly Connected Replicated Storage System](http://www.cs.utexas.edu/~lorenzo/corsi/cs380d/papers/p172-terry.pdf)

* [Map-Reduce-Merge: Simplified Relational Data Processing on Large Clusters](http://www.cs.duke.edu/courses/cps399.28/current/papers/sigmod07-YangDasdanEtAl-map_reduce_merge.pdf)

* [MDCC: Multi-Data Center Consistency](https://amplab.cs.berkeley.edu/wp-content/uploads/2013/03/mdcc-eurosys13.pdf)

* [Optimistic replication](http://pages.cs.wisc.edu/~remzi/Classes/739/Spring2004/Papers/optimistic-survey.pdf)

* [The Dangers of Replication and a Solution](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.21.2707&rep=rep1&type=pdf)

* [Towards a Next Generation Data Center Architecture: Scalability and Commoditization](http://research.microsoft.com/pubs/79348/presto27-greenberg.pdf)

* :scroll: [Bigtable: A Distributed Storage System for Structured Data](../datastores/bigtable-a-distributed-storage-system-for-structured-data.pdf)

* :scroll: [Database Metatheory: Asking Big Queries](../datastores/database-metatheory--asking-the-big-queries.pdf)

* :scroll: [Dynamo: Amazon’s Highly Available Key-value Store](../datastores/dynamo-amazons-highly-available-key-value-store.pdf)

* :scroll: [Flat Datacenter Storage](../datastores/flat-datacenter-storage.pdf)

* :scroll: [Freenet: A Distributed Anonymous Information Storage and Retrieval System](../datastores/freenet-a-distributed-anonymous-information-and-retrieval-system.pdf)

* :scroll: [Megastore: Providing Scalable, Highly Available Storage for Interactive Services](../datastores/megastore-providing-scalable-highly-available-storage-for-interactive-services.pdf)

* :scroll: [A Solution to the Network Challenges of Data Recovery in Erasure-coded Distributed Storage Systems: A Study on the Facebook Warehouse Cluster](../datastores/network-challenges-of-data-recovery-in-erasure-coded-distributed-storage-systems.pdf)

* :scroll: [RADOS: A Scalable, Reliable Storage Service for Petabyte-scale Storage Clusters](../datastores/rados-a-scalable-reliable-storage-service-for-petabyte-scale-storage-clusters.pdf)

* :scroll: [Spanner: Google’s Globally-Distributed Database](../datastores/spanner-google's-globally-distributed-database.pdf)

* :scroll: [TAO: Facebooks Distributed Data Store for the Social Graph'](../datastores/tao-facebook-distributed-datastore.pdf)

* :scroll: [Transactional storage for geo-replicated systems](../datastores/transactional-storage-for-geo-replicated-systems.pdf)

* :scroll: [Warp: Multi-Key Transactions for Key-Value Stores](../datastores/warp-multi-key-transactions-for-key-value-stores.pdf)

* :scroll: [Spartan: A distributed array framework with smart tiling](../datastores/spartan-a-distributed-array-framework-with-smart-tiling.pdf)

### Physics

* :scroll: [“On the Electrodynamics of Moving Bodies” (1905) — Einstein](../physics/on-the-electrodynamics-of-moving-bodies.pdf)

    By solving the [asymmetries](http://en.wikipedia.org/wiki/Moving_magnet_and_conductor_problem) that arise in Maxwell’s equations, Einstein’s 1905 paper set the stage for current distributed systems work by demonstrating that there is no absolute frame of reference and by providing an upper bound on the speed of communication.

### <a name="testing-verification-and-correctness"></a>Testing, Verification, and Correctness

* :scroll: [Simple Testing Can Prevent Most Critical Failures:
An Analysis of Production Failures in Distributed
Data-Intensive Systems](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-yuan.pdf)

* :scroll: [IronFleet: Proving Practical Distributed Systems Correct](https://www.microsoft.com/en-us/research/wp-content/uploads/2015/10/ironfleet.pdf))

# 二、 核心学术论文与本地文献原件


### 本地归档核心学术论文（PDF）

- **[A History Of The Virtual Synchrony Replication Model](./raw/distributed_systems/a-history-of-the-virtual-synchrony-replication-model.pdf)** `(0.39 MB)`
- **[A Hundred Impossibility Proofs For Distributed Computing](./raw/distributed_systems/a-hundred-impossibility-proofs-for-distributed-computing.pdf)** `(2.79 MB)`
- **[A Note On Distributed Computing](./raw/distributed_systems/a-note-on-distributed-computing.pdf)** `(0.05 MB)`
- **[A Response To Cheriton And Skeens Criticism Of Causal And Totally Ordered Communication](./raw/distributed_systems/a-response-to-cheriton-and-skeens-criticism-of-causal-and-totally-ordered-communication.pdf)** `(0.25 MB)`
- **[A Universal Modular Actor Formalism For Artificial Intelligence](./raw/distributed_systems/a-universal-modular-actor-formalism-for-artificial-intelligence.pdf)** `(1.15 MB)`
- **[A Versatile Scheme For Routing Highly Variable Traffic In Service Overlays And Ip](./raw/distributed_systems/a-versatile-scheme-for-routing-highly-variable-traffic-in-service-overlays-and-ip.pdf)** `(0.24 MB)`
- **[Beehive Lookup Performance For Power Law Query Distributions In Peer To Peer Overlays](./raw/distributed_systems/beehive-lookup-performance-for-power-law-query-distributions-in-peer-to-peer-overlays.pdf)** `(0.61 MB)`
- **[Brewers Conjecture](./raw/distributed_systems/brewers-conjecture.pdf)** `(0.13 MB)`
- **[Byzantine Chain Replication](./raw/distributed_systems/byzantine-chain-replication.pdf)** `(0.29 MB)`
- **[Byzantine Fault Tolerant Distributed Commit Protocol](./raw/distributed_systems/byzantine-fault-tolerant-distributed-commit-protocol.pdf)** `(0.17 MB)`
- **[Chain Replication For Supporting High Throughput And Availability](./raw/distributed_systems/chain-replication-for-supporting-high-throughput-and-availability.pdf)** `(0.21 MB)`
- **[Commodifying Replicated State Machines With Openreplica](./raw/distributed_systems/commodifying-replicated-state-machines-with-openreplica.pdf)** `(0.21 MB)`
- **[Consensus In Presence Of Partial Synchrony](./raw/distributed_systems/consensus-in-presence-of-partial-synchrony.pdf)** `(3.04 MB)`
- **[Consistent Global States Of Distributed Systems Fundamental Concepts And Mechanisms](./raw/distributed_systems/consistent-global-states-of-distributed-systems-fundamental-concepts-and-mechanisms.pdf)** `(0.22 MB)`
- **[Consistent Hashing And Random Trees](./raw/distributed_systems/consistent-hashing-and-random-trees.pdf)** `(0.18 MB)`
- **[Copysets Reducing The Frequency Of Data Loss In Cloud Storage](./raw/distributed_systems/copysets-reducing-the-frequency-of-data-loss-in-cloud-storage.pdf)** `(2.55 MB)`
- **[Dapper A Large Scale Distributed Tracing Infrastructure](./raw/distributed_systems/dapper-a-large-scale-distributed-tracing-infrastructure.pdf)** `(1.48 MB)`
- **[Distributed Snapshots Determining Global States Of Distributed Systems](./raw/distributed_systems/distributed-snapshots-determining-global-states-of-distributed-systems.pdf)** `(0.95 MB)`
- **[Eluding Carnivores File Sharing With Strong Anonymity](./raw/distributed_systems/eluding-carnivores-file-sharing-with-strong-anonymity.pdf)** `(0.07 MB)`
- **[End To End Arguments In System Design](./raw/distributed_systems/end-to-end-arguments-in-system-design.pdf)** `(0.04 MB)`
- **[Epidemic Algorithms For Replicated Database Maintenance](./raw/distributed_systems/epidemic-algorithms-for-replicated-database-maintenance.pdf)** `(2.13 MB)`
- **[Harvest Yield And Scalable Tolerant Systems](./raw/distributed_systems/harvest-yield-and-scalable-tolerant-systems.pdf)** `(0.05 MB)`
- **[Herbivore A Scalable And Efficient Protocol For Anonymous](./raw/distributed_systems/herbivore-a-scalable-and-efficient-protocol-for-anonymous.pdf)** `(0.29 MB)`
- **[High Level Specifications  Lessons From Industry](./raw/distributed_systems/high-level-specifications--lessons-from-industry.pdf)** `(0.18 MB)`
- **[How The Hidden Hand Shapes The Market For Software Reliability](./raw/distributed_systems/how-the-hidden-hand-shapes-the-market-for-software-reliability.pdf)** `(0.06 MB)`
- **[Implementing The Omega Failure Detector In Crash Recovery Failure Model](./raw/distributed_systems/implementing-the-omega-failure-detector-in-crash-recovery-failure-model.pdf)** `(0.24 MB)`
- **[Impossibility Of Consensus With One Faulty Process](./raw/distributed_systems/impossibility-of-consensus-with-one-faulty-process.pdf)** `(0.69 MB)`
- **[In Search Of An Understandable Consensus Algorithm](./raw/distributed_systems/in-search-of-an-understandable-consensus-algorithm.pdf)** `(0.50 MB)`
- **[Ironfleet Proving Practical Distributed Systems Correct](./raw/distributed_systems/ironFleet-proving-practical-distributed-systems-correct.pdf)** `(0.45 MB)`
- **[Join Calculus](./raw/distributed_systems/join-calculus.pdf)** `(0.56 MB)`
- **[Kelips Building An Efficient And Stable P2P Dht Through Increased Memory And Background Overhead](./raw/distributed_systems/kelips-building-an-efficient-and-stable-p2p-dht-through-increased-memory-and-background-overhead.pdf)** `(0.21 MB)`
- **[Large Scale Incremental Processing Using Distributed Transactions And Notifications](./raw/distributed_systems/large-scale-incremental-processing-using-distributed-transactions-and-notifications.pdf)** `(0.21 MB)`
- **[Life Beyond Distributed Transactions An Apostates Opinion](./raw/distributed_systems/life-beyond-distributed-transactions-an-apostates-opinion.pdf)** `(0.84 MB)`
- **[Mapreduce Simplified Data Processing On Large Clusters](./raw/distributed_systems/mapreduce-simplified-data-processing-on-large-clusters.pdf)** `(0.18 MB)`
- **[Mesos A Platform For Fine Grained Resource Sharing In The Data Center](./raw/distributed_systems/mesos-a-platform-for-fine-grained-resource-sharing-in-the-data-center.pdf)** `(0.81 MB)`
- **[Oblivious Routing Of Highly Variable Traffic In Service Overlays And Ip Backbones](./raw/distributed_systems/oblivious-routing-of-highly-variable-traffic-in-service-overlays-and-ip-backbones.pdf)** `(0.92 MB)`
- **[On Proof And Progress In Mathematics](./raw/distributed_systems/on-proof-and-progress-in-mathematics.pdf)** `(0.16 MB)`
- **[P5 A Protocal For Scalable Anonymous Communication](./raw/distributed_systems/p5-a-protocal-for-scalable-anonymous-communication.pdf)** `(0.19 MB)`
- **[Pastry Scalable Decentralized Object Location And Routing For Large Scale Peer To Peer Systems](./raw/distributed_systems/pastry-scalable-decentralized-object-location-and-routing-for-large-scale-peer-to-peer-systems.pdf)** `(0.16 MB)`
- **[Paxos Made Moderately Complex](./raw/distributed_systems/paxos-made-moderately-complex.pdf)** `(0.27 MB)`
- **[Paxos Made Simple](./raw/distributed_systems/paxos-made-simple.pdf)** `(0.09 MB)`
- **[Self Stabilizing Systems In Spite Of Distributed Control](./raw/distributed_systems/self-stabilizing-systems-in-spite-of-distributed-control.pdf)** `(0.21 MB)`
- **[Sift Design And Analysis Of A Fault Tolerant Computer For Aircraft Contro](./raw/distributed_systems/sift-design-and-analysis-of-a-fault-tolerant-computer-for-aircraft-contro.pdf)** `(1.69 MB)`
- **[Signal & Collect Graph Algorithms For The (Semantic) Web](./raw/distributed_systems/signal-&-collect-graph-algorithms-for-the-(semantic)-web.pdf)** `(0.53 MB)`
- **[Simple Testing Can Prevent Most Critical Failures](./raw/distributed_systems/simple-testing-can-prevent-most-critical-failures.pdf)** `(0.59 MB)`
- **[Solution Of A Problem In Concurrent Programming Control](./raw/distributed_systems/solution-of-a-problem-in-concurrent-programming-control.pdf)** `(0.11 MB)`
- **[Sparse Partitions](./raw/distributed_systems/sparse-partitions.pdf)** `(1.02 MB)`
- **[Stronger Semantics For Low Latency Geo Replicated Storage](./raw/distributed_systems/stronger-semantics-for-low-latency-geo-replicated-storage.pdf)** `(0.47 MB)`
- **[The Akamai Network](./raw/distributed_systems/the-akamai-network.pdf)** `(0.48 MB)`
- **[The Chubby Lock Service For Loosely Coupled Distributed Systems](./raw/distributed_systems/the-chubby-lock-service-for-loosely-coupled-distributed-systems.pdf)** `(0.11 MB)`
- **[The Dining Cryptographers Problem](./raw/distributed_systems/the-dining-cryptographers-problem.pdf)** `(0.68 MB)`
- **[Tiered Replication A Cost Effective Alternative To Full Cluster Geo Replication](./raw/distributed_systems/tiered-replication-a-cost-effective-alternative-to-full-cluster-geo-replication.pdf)** `(0.66 MB)`
- **[Tor The Second Generation Onion Router](./raw/distributed_systems/tor-the-second-generation-onion-router.pdf)** `(0.17 MB)`
- **[Towards A Cloud Computing Research Agenda](./raw/distributed_systems/towards-a-cloud-computing-research-agenda.pdf)** `(0.30 MB)`
- **[Understanding The Limitations Of Causally And Totally Ordered Communication](./raw/distributed_systems/understanding-the-limitations-of-causally-and-totally-ordered-communication.pdf)** `(0.06 MB)`
- **[Viewing Control Structures As Patterns Of Passing Messages](./raw/distributed_systems/viewing-control-structures-as-patterns-of-passing-messages.pdf)** `(11.15 MB)`
- **[Zab High Performance Broadcast For Primary Backup Systems](./raw/distributed_systems/zab-high-performance-broadcast-for-primary-backup-systems.pdf)** `(1.14 MB)`
- **[Zookeeper Wait Free Coordination For Internet Scale Systems](./raw/distributed_systems/zookeeper-wait-free-coordination-for-internet-scale-systems.pdf)** `(2.26 MB)`


---
_本地归档路径: `corpus/papers_we_love/raw/distributed_systems/`_
