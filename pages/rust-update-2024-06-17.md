---
title: Rust social status update 2024.06
timestamp: 2024-06-17T20:30:01
author: szabgab
published: true
description:
tags:
    - Rust
todo:
    - TODO
---

This post will be ready soon... in time before This week in Rust is published


June 17 is a rather arbitrary date to write this report, but the [Rust user groups](/user-groups) page was updated exactly 3 months ago. As I updated the numbers
there today I wanted to submit it to [This week in Rust](https://this-week-in-rust.org/), but, rather understandably, they don't like to include the same URL twice.
They even have a CI job failing if the same URL is submitted twice.

So I figured instead of submitting that page I could update some of the other data pages I collected and I could write a report about all the changes. That will be probably more
interesting and it will also have a different URL. Then if I remember to update it later, I could repeate and maybe expand this report a few months from now.

So this is the report.

## Rust User Groups

There are mostly Meetup groups, though not all of them.

| date       | no. groups | members | change |
| ---------- | ---------- | ------- | ------ |
| 2023.12.11 |  99        | 59,629  |        |
| 2024.03.17 | 113        | 65,200  |  5,571 |
| 2024.06.17 | 116        | 68,126  |  2,926 |

This is 4.4% growth in 3 month or roughly 20% annually. I think that is a very impressiv growth. Using the git commits one could check the changes in the specific groups,
but it seems that groups that organize events get many new members while others that don't organize event may also loose a few.

The two largest Rust groups are the [Rust London User Group](https://www.meetup.com/rust-london-user-group/) (3,223) and [Rust NYC](https://www.meetup.com/rust-nyc/) (3,041).
These are also the only groups with more than 3000 members.

The 3rd would be the [Code-Mavens](https://www.meetup.com/code-mavens/) group (2,949) that I use to organize virtual events, however it is not a purely Rust group so
I did not include it in the list.

The smallest group is [Rust in Israel](https://www.meetup.com/rust-in-israel/) (18 members) that I created on this Sunday. It will grow.

For full details, see of [Rust User Groups](/user-groups).

## Rust on LinkedIn

[Rust Programming Language](https://www.linkedin.com/groups/4973032/) - 23,261 members 24,139

[Rust on LinkedIn](/rust-on-linkedin)

## Rust on Facebook

[Rust on Facebook](/rust-on-facebook)


## Rust on Reddit

Subredit members

| subredit                                                  | 2024.04.08 | 2024.06.19 | change |
| --------------------------------------------------------- | ---------- | ---------- | ------ |
| [/r/rust](https://www.reddit.com/r/rust/)                 | 286K       | 297K       | +3.8%  |
| [/r/learnrust](https://www.reddit.com/r/learnrust/)       |  25K       |  27K       |   +8%  |
| [/r/rust_gamedev](https://www.reddit.com/r/rust_gamedev/) |  38K       |  38K       |        |

The scary part is that on the Gamedev both the number of subscribers as the number of people currently online were the same as 2 months ago.

See [Rust on Reddit](/rust-on-reddit)

## Rust on Telegram

There might be other groups, but I am not aware of them and I've just created the Rust Maven telegram group so let me put it here:

Number of members.

|  group                                        | 2024.06.19 |
| --------------------------------------------- | ---------- |
| [Rust Maven](https://t.me/+5P2gCQIWFaBkYmI0)  |  0         |

And you are invited to join!

## WhatsApp

Are there any Rust groups on WhatsApp that you would like me to include in my report?

## The popularity of Rust

The [Stack Overflow  survey 2023](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages) indicates that Rust
is the most admired language by a largegap at 84.66%  (the next is Elixir at 73.13%). That is a lot of people love Rust. On the other hand it is only 6th most desired language
with 30.56%. Ahead of it are SQL and HTML/CSS that are, well, different. Also TypeScript, Python, and JavaScript. As I understand this indicates how many job openings are.

In a nutshell, this seem to indicate that many people will want to use Rust, but the number of open positions are relatively low.

However, being on the 6th place is also extremely good.

[PYPL -  PopularitY of Programming Language](https://pypl.github.io/PYPL.html)

Indicates that Rust is number 10 with 2.5% share and +0.4% 1-year trend.

Python is number 1 with 29.06% share and 1.4% 1-year trend.

C/C++ (budled together) is number 5 with 6.4%  with -0.0% 1-year trend. (emphasize of being negative 0 :-).


[TIOBE](https://www.tiobe.com/tiobe-index/)

Rust is number 17 (up from number 20 a year ago) with 1.17% ratings and +0.26% change.

Python is number 1 with 15.39% ratings and +2.93% change.

C++ is number 2 with 10.03% ratings and -1.33% change.

C is number 3 with 9.23% ratings and -3.14 change.

In their description they seem to indicated that in the last month C++ overtook C for the first time, but if I understand correctly the "change" field, we should say that C fell more than C++.


