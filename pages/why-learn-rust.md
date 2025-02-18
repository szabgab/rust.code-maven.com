---
title: Why learn Rust?
timestamp: 2025-02-18T09:30:01
author: szabgab
published: true
description: Some reasons why it is a good idea to learn Rust and start considering moving to Rust.
tags:
    - popularity
    - TIOBE
todo:
    - update
---

As I tell people that I have been investing most of my time in the Rust programming language in the last year, people ask me: Why?

So I am trying to collect my thoughts why I think it is valuable for me to learn Rust and why would it be valuable to you too?

## Technical aspects

Rust is primarily in the space of C/C++, but it has a lot of aspects that make it similar to the dynamic languages I've used in the last 30 or so years. (Perl, Python, Ruby, JavaScript, PHP).

It is as fast as C/C++ and as memory efficient as those languages, but it does not have the memory-management issues C/C++ have. It handles freeing memory with the ownership/borrowing system.

You have to be a lot more aware of anything that can go wrong in your application. Something that's too easy in the dynamic languages to skip. Thus it is harder to write in Rust than in the dynamic languages, but the result is much less error-prone.


## Economic aspects

* Reduced TCO - total Cost of Ownership. There are plenty of reports indicating that fixing a bug in production is way more expensive than fixing it during QA which is more expensive than fixing it during development. With the strict rules the Rust compiler employes and the excellent and built-in linter, Rust-based projects will have a lot less bugs in production. This probably increases the initial development cost, but it will reduce the total cost of the project. How soon will you see these benefits depends on the type of project. Probably you will already see it before you reach production as you can shorten the QA cycles.


* Save time in QA and test writing: Dynamic languages (Python, JavaScript, etc.) are a lot more "forgiving" which means one has to invest more time in writing automated tests or has to spend more time in QA.

* Avoid unexpected cost due to safety issues. This is a bit tricky. As long as you don't have an issue you don't see the cost, but once someone can exploit your software or once there is a bug, the event can have catastrophic consequences. Think about a bug in the software of a spaceship, or in a financial software that either miscalculates or can be breached.

* Free reusable building blocks, 3rd-party libraries. C++ has some 2300 3rd party libraries, Rust has over 140,000 in [Crates](https://crates.io/).

* Easily write to multiple platforms: Rust has out of the box cross-compilation.


## Social and business validation

Great technology and the prospect of economic benefits are a must, but most of us would not want to be the sole users of some technology. Knowing who else uses it can help.
Especially if we personally know that other person or if it is a well-known organization. There are many [companies using Rust](/companies).

## Community related aspects


Rust was first published in 2015, so its public life isn't too long, but its user base seems to have a steady growth.


* [TIOBE](https://www.tiobe.com/tiobe-index/) is far from being a reliable indicator, but it can capture some trends. In February 2024 It puts Rust on the **18th** place. In February 2025 Rust was in the **13th** place.

* [PyPL](https://pypl.github.io/PYPL.html) put it in the **11th** place in February 2024 and **8th** place in February 2025.


* [Octoverse 2022](https://octoverse.github.com/2022/top-programming-languages), the annual report of GitHub, put Rust as the 2nd fastest growing programming language after HCL with more than 50% year-to-year growth.
* [Octoverse 2024](https://github.blog/news-insights/octoverse/octoverse-2024/) tells us that Rust continues to gain popularity.

* Rust technically is closer to C/C++, but community-wise closer to Python/JavaScript/etc.

Originally the primary target areas of Rust was where C and C++ were used traditionally, but there are plenty of cases where some code written in Python or in other dynamic but slow languages were rewritten in rust.

Neither C not C++ have a lot of community: Very few Meetups and conferences. On the other hand Rust has [more that 100 user groups](/user-groups) with almost 80,000 members world wide.
It is still much smaller than Python, but it is growing rapidly.



