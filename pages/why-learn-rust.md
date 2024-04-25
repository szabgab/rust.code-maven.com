---
title: Why learn Rust?
timestamp: 2024-03-08T09:30:01
author: szabgab
published: false
description:
tags:
    - popularity
    - TIOBE
todo:
    - TODO
---

As I tell people that I have been investing most of my time in the Rust programming language in the last year, people ask me: Why?

So I am trying to collect my thoughts why I think it is valuable for me to learn Rust and why would it be valuable to you too?


## Technical aspects


Rust is primarily in the space of C/C++, but it has a lot of aspects that make it similar to the dynamic languages I've used in the last 30 or so years. (Perl, Python, Ruby, Javascript, PHP).

It is as fast as C/C++ and as memory efficient as those languages, but it does not have the memory-management issues those languages have.


You have to be a lot more aware of anything that can go wrong in your application. Something that's too easy in the dynamic langages to skip. Thus it is harder to write in Rust than in the dynamic languages, but the result is much less error-prone.


## Economic aspects

* Reduced TCO - total Cost of Ownership. There are plenty of reports indicating that fixing a bug in production is way more expensive than fixing it during QA which is more expensive than fixing it during development. With the strict rules the Rust compiler employes and the excellent and built-in linter, Rust-based projects will have a lot less bugs in production. This probably increases the initial development cost, but it will reduce the total cost of the project. How soon will you see these benefits depends on the type of project. Probably you will already see it before your reach production as you can shorten the QA cycles.


* Save time in QA and test writing: Dynamic languages (Python, JavaScript, etc.) are a lot more "forgiving" which means one has to invest more time in writing automated tests or has to spend more time in QA.

* Avoid unexpected cost due to safety issues. This is a bit tricky. As long as you don't have an issue you don't see the cost, but once someone can exploit your software or once there is a bug, the event can hava catastrphic consequences. Think about a bug in the software of a spaceship, or in a financial software that either miscalculates or can be breached.

* Free reusable building blocks, 3rd-pary libraries. C++ has some 2300 3rd party libraries, Rust has over 140,000 in [Crates](https://crates.io/).

* Easily write to multiple platformst - Out of the box cross-compilation.


## Social and business validation

Great technology and the prospect of economic benefits are great, but most of us would not want to be the sole users of some technology. Knowing who else uses it can help.
Especially if we personally know that other person or if it is a well-known organization.






## Community related aspects


Rust was first published in 2015, so its public life isn't too long, but its user base seems to have a steady growth.



* [TIOBE](https://www.tiobe.com/tiobe-index/) is far from being a reliable indicator, but it can capture some trends. In February 2024 It puts Rust on the **18th** place.

* [PyPL](https://pypl.github.io/PYPL.html) puts t in the **11th** place.


* [Octoverse 2022](https://octoverse.github.com/2022/top-programming-languages), the annual report of GitHub, put Rust as the 2nd fastest growing programming language after HCL with more than 50% year-to-year growth.


* Rust technically is closer to C/C++, but community-wise closer to Python/JavaScript/etc.

The primary target market of Rust is the where C and C++ are used traditionally.

Neither C not C++ have a lot of community: Very few Meetups and conferences. On the other hand rust has some [100 user groups]


