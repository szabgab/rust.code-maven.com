---
title: Rust vs. C++
timestamp: 2024-04-07T15:30:01
author: szabgab
published: true
description: There are many aspects one needs to consider when picking a programming language. Let me try to compare C++ and Rust.
tags:
    - Rust
    - C++
    - CPP
---

Teams using C++ often ask themselves, should we switch to Rust or should we keep using C++.
Sometimes the also ask me, but I've never written C++ so I would not dare to offer my own thoughts about that.

Instead, on this page I am trying to collect some points one could take in account when comparing the two languages.

There are many articles comparing their technical merits of the two languages,
I don't think repeating them here would benefit anyone, though I should link to them. So I look at some other metrics. Some might be relevant to you, some not.


## Speed

In general it is said that the run-time speed of the two languages is the same.

## Speed of development

This is extremely hard to measure as the level of experience of the programmers has a huge impact on this.
The anecdotal evidence is that it might take longer to get the Rust code compile, but then there are a lot less bugs which means the over development and QA time
will be shorter and there will be less bugs found after deployment.

## Maturity of the language and the ecosystem

Clearly C++ has a huge advantage here. It has been around since 1985 (39 years) while Rust only appeared in 2015 (9 years ago).
Rust progresses much faster than C++ for several reasons. It is still in its "early years". It does not have such a huge baggage that
requires backward compability and it has a well-designed pas to changes in the language that maintains backward compability while allows the
progress of the language.


## Number of available programmers

On one hand there are probably a lot more C++ developers with a lot more experience in C++ than Rust developers.
On the other hand there might be more people who are enthusiastic about Rust and would like to work in Rust. In [The State of Developer Ecosystem 2023](https://www.jetbrains.com/lp/devecosystem-2023/)
conducted by JetBrains 10% of the developers are likely to adopt Rust, the highest among all the languages, while this number is only 4% for C++ and 2% for C.
(On the other hand only 10% used Rust vs. 25% who used C++ and 18% who used C.)

## Size of the "community"

Meaning people who are attending Meetups, conferences. There is a separate entry for questions on StackOverflow, and open source developers.

We found [113 Rust user-groups](/user-groups) with a combined 65,200 members. (The same person can be in multiple groups so the actual number is lower. Meetup lists [248 Rust groups](https://www.meetup.com/topics/rust/)
with a total of 109,982 member, but as far as I can tell for many of those Rust is not the main subject.

I don't have a collection of C++ or C user-groups but Meetup lists [23 C++ groups](https://www.meetup.com/topics/cpp/) with 12,664 members.

There a number of Rust conferences every year. I don't know much about C++ conferences. Are there?

## StackOverflow

* There are [41,065 questions tagged as Rust](https://stackoverflow.com/questions/tagged/rust).
* There are [806,882 questions tagged as C++](https://stackoverflow.com/questions/tagged/c%2b%2b).
* There are [403,979 questions tagged as C](https://stackoverflow.com/questions/tagged/c).

Of course one has to keep in mind that Stack Overflow was established in 2008 and Rust only appeared in 2015 and the number of developers started to grow only then. I think
It would be much more useful to compare the number of questions in the last year.

The [2023 StackOverflow survey](https://survey.stackoverflow.co/2023/) showed

* 22.42% use C++
* 19.34% use C
* 13.05% use Rust

## Reddit

* [r/cpp](https://www.reddit.com/r/cpp/) has 277K  members (138 online)
* [r/rust](https://www.reddit.com/r/rust/) has 286K  members (364 online)

Obviously the "number online" will fluctuate a lot, but I checked them at the same time.

There are also other subreddits covering the topics.


## LinkedIn

There are several [groups on LinkedIn dedicated to C++ developers](https://www.linkedin.com/search/results/groups/?keywords=c%2B%2B&origin=SWITCH_SEARCH_VERTICAL&sid=%3A%40*). These seem to be the biggest ones:

* [C++ professionals group](https://www.linkedin.com/groups/86782/) with 57,857 members
* [C++ Developers Group](https://www.linkedin.com/groups/86998/) with 37,669 members.
* [C++ Professionals](https://www.linkedin.com/groups/100895/) with 30,209 members.

Rust has less groups with less members:

* [Rust Programming Language](https://www.linkedin.com/groups/4973032/) with 23,229 members.
* [Rust Developer Community (Rust Lang)](https://www.linkedin.com/groups/12537155/) with 3,651 members.

## GitHub

* [Rust](https://github.com/topics/rust) has 54,558 public repositories
* [C++](https://github.com/topics/cpp) has 67,312

One has to be careful with these numbers as these are based on manual tagging of repositories by the project owner.

## 3rd party (Open Source) libraries

* Rust has 140,000 [Crates](https://crates.io/).
* [vcpkg](https://vcpkg.io/en/packages.html), the largest 3rd party registry for C++ has 2,400 packages. There are of course many more packages available around the Internet. One of the difficulties with C++ is that there is no standardized central registry and no easy way to manage the 3rd-party libraries.

Some interesting information related to this topic

* A list of open-source C++ libraries](https://en.cppreference.com/w/cpp/links/libs).
* [C++ Ecosystem in 2021: 1 in 5 C++ developers are using C++20 and a third of us are not writing any unit tests at all, and other facts](https://blog.jetbrains.com/clion/2021/07/cpp-ecosystem-in-2021/).

## Security

Rust has out-of-the-box enforcement of a lot of things for which C++ requires extra tools. I hear many security issues can be detected by the tools available for C++,
but those cost money and have a learning curve. The Rust compiler prevents most of those issues entering the code-base in the first place.

## Corporations

* As I understand Microsoft has been switching some projects from C++ to Rust.
* Rust teams at Google are twics as productive as teams using C++. See video: [Lars Bergstrom - Beyond Safety and Speed: How Rust Fuels Team Productivity](https://www.youtube.com/watch?v=QrrH2lcl9ew).


* Linus Torvalds allows only two languages in the Linux kernel. C, the language that was originally used to write it and recently started to accept Rust as well.

## Discussion

There was a long [thread on Reddit](https://www.reddit.com/r/rust/comments/1by3lle/rust_vs_c/) about this. Some of the comments are quite useful.

