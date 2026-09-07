---
title: "引用文献: OS/360 and successors (EN Wikipedia)"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "https://en.wikipedia.org/wiki/OS/360"
tags:
  - "外部参考资料"
  - "黑客定律文献库"
  - "维基百科"
---

〔本文档为黑客定律与工程哲学文库中《OS/360》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: OS/360 and successors (EN Wikipedia)
- **原文链接**: [https://en.wikipedia.org/wiki/OS/360](https://en.wikipedia.org/wiki/OS/360)
- **引用锚文本**: OS/360
- **抓取状态**: success

# 二、 文献正文内容

OS/360, officially known as IBM System/360 Operating System, is a discontinued batch processing operating system developed by IBM for their then-new System/360 mainframe computer, announced in 1964; it was influenced by the earlier IBSYS/IBJOB and Input/Output Control System (IOCS) packages for the IBM 7090/7094 and even more so by the PR155 Operating System for the IBM 1410/7010 processors. It was one of the earliest operating systems to require the computer hardware to include at least one direct access storage device.
Although OS/360 itself was discontinued, successor operating systems, including the virtual storage MVS and the 64-bit z/OS, are still run as of 2023 and maintain application-level compatibility with OS/360.


== Overview ==
IBM announced three different levels of OS/360, generated from the same tapes and sharing most of their code. IBM eventually renamed these options and made some significant design changes:

Single Sequential Scheduler (SSS)
Option 1
Primary Control Program (PCP)
Multiple Sequential Schedulers (MSS)
Option 2
Multiprogramming with a Fixed number of Tasks (MFT)
MFT II
Multiple Priority Schedulers (MPS)
Option 4
VMS
Multiprogramming with a Variable number of Tasks (MVT)
Model 65 Multiprocessing (M65MP)
Users often coined nicknames, e.g., "Big OS", "OS/MFT", but none of these names had any official recognition by IBM.
IBM provided OS/360 as a set of libraries on tape that the installation had to restore to DASD in order to perform a system generation. IBM also offered a set of optional source tapes that the installation could use to modify and assemble modules that IBM normally provided as object code. In addition, IBM offered microfiche that had assembly listing of the basic program material and of subsequent service. IBM continued distributing source code until it imposed an Object Code Only (OCO) policy for licensed software.
The other major operating system for System/360 hardware was DOS/360.
OS/360 is in the public domain and can be downloaded freely. As well as being run on actual System/360 hardware, it can be executed on the free Hercules emulator, which runs under most UNIX and Unix-like systems including Linux, Solaris, and macOS, as well as Windows. There are OS/360 turnkey CDs that provide pregenerated OS/360 21.8 systems ready to run under Hercules.


== Origin ==

IBM originally intended that System/360 should have only one batch-oriented operating system, OS/360, capable of running on machines as small as 32 KiB. It also intended to supply a separate timesharing operating system, TSS/360, for the System/360 Model 67. IBM failed to meet OS/360's memory usage goal, and it required 44 KiB for even limited production. There are at least two accounts of why IBM eventually decided to produce other, simpler batch-oriented operating systems:

because it found that the "approximately 1.5 million instructions that enable the system to operate with virtually no manual intervention" comprising OS/360 would not fit into the limited memory available on the smaller System/360 models; or
because it realized that the development of OS/360 would take much longer than expected.
IBM introduced a series of stop-gaps to prevent System/360 hardware sales from collapsing—first Basic Programming Support (BPS) and BOS/360 (Basic Operating System, for the smallest machines with 8K byte memories), then TOS/360 (Tape Operating System, for machines with at least 16K byte memories and only tape drives), and finally DOS/360 (Disk Operating System), which became a mainstream operating system and is the ancestor of today's widely used VSEn.
IBM released three variants of OS/360: PCP (Primary Control Program), a stop-gap which could run only one job at a time, in 1966; MFT (Multiprogramming with Fixed number of Tasks) for the mid-range machines, and MVT (Multiprogramming with Variable number of Tasks) for the top end. MFT and MVT were used until at least 1981, a decade after their successors had been launched. The division between MFT and MVT arose because of storage limitations and scheduling constraints. Initially IBM maintained that MFT and MVT were simply "two configurations of the OS/360 control program", although later IBM described them as "separate versions of OS/360".
IBM originally wrote OS/360 in assembly language. Later on, IBM wrote some OS/360 code in a new language, Basic Systems Language (BSL), derived from PL/I. A large amount of the TSO code in Release 20 was written in BSL.
TSS/360 was so late and unreliable that IBM canceled it, although IBM later supplied three releases of the TSS/370 PRPQ. By this time CP-67 was running well enough for IBM to offer it without warranty as a timesharing facility for a few large customers.


== OS/360 variants ==
These three options offered such similar facilities that porting applications between them usually required minimal effort; the same versions of most IBM Program Products, application and utility software ran on both. The text below mostly treats PCP, MFT and MVT as simply new names for the original SSS, MSS and MPS, although there were some design changes. Also, the text does not distinguish between M65MP and MVT.
Officially, PCP, MFT and MVT are not separate operating systems from OS/360,they are only install-time configuration options—in today's words, three different variants of the OS Nucleus and Scheduler.  However, because of quite different behavior and memory requirements, users commonly consider them de facto separate operating systems and refer to them as "early OS/360", "OS/MFT", "OS/MVT", respectively. MFT differs from MVT mainly in the way in which it manages memory: when installing MFT, customers specify in the system generation (SysGen) a fixed number of  partitions, areas of memory with fixed boundaries, in which application programs can be run simultaneously.


=== PCP ===
Primary Control Program (PCP) was intended for machines with small memories. It is similar to MFT with one partition. Experience indicated that it was not advisable to install OS/360 on systems with less than 128 KiB of memory, although limited production use was possible on much smaller machines, such as 48 KiB of memory. IBM dropped the PCP option in the final releases of OS/360, leaving only MFT II and MVT, both of which required more memory.
Also referred to as SYS=MIN in macro expansions that were system-dependent.


=== MFT ===
Multiprogramming with a Fixed number of Tasks (MFT) was intended to serve as a stop-gap until Multiprogramming with a Variable number of Tasks (MVT), the intended target configuration of OS/360, became available in 1967. Early versions of MVT had many problems, so the simpler MFT continued to be used for many years. After introducing new System/370 machines with virtual memory in 1972, IBM developed MFT 2 into OS/VS1, the last system of this particular line.
The first version of MFT shared much of the code and architecture with PCP, and was limited to four partitions. It was very cumbersome to run multiple partitions. Many installations used Houston Automatic Spooling Priority (HASP) to mitigate the complexity.
MFT Version II (MFT-II) shared much more of the Control Program and Scheduler code with MVT, and was much more flexible to run. The maximum number of partitions increased to 52.
Later modifications of MFT-II added sub-tasking, so that the fixed number of tasks was no longer fixed, although the number of partitions did remain a limitation.
Experience indicated that it was not advisable to install MFT on systems with less than 256 KiB of memory, which in the 1960s was quite a large amount.
Also referred to as SYS=INT in macro expansions that were system-dependent.


=== MVT ===
Multiprogramming with a Variable number of Tasks (MVT) was the most sophisticated of three available configurations of OS/360's control program, and one of two available configurations in the final releases. MVT was intended for the largest machines in the System/360 family.  Introduced in 1964, it did not become available until 1967. Early versions had many problems and the simpler MFT continued to be used for many years. Experience indicated that it was not advisable to install MVT on systems with less than 512 KiB of memory.
MVT treated all memory not used by the operating system as a single pool from which contiguous regions could be allocated as required, by an unlimited number of simultaneous application and systems programs.  This scheme was more flexible than MFT's and in principle used memory more efficiently, but was liable to fragmentation—after a while one could find that, although there was enough spare memory in total to run a program, it was divided into separate chunks none of which was large enough.  System/360 lacked memory relocation hardware so memory compaction could not be used to reduce fragmentation.  A facility called Rollout/Rollin could swap a running job out to secondary storage to make its memory available to another job.  The rolled-out job would, however, have to be rolled-in to the original memory locations when they again became available.
In 1971 the Time Sharing Option (TSO) for use with MVT was added as part of release 20.1.  TSO became widely used for program development because it provided an editor,  the ability to submit batch jobs, be notified of their completion, and view the results without waiting for printed reports, and debuggers for some of the programming languages used on System/360. TSO in OS/360 communicated with terminals by using Telecommunications Access Method (TCAM).  TCAM's name suggests that IBM hoped it would become the standard access method for data communications, but in fact TCAM in OS/VS2 was used almost entirely for TSO and was largely superseded by Virtual Telecommunications Access Method (VTAM) in the mid-to-late 1970s.
Also referred to as SYS=VMS in invocations of some macros that were system-dependent.


==== M65MP ====
Model 65 Multiprocessing (M65MP) is a variant of MVT. It runs on a 360/65 in Multisystem mode M65MP traps use of the Set System Mask (SSM) instruction to serialize disabled code between the two CPUs. For the most part an M65MP system has the same behavior and interfaces as any other MVT system.
The keyword parameter SYS=VMS included M65MP as well as uniprocessor MVT.


=== Shared features ===
PCP, MFT and MVT provide similar facilities from the point of view of application programs:

The same application programming interface (API) and application binary interface (ABI), so application programs can be transferred between MFT and MVT without even needing to be modified or re-assembled or re-compiled.
The same JCL (Job Control Language, for initiating batch jobs), which was more flexible and easier to use, though more complex, than that of DOS/360.
The same facilities (access methods) for reading and writing files and for data communications:
Sequential data sets are normally read or written one record at a time from beginning to end, using BSAM or QSAM. This was the only technique that could be used for tape drives, card readers / punches and printers.
In indexed (ISAM) files a specified section of each record is defined as a key which can be used to look up specific records.
In direct access (BDAM) files, the application program has to specify the relative block number, the relative track and record (TTR) or the actual physical location (MBBCCHHR) in a Direct-access storage device (DASD) of the data it wanted to access, or the starting point for a search by key. BDAM programming was not easy and most organizations never used it themselves; but it was the fastest way to access data on disks and many software companies used it in their products, especially database management systems such as ADABAS, IDMS and IBM's DL/I. It is also available from OS/360 Fortran. BDAM datasets are unblocked, with one logical record per physical record.
An additional file structure, partitioned, and access method (BPAM), is mainly used for managing program libraries.  Although partitioned files need to be compressed to reclaim free space, this has less impact than did a similar requirement for DOS/360's Core Image Library, because MFT and MVT allow multiple partitioned datasets and each project generally has at least one.
Generation Data Groups (GDGs) were originally designed to support grandfather-father-son backup procedures: if a file was modified, the changed version became the new son, the previous son became the father, the previous father became the grandfather and the previous grandfather was deleted. But one could set up GDGs with more than 3 generations, and some applications used GDGs to collect data from large and variable numbers of sources and feed the information to one program; each collecting program created a new generation of the file and the final program read the whole group as a single sequential file (by not specifying a generation in the JCL).
BTAM, a data communications facility, was primitive and hard to use by today's standards. However, it could communicate with almost any type of terminal, which was a big advantage at a time when there was hardly any standardization of communications protocols.
The file naming system allows files to be managed as hierarchies with at most 8 character names at each level, e.g. PROJECT.USER.FILENAME. This is tied to the implementation of the system catalog (SYSCTLG) and Control Volumes (CVOLs), which used records with 8 byte keys.


==== Compilers, service aids and utilities ====

OS/360 provides the same languages, service aids and utilities for PCP, MFT and MVT. The OS/VS systems drop sort/merge and all language processors, but provide a new assembler, IFOX00 (Assembler XF). IBM released program products to replace most of the free language processors and some of the other support programs. OS/360 supports the following

Assembler
ALGOL 60
COBOL
FORTRAN IV
PL/I
RPG


=== Shared features excluding PCP ===
Some features were available only for MFT and MVT:

A SPOOLing facility for MFT II and MVT (which DOS/360 initially lacked, but was, later, provided by the POWER application).
Applications in MFT (Release 19 and later) and MVT could create sub-tasks, which allowed multitasking (multithreading) within the one job.
Graphic Job Processing
Satellite Graphic Job Processing
Remote Job Entry
Queued Telecommunications Access Method (QTAM)
Telecommunications Access Method (TCAM)


== System/370 and virtual memory operating systems ==
When System/370 was announced in 1970 it offered essentially the same facilities as System/360 but with about 4 times the processor speeds of similarly priced System/360 CPUs. Then in 1972 IBM announced System/370 Advanced Functions, of which the main item was that future sales of System/370 would include virtual memory capability and this could also be retro-fitted to existing System/370 CPUs. Hence IBM also committed to delivering enhanced operating systems which could support the use of virtual memory.


=== OS/360 ===
IBM provided an OS/360 SYSGEN option for S/370 support, which did not support DAT but did:

Support control registers
Support enhanced I/O
Provide a S/370 Machine Check Handler
Provide limited support for the new timer facilities


=== OS/VS1 ===

OS/VS1 is the successor to MFT, and offers similar facilities with several additions, e.g., RES, virtual memory. VSAM (see below) was initially available as an independent component release (ICR) and later integrated into the OS/VS1 base. 
IBM intended OS/VS1 to manage a medium-sized work load (for the 1970s) consisting only of batch processing applications, running within a fixed number of operating system partitions via the batch job management system Job Entry Subsystem 1 (JES1), which replaced the spooling facilities of OS/360. However, OS/VS1 could, and often did, support interactive applications and users by running IBM's CICS transaction processing monitor as a job within one of its partitions.
IBM released fairly minor enhancements of OS/VS1 until 1983, and in 1984 announced that there would be no more. AIX/370, AIX/ESA, DPPX, IX/370,  OS/VS1 and TSS/370 are the only System/370 operating systems that do not have modern descendants.


==== Basic Programming Extensions (BPE) ====
OS/VS1 Basic Programming Extensions (BPE), product 5662-257, provides support for new 1980s hardware, such as 3380 Direct Access Storage, and for VM handshaking between VTAM and VM/VTAM Communications Network Application (VCNA).


=== OS/VS2 SVS and MVS ===
OS/VS2 release 1 was just MVT plus virtual memory and VSAM (see below).  This version was eventually renamed OS/VS2 SVS, for Single Virtual Storage, when OS/VS2 Release 2, also known as MVS, for Multiple Virtual Storage, was introduced.  SVS was intended as a stepping stone from MVT to MVS, and is only of historical interest today.
In 1974 IBM released what it described as OS/VS2 Release 2 but which was really a new operating system that was upwards-compatible with OS/VS2 Release 1. The Supervisor of the new system had been largely rewritten in a new dialect of BSL, PL/S; BSL and PL/S were dialects of PL/I with extensions designed to transcribe Assembly language code, including privileged instructions needed to control the computer as a whole. Time-sensitive OS components, such as the OS Dispatcher and the IOS, notably, among many others, remained coded in Assembly Language, which had been enhanced for OS/VS in the IFOX00 Assembler (from the older, OS/360 IEUASM Assembler).

The new version's most noticeable feature was that it supported multiple virtual address spaces: different applications thought they were using the same range of virtual addresses, but the new system's virtual memory facilities mapped these to different ranges of real memory addresses. Each application's address space consists of 3 areas: operating system (one instance shared by all jobs); an application area which was unique for each application; shared virtual area used for various purposes including inter-job communication. IBM promised that the application areas would always be at least 8MB. This approach eliminated the risk of memory fragmentation that was present in MVT and SVS, and improved the system's internal security. The new system rapidly became known as "MVS" (Multiple Virtual Storages), the original OS/VS2 became known as "SVS" (Single Virtual Storage) and IBM itself accepted this terminology and labelled MVS's successors "MVS/xxx".
MVS introduced a new approach to workload management, allowing users to define performance targets for high-priority batch jobs. This enabled users to give their systems more work than before without affecting the performance of the highest-priority jobs.
MVS was IBM's first mainstream operating system on the System/370 to support what IBM called tightly coupled multiprocessing, in which 2 (later, up to 12, for IBM mainframes, and up to 16, for Amdahl mainframes) CPUs shared concurrent access to the same memory (and a single copy of the operating system and peripheral devices), providing greater processing power and a degree of graceful degradation if one CPU failed (which, fortunately, became an increasingly rare event, as system up time rose from hours to days and, then, to years.)
Initially MVS was supplied with a job queue manager called JES2 (Job Entry Subsystem 2), which was descended from HASP (Houston Automatic Spooling Priority) and also supported Remote Job Entry from workstations located elsewhere. JES2 can only manage jobs for one CPU (which might be a tightly coupled multiprocessor system). In 1976 IBM provided another option, JES3 (Job Entry Subsystem 3), a descendant of ASP (Attached Support Processor), which allows one CPU to manage a single job queue feeding work to several physically distinct CPUs, and therefore allows one operator's console to manage the work of all those CPUs. Note: JES1 was the job queue manager for OS/VS1 (see above).


=== VSAM ===
IBM hoped that Virtual storage access method (VSAM) would replace its earlier sequential, indexed and direct access methods as it provided improved versions of these:

Entry-Sequenced Datasets (ESDS) provide facilities similar to those of both sequential and BDAM (direct) datasets, since they can be read either from start to finish or directly by specifying an offset from the start.
Key-Sequenced Datasets (KSDS) are a major upgrade from IBM's ISAM: they allow secondary keys with non-unique values and keys formed by concatenating non-contiguous fields in any order; they greatly reduce the performance problems caused by overflow records used to handle insertions and updates in ISAM; and they greatly reduce the risk that a software or hardware failure in the middle of an index update might corrupt the index. VSAM provides an ISAM / VSAM Interface which allows ISAM-based applications to use VSAM KSDS without reprogramming.
Relative Record Datasets (RRDS) are a replacement for direct access (BDAM) datasets, allowing applications to access a record by specifying a relative record number. Unlike ESDS and KSDS, RRDS does not support variable-length records.
These VSAM formats became the basis of IBM's database management systems, IMS/VS and DB2 (usually ESDS for the actual data storage and KSDS for indexes).
VSAM also provides a new implementation of the catalog facility which enables applications to access files by name, without needing to know which disk drive(s) they are on. VSAM datasets must be defined in a VSAM catalog before they are used, and non-VSAM datasets can also be listed in a VSAM catalog. The MVS Master Catalog must be a VSAM catalog.  Catalogs were originally provided in OS/360 in the form of CVOLs; MVS added a separate catalog structure for VSAM; later IBM added a third type of catalog known as an ICF catalog. (IBM removed support for CVOL and VSAM catalogs as of 2000, since they were not Y2K-compliant; hence in z/OS, only ICF catalogs are supported.)


=== SNA ===
In 1974, IBM announced Systems Network Architecture, which was meant to reduce the cost of running large networks of terminals, mainly by using communications lines much more efficiently. This is only available for IBM's virtual memory operating systems, since its mainframe software component, VTAM, is only available with these operating systems.


== Later MVS versions and enhancements ==

In 1977 IBM announced MVS/System Extensions, a program product (i.e., it cost extra money) which improved MVS performance and added functionality.
Descendants of the original MVS are still used on the latest descendants of System/360, System/390 and zSeries; it was renamed to OS/390 for System/390, and the 64-bit version for the zSeries was named z/OS.


== Structure, interface and logic ==

For reasons of size, this section concentrates on the MVT option of OS/360; PCP and MFT are very similar; with changes in nomenclature and some minor differences. OS/VS1 and SVS have much the same structure as MFT II and MVT, while MVS, although retaining much of the logic of MVT, has major enhancements better addressed in a separate article. This article adheres to IBM's usage of the term storage rather than memory.


=== CPU allocation ===
OS/360 assigns processors to tasks, which are analogous to light-weight processes or threads in other systems. Each task has a Task Control Block (TCB) and a stack of Request Blocks (RBs). A task is either dispatchable or nondispatchable and an RB is either waiting or not waiting. The Dispatcher selects the highest priority dispatchable task whose current RB is not waiting.
MVS assigns processors to address spaces, which are analogous to processes, and to Service Request Block (SRBs) and tasks within address spaces. Each address space has an Address Space Control Block (ASCB), a queue of SRBs and a queue of TCBs.


=== Storage layout ===

In OS/360 all storage is visible to all code, although fetch protection may prevent access by unprivileged code to some control blocks or between jobs.
Main storage for MVT is divided into a system (fixed) area at the bottom of real storage, a common area at the top and a private area in the middle. These contain the following areas.

Fixed area
This area starts at absolute location 0 and contains
The Nucleus.
This is a section of storage at location 0, loaded from SYS1.NUCLEUS(IEANUCxx), where xx is normally 01. For Model 65 MP (M65MP), there is a separate copy of the prefix (locations 0-4095) for each processor and the term absolute address refers to the address actually sent to memory after any prefixing while the term real address refers to the address before any prefixing is applied. The Nucleus contains interrupt handlers, control blocks, type 1 and type 2 SVC routines, SVC transient area, routines that can be directly called by unprivileged code and routines that can only be called by privileged code.
System Queue Area (SQA).
This is an area from which the system acquires storage shared between jobs.
Private area.
This an area from which MVT acquires storage for regions. The types of regions include
Job
Allocated by the Initiator for batch jobs submitted by, e.g., card reader, CRJE, RJE, TSO
Foreground
Contains a swapped-in TSO session. Created by the Time Sharing Control (TSC) task.
Mount
Allocated by Started Task Control (STC) for operator MOUNT command.
Started task
Allocated by Started Task Control (STC) for operator START command.
Common area
'This is a section of storage at the highest physical address. It contains

Resident BLDL area
This is an area caching selected directory entries.
Master Scheduler region
The Master Scheduler and the Communication task run in this region, which NIP allocates.
Link Pack Area (LPA)
This is an area into which MVT loads reentrant access method routines and other programs at IPL time that can subsequently be shared by all jobs.


==== SVS storage layout ====
SVS is similar except that the system area is at the top and bottom of virtual storage rather than real storage, the Nucleus is in Virtual=Real (V=R) storage, the private area contains a Local System Queue Area (LSQA) for each region and the LPA is split into: 

Pageable Link Pack Area (PLPA)
This is an area containing all of the modules in SYS1.LPALIB; the page dataset backing it up is retained across IPLs, except when the create LPA (CLPA) option is specified.
Modified Link Pack Area (MLPA)
This is an area containing modules listed in an IEALPAxx member of SYS1.PARMLIB and replacing modules in the PLPA for the duration of an IPL.
Fixed Link Pack Area (FLPA).
This is an area containing page-fixed modules listed in an IEAFIXxx member of SYS1.PARMLIB and replacing modules in the PLPA for the duration of an IPL.


==== MVS/370 Storage layout ====
In MVS all address spaces share the system area but not the private area. MVS adds:

Common Storage Area (CSA)
This is an area from which MVS can dynamically allocate storage shared by all address spaces.
System Work Area (SWA)
This is an area containing control blocks that in OS/360 and SVS had been contained in SYS1.SYSJOBQE.


==== MVS/XA, MVS/ESA and OS/390 storage layout ====
In MVS/XA and later, there are system, private and common areas below the 16 MiB (224 bytes) line, and extended areas between 16 MiB and 2 GiB (231 bytes).


==== z/OS storage layout ====
In z/OS, the storage layout is similar to that of MVS/XA, MVS/ESA and OS/390; storage above the 2 GiB bar is managed by different services than storage below the bar.


=== System job queue, SYSIN and SYSOUT ===
OS/360 stores the system job queue as 176 byte records in SYS1.SYSJOBQE. Other than the control blocks used for JCL and messages, OS/360 SPOOL processing uses normal temporary DASD datasets pointed to by Dataset Blocks (DSB) on the job queue.
A job may have associated System Input (SYSIN) datasets created by the Reader/Interpreter from in-stream data on cards, DASD or tape, and system output (SYSOUT) files created by the job; the term SYSOUT also includes the messages associated with the job. A job may have SYSOUT in multiple output classes, and the system output Writer processes each output class separately.
The support in OS/VS2 R1 (SVS) is essentially the same. The programs Attached Support Processor (ASP) and Houston Automatic Spooling Priority (HASP) usurp the spooling functions of OS/360 and SVS, maintaining SYSIN and SYSOUT datasets with their own SPOOL mechanisms.
In OS/VS1, SYSIN and SYSOUT datasets are managed by Job Entry Subsystem 1 (JES1), retaining many of the same operator commands but replacing the SPOOL mechanism.
In MVS, OS/VS2 R2 and later, SYSIN and SYSOUT datasets are managed by a Job Entry Subsystem (JES2 or JES3), retaining many of the HASP or ASP operator commands and providing their own SPOOL mechanisms.


=== IPL process ===
When the operators initially selects LOAD, The system sends a READ IPL command to the selected device, reading 24 bytes from cylinder 0, track 0, record 1, into storage location 0. Bytes 8-23 read and transfer to the bootstrap record cylinder 0, track 0, record 2, which in turn reads and transfers to the IPL Loader. The IPL Loader does initial housekeeping, locates the requested nucleus and loads it with relocation; the interrupt handler csect is always at location 0 and the Nucleus Initialization csect is always last, so that it can eventually be discarded. The Nucleus Initialization Program (NIP) initializes various system components.
Initially NIP must provide services that it needs. However, as it initializes various OS facilities it begins using standard services, e.g., it uses the OBTAIN macro to read a Data Set Control Block (DSCB) from a Volume Table of Contents (VTOC).
NIP reads members from the SYS1.PARMLIB dataset to obtain various operational data. These members may be specified by default, by the operator, or by parameters in other members.
NIP eventually creates a Master Schedule region from the private area and transfers to IEEVIPL to initialize the Master Scheduler and Communication Task, which completes system Initialization.


=== Job-like units of work ===
In addition to batch jobs, OS/360 supports several categories of work that it handles similarly to batch jobs. In all cases the system has to process JCL, allocate a region, allocate devices and initiate the job-step program. For a batch job, the Reader/Interpreter processes JCL from an external device; for START commands,  MOUNT commands  and TSO LOGON, the system generates JCL invoking a cataloged procedure.
For each job-like unit of work, the Reader/Interpreter converts the JCL to control blocks (tables) on SYS1.SYSJOBQE (system job queue) and the Initiator uses those control blocks to run the job. The details below are for MVT and SVS; PCP does not support spooling and MFT partition handling is somewhat different from MVT region handling.


==== Reader/Interpreter ====
The Reader/Interpreter (R/I) performs two functions, depending on how the system calls it.
When the OS initializes the Master Scheduler, processes a MOUNT command, processes a START command or processes a TSO LOGON, the system generates JCL invoking a cataloged procedure and invokes the R/I, followed by the Initiator.
For a Reader procedure, the R/I creates a separate input job entry on the job queue for each job in the input stream. The R/I recognizes explicit DD *, implicit DD* and explicit DD DATA in-stream (SYSIN) data sets, and allocates a separate DASD dataset for each, using a special DSNAME that includes the job name and a timestamp.


==== Initiator ====
The Initiator performs two functions, depending on how the system calls it.
For Master Scheduler Initiation, processing commands and processing TSO LOGONs, the system calls the Initiator to run the job just constructed by the R/I, in a newly acquired region.
For batch jobs, the Initiator selects work from the input queue, acquires a region for each step and runs the steps in sequence, skipping steps when requested in the JCL.


==== Writer ====
The system output writer takes jobs from the job queue and transcribes the SYSOUT data, usually to a printer, card punch or tape drive. It selects work based on, e.g., output class.


=== Interfaces ===


==== API ====
OS/360 has a control block known as the Communications Vector Table (CVT), which contains pointers to other control blocks and to various routines. Some of the OS/360 macro-instructions refer to fields in the CVT and other control blocks.
OS/360 has macros to provide dsect mappings of some control blocks, although many macros have hard coded offsets rather than the names in the mapping macros.
OS/360 services typically have parameters in register 1; some use registers 0 and 1 and some use registers 0, 1 and 15.
OS/360 service macros often have three forms:

Execute form
Perform the function using a provided parameter list
List form
Generate a parameter list with preset parameters that can be used by an execute form macro.
Standard form
Generate any required parameter list and perform the function.
Many of the important services are implemented in SVC routines, and preserve registers 2-14. Others are implemented as directly callable subroutines, with entry addresses in system control blocks, and require that Register 13 point to a standard save area; these typically preserve registers 2–13. In either case, register 15 at exit normally contains a return code; many services return additional data in registers 0 and 1.


===== Synchronization =====

OS/360 relies heavily on serialization using an Event Control Block (ECB), which represents an event that can be waited for; an ECB contains a completion code for an event or the address of a Request Block (RB) waiting for that event. The WAIT macro puts a task into a wait state until the specified events occur; the POST macro marks an ECB as complete, stores the completion code into the ECB and decrements the wait count in the waiting RB, if any, possibly causing the associated task to be dispatched.
MVS adds other synchronization methods.


===== Serialization =====

OS/360 uses the ENQ and DEQ to serialize access to resources, identified by a queue name (qname or major) and resource name (rname or minor). The ENQ macro delays a task until all of the requested resources are available. The DEQ macros returns named resources previously requested by an ENQ. A DEQ need not release all resources obtained by corresponding ENQ requests, but subsequent DEQ requests must eventually release all of them.
MVS adds other serialization methods.


==== Operator consoles ====
OS/360 requires at least one operator console; there are two levels of support

Primary
This is a basic level of support allowing only one primary console, one optional alternate console
Multiple Console Support (MCS)
MCS allows one master console and up to 31 secondary consoles. Any of the 32 consoles may have an alternate console to receive its message traffic in the event it fails. MCS allows recording of message traffic on a hardcopy log, which may be either a secondary console or the system log (SYSLOG) on DASD. MCS includes Device Independent Display Operator Console Support (DIDOCS), a unified framework for supporting CRT-based consoles.
Each message issued by a Write To Operator (WTO) or Write To Operator with Reply (WTOR) has associated with it one or more routing codes. The operator can limit a console to displaying only specific routing codes. A typical use for this would be placing a console in a tape library and displaying only messages relevant to the tape librarian on it. The special routing code 7 causes the message text to be included in the job's message log.
Each message also has a descriptor that controls how it is processed.
One of the key operator commands is REPLY, which provides a response to the WTOR macro. The operator can only reply to a WTOR at a console that received its message text.
A key command for display consoles, e.g., 2250, 2260, 3270, is CONTROL (K), which sets processing options for, e.g., scrolling.


==== JCL ====


== Timeline ==
These data are taken from IBM 360 Operating Systems Release History, System/370 Market Chronology of Products & Services,
IBM. "z/OS, z/OS.e, and OS/390 marketing and service announce, availability, and withdrawal dates". Archived from the original on March 14, 2008. and IBM announcement letters.


== See also ==
History of IBM mainframe operating systems


=== References in popular culture ===
ABEND


== Notes ==


== References ==


=== Citations ===


=== OS/360 manuals ===
DMMAC
OS Data Management Macro Instructions - Release 21.7 (PDF). IBM Systems Reference Library (Second ed.). IBM. June 1973. GC26-3794-1. Retrieved June 7, 2022.
DMSVC
OS Data Management Macro Services Guide - Release 21.7 (PDF). IBM Systems Reference Library (Third ed.). IBM. July 1973. GC26-3746-2. Retrieved June 7, 2022.
DMSYS
OS Data Management for System Programmers - Release 21 (PDF). IBM Systems Reference Library (Twelfth ed.). IBM. April 1973. GC28-6550-11. Retrieved June 7, 2022.
INTRO
OS/360 Introduction (PDF). Systems Reference Library (Fourth ed.). IBM. 1972. GC28-6534-3. Retrieved July 7, 2022.
Technical Newsletter GN28-2512. January 15, 1972.
JCLREF
IBM System/360 Operating System: Job Control Language Reference - OS Release 21.7 (PDF). IBM Systems Reference Library (Fiflth ed.). IBM. August 1976. GC28-6704-4. Retrieved June 7, 2022.
JCLUSER
IBM System/360 Operating System: Job Control Language User's Guide (PDF). IBM Systems Reference Library (Third ed.). IBM. June 1971. GC28-6703-2. Retrieved June 7, 2022.
LKED
IBM OS - Linkage Editor and Loader - Program Numbers 360S-ED-510 - 360S-ED-521 - 360S-LD-547 (PDF). IBM Systems Reference Library (Eleventh ed.). IBM. April 1973. GC28-6538-10. Retrieved June 7, 2022.
MFT
IBM System/360 Operating System: MFT Guide - OS Release 21.7 (PDF). IBM Systems Reference Library (Eleventh ed.). IBM. August 1974. GC27-6939-10. Retrieved June 7, 2022.
MVT
IBM System/360 Operating System: MVT Guide - OS Release 21.7 (PDF). IBM Systems Reference Library (Sixth ed.). IBM. August 1974. GC28-6720-5. Retrieved June 7, 2022.
IBM System/360 Operating System: Operator's Reference - OS Release 21 (PDF). Systems Reference Library. IBM. Retrieved July 21, 2025.
SMF
OS SMF (PDF). IBM Systems Reference Library (Eighth ed.). IBM. April 1973. GC28-6712-7. Retrieved June 7, 2022.
SUP
OS Release 21 - IBM System/360 Operating System - Supervisor Services and Macro Instructions (PDF). IBM Systems Reference Library (Eighth ed.). IBM. September 1974. GC28-6646-7. Retrieved June 7, 2022.
SYSCB
IBM System/360 Operating System: System Control Blocks (PDF). IBM Systems Reference Library (Tenth ed.). IBM. April 1973. GC28-6628-9. Retrieved June 7, 2022.
SYSGEN
OS System Generation Release 21.8 (PDF). IBM Systems Reference Library (Fourteenth ed.). IBM. August 1974. GC28-6554-13. Retrieved June 7, 2022.
SYSPG
IBM System/360 Operating System: System Programmer's Guide (PDF). IBM Systems Reference Library (Ninth ed.). IBM. June 1971. GC28-6550-9. Retrieved June 7, 2022.
TSO
IBM System/360 Operating System: Time Sharing Option Guide - OS Release 21.7 (PDF). IBM Systems Reference Library (Eighth ed.). IBM. April 1973. GC28-6698-7. Retrieved June 7, 2022.
TSOCMD
IBM System/360 Operating System: Time Sharing Option - Command Language Reference - OS Release 21.7 (PDF). IBM Systems Reference Library (Fifth ed.). IBM. April 1973. GC28-6732-4. Retrieved June 7, 2022.


=== OS/360 logic manuals ===
IOS 2nd ed
IBM System/360 Operating System - Input/Output Supervisor - Program Number 360S-CI-535 (PDF). Program Logic (Second ed.). IBM. April 1967. GY28-6616-1. Retrieved June 7, 2022.
IOS
OS I/O Supervisor Logic - Release 21 - Program Number 360S-CI-535 (PDF). Program Logic (Tenth ed.). IBM. February 1972. GY28-6616-9. Retrieved June 7, 2022.
IPL
IBM System/360 Operating System - Initial Program Loader and Nucleus Initialization Program - Program Number 360S-CI-535 (PDF). Program Logic (Sixth ed.). IBM. March 1972. GY28-6661-5. Retrieved June 7, 2022.
PCPJOB
IBM System/360 Operating System: Job Management, - Program Logic Manual, - Program Number 360S-CI-505 (PDF) (Sixth ed.). IBM. June 1970. GY28-6613-5. Retrieved June 7, 2022.
MVTJOB
IBM System/360 Operating System: MVT Job Management, Program Logic Manual, - Program Number 360S-CI-535 OS Release 21 (PDF) (Tenth ed.). IBM. March 1972. GY28-6660-9. Retrieved June 7, 2022.
MVTSUP
IBM System/360 Operating System - MVT Supervisor (PDF). Program Logic (Eighth ed.). May 1973. GY28-6659-7. Retrieved June 7, 2022.


== Further reading ==


=== Manuals ===
IBM, "MVT Guide" - GC28-6720-4, R21, March 1972
IBM, "MVT Supervisor PLM" - GY28-6659-7, Program Logic Manual, March 1972
IBM, "OS I/O Supervisor PLM" - GY28-6616-1, Program Logic Manual, April 1967
IBM, "OS I/O Supervisor PLM" - GY28-6616-9, Program Logic Manual, R21.7, April 1973


=== Books ===
Brooks, Jr., Frederick P. (1975). "The Mythical Man-Month: Essays on Software Engineering", Addison-Wesley. ISBN 0-201-00650-2.  (Reprinted with corrections, January 1982)
Binder, Robert V. (1985). "Application Debugging: An MVS Abend Handbook for Cobol, Assembly, PL/I, and Fortran Programmers ", Prentice-Hall. ISBN 0-13-039348-7.
Pugh, Emerson W.; Johnson, Lyle R.; Palmer, John H. (1991). IBM's 360 and Early 370 Systems, Cambridge : MIT Press. (pp. 291–345)


=== Articles ===
"Building the System/360 Mainframe Nearly Destroyed IBM". IEEE Spectrum. 2019-04-05. Retrieved 2022-05-02.


== External links ==
Operating System/360 1965–1972
MVS... Long History on archive.org

---
_本地归档时间: 2026-09-05 | 来源: https://en.wikipedia.org/wiki/OS/360_
