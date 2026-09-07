---
title: "经典论文导读: Robotics"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "Robotics"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
  - "Papers We Love"
  - "计算机经典论文"
  - "Robotics"
---

〔本篇为 Papers We Love 经典学术文献库关于“Robotics”领域收录的传世奠基论文全景导读与本地文献档案。〕

# 一、 领域学术导读与背景

Robotics
====

[Adaptive Road Following using Self-Supervised Learning and Reverse Optical Flow](http://www.roboticsproceedings.org/rss01/p36.pdf)

[DP-SLAM: Fast, Robust Simultaneous Localization and Mapping Without Predetermined Landmarks](http://people.ee.duke.edu/~lcarin/Lihan4.21.06a.pdf)

[The Dynamic Window Approach to Collision Avoidance](https://www.ri.cmu.edu/pub_files/pub1/fox_dieter_1997_1/fox_dieter_1997_1.pdf)

[Online Trajectory Generation: Basic Concepts for Instantaneous Reactions to Unforeseen Events](http://ieeexplore.ieee.org/xpl/freeabs_all.jsp?arnumber=5350749)

[Probablistic Roadmaps for Path Planning in High-Dimensional Configuration Spaces](https://www.cs.cmu.edu/~./motionplanning/papers/sbp_papers/PRM/prmbasic_01.pdf)

[Rapidly-Exploring Random Trees: A New Tool for Path Planning](http://msl.cs.uiuc.edu/~lavalle/papers/Lav98c.pdf)

[RGB-D Mapping: Using Depth Cameras for Dense 3D Modeling of Indoor Environments](https://rse-lab.cs.washington.edu/postscripts/3d-mapping-iser-10-final.pdf)



Reasoning for the new papers:

The dynamic window approach to collision avoidance is an influential
paper for mobile robots. The method is based on a robot's dynamics
rather than higher-level representations of a robot and/or obstacles in
an environment.

The PRM and RRT algorithms are two seminal papers in robot motion
planning. The problem of motion planning scales exponentially with the
degrees of freedom a robot has and the degrees of freedom the obstacles
in an environment have. Thus, planning with high degrees of freedom leads to many problems
such as incompleteness and extremely slow speed. The PRM method was the first to
propose a sampling-based strategy to deal with motion planning and
created a practical method for offline planning of robot manipulators.
The RRT method modified PRM by using a tree structure rather than a
graph so that non-holonomic and other constraints could be considered
when planning.

The Instantaneous Trajectory Generation method is relatively new, but
very important. It allows for extremely fast trajectory generation for
robots of high degrees of freedom (motion states generated within 1
millisecond). It has been used to implement robot sword fighting and
other activities that require fast reaction-based planning. The author
started a business based simply on the work and has shown the
algorithm's success in many robot applications.

# 二、 核心学术论文与本地文献原件



---
_本地归档路径: `corpus/papers_we_love/raw/robotics/`_
