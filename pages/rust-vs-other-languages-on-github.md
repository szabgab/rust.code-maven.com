---
title: Rust vs. other languages on GitHub
timestamp: 2024-03-11T10:30:01
author: szabgab
published: true
description: The number of projects tagged on GitHub by a language can have some indication on the popularity, but it also has some serious issues.
tags:
    - GitHub
todo:
    - automate fetching this data
    - get the number of repos by language as recognized by the language analyzer of GitHub
---

One of the aspects we take in account when picking a language for a project is the popularity of the language. It has some indication regarding the availability of programmers and the availability of 3rd party packages. I looked at the number on GitHub topics for some clue.


| Language | repositories | followers |
| -------- | ------------ | --------- |
| [Rust](https://github.com/topics/rust)             |  54,558 |  38,800 |
| [Ada](https://github.com/topics/ada)               |     706 |      72 |
| [C](https://github.com/topics/c)                   |  61,456 | 110,000 |
| [C++](https://github.com/topics/cpp)               |  67,312 | 124,000 |
| [Go](https://github.com/topics/golang)             |  58,155 |  61,000 |
| [Golang](https://github.com/topics/golang)         |  91,210 |  61,000 |
| [Java](https://github.com/topics/java)             | 232,184 | 153,000 |
| [JavaScript](https://github.com/topics/javascript) | 511,622 | 170,000 |
| [Kotlin](https://github.com/topics/kotlin)         |  49,809 |  33,500 |
| [Perl](https://github.com/topics/perl)             |   4,450 |   8,700 |
| [PHP](https://github.com/topics/php)               | 115,931 |  53,800 |
| [Python](https://github.com/topics/python)         | 428,293 | 285,000 |
| [Ruby](https://github.com/topics/ruby)             |  34,901 |  21,100 |

The number of projects tagged on GitHub by a language can have some indication on the popularity, but it also has some serious issues.

* There are two popular tags used for Go/Golang, so I included both.

* Perl: [CPAN rocks](https://cpan.rocks/) indicate that there are about 40,000 distributions (libraries, crates, if you wish), 9,276  that is 23.3% indicate that they have GitHub repository.
Despite this we only see 4.450 tagged as Perl.

* Python: [PyDigger](https://pydigger.com/stats) indicates that roughtly 63.13% of the packages indicate that they have GitHub repository. [PyPi](https://pypi.org/) reports a total of 522,170. That would mean about 330,000 Python projects on GitHub while 428,000 are tagged as Python.

* Rust: [Rust digger](https://rust-digger.code-maven.com/) indicates there are a total of 132,501 crates. Out that 100,013 (75.48%) indicate their repository on GitHub. However, on GitHub only 54,558 actually have been tagged as Rust.

* C++: [vcpkg](https://vcpkg.io/en/packages.html) the largest registtry of C++ libraries has only 2,396 libraries while there are 67,312 tagged as C++ on GitHub.
