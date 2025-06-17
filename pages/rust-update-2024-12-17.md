---
title: Rust social status update 2024.12
timestamp: 2024-12-17T16:30:01
author: szabgab
published: true
description: How many member do various Rust social groups have?
tags:
    - popularity
---

While the technical capabilities of a programming language are very important, the social acceptance of the language also has a huge
impact on the ecosystem around the language.

It is also a good (or at least reasonable) measurement of the popularity of the language.

The [Rust user groups](/user-groups) page was updated exactly 6 months ago so today, December 17 is the day I share the new report.

The [report on 2024.06.17](/rust-update-2024-06-17).

## Rust User Groups

There are mostly Meetup groups in [the list](/user-groups), though some use different platforms. In some cases the number of members cannot be determined.

Overall:

| date       | no. groups | members | change |
| ---------- | ---------- | ------- | ------ |
| 2023.12.11 |  99        | 59,629  |        |
| 2024.03.17 | 113        | 65,200  |  5,571 |
| 2024.06.17 | 116        | 68,126  |  2,926 |
| 2024.12.17 | 132        | 77,688  |  9,562 |

In the last 6 months the number of members grew by 14%. Thats over 30% annually.

In the previous 3 month the growth was 4.4% or roughly 20% annually.

The three largest Rust groups are the [Rust London User Group](https://www.meetup.com/rust-london-user-group/) (3,357), [Rust NYC](https://www.meetup.com/rust-nyc/) (3,192), and
[Rust Berlin](https://www.meetup.com/rust-berlin/) (3,123).

These are also the only groups with more than 3,000 members.

My [**Code-Mavens**](https://www.meetup.com/code-mavens/) group has 3,213. I use it to organize virtual events in English about Rust, Python, and Perl.
I did not include it in the list.

Since the previous report the **Rust in Israel** group I created gained 200 members, but then a few weeks ago I inherited the [Rust Tel Aviv](https://www.meetup.com/rust-tlv/) group with 950 and asked all the people to move over. It now has 1,011 members.

For full details, see of [Rust User Groups](/user-groups).

I updated the figures in several commit that will allow someone to check the changes more easily. Here are the commits.

| action                                 | no. groups | members | commit |
| -------------------------------------- | ---------- | ------- | ------ |
| Before                                 | 116        | 68,126  |        |
| Update members removed closed groups   | 107        | 71,116  | [commit](https://github.com/szabgab/rust.code-maven.com/commit/92815e1ac5f55a651acdde2a0ad191693976048f) |
| Add group I did not know about earlier | 132        | 77,688  | [commit](https://github.com/szabgab/rust.code-maven.com/commit/32a557b02ff2d9b36f0b2ed3c4cc5536297479b5) |
| Sort groups                            | 132        | 77,688  | [commit](https://github.com/szabgab/rust.code-maven.com/commit/4ef74d46ff015866226eb4cff462ec06b0b43bfb) |



## Rust on LinkedIn

| group                                                                                                        |  2024.04.08 | 2024.06.17 | change (2 months) | 2024.12.17 | change (6 month) |
| ------------------------------------------------------------------------------------------------------------ | ----------- | ---------- | ----------------- | ---------- | ---------------- |
| [Rust Programming Language](https://www.linkedin.com/groups/4973032/)                                        | 23,261      | 24,139     |  3.7 %            | 26,186     |   8.4%           |
| [Rust Developer Community (Rust Lang)](https://www.linkedin.com/groups/12537155/)                            |  3,668      |  4,048     | 10.3 %            |  5,068     |  25.1%           |
| [Rust Developers](https://www.linkedin.com/groups/6931877/)                                                  |  1,311      |  1,365     |  4.1 %            |  1,455     |   6.5%           |
| [Rust (Programming Language)](https://www.linkedin.com/groups/12566531/)                                     |  1,308      |  1,489     | 13.8 %            |  2,025     |  35.9%           |

[Rust on LinkedIn](/rust-on-linkedin)

## Rust on Facebook

| Group                                                                                | 2024.04.08 | 2024.07.19 | change | 2024.12.17 |
| ------------------------------------------------------------------------------------ | ---------- | ---------- | ------ | ---------- |
| [Rust Developers](https://www.facebook.com/groups/1412062792318164/) (inactive)      | 8.8K       | 9.0K       | 2.27%  |  9.5K      |
| [**Rust Programming Language**](https://www.facebook.com/groups/872919370237098/)    | 3.0K       | 3.4K       | 13%    |  4.1K      |
| [Rust-lang.tw](https://www.facebook.com/groups/rust.tw/)                             | 2.5K       | 2.6K       | 4%     |  2.7K      |
| [Programming Rust](https://www.facebook.com/groups/programming.rust/)                | 1.9K       | 2.0K       | 5%     |  2.3K      |
| [Rust Developers Vietnam](https://www.facebook.com/groups/rustdevelopersvietnam/)    | 1.8K       | 2.2K       | 22%    |  2.7K      |

The first group listed is inactive, but still people become members.

Full report on [Rust on Facebook](/rust-on-facebook)

## Rust on Reddit

Subredit members

| subredit                                                  | 2024.04.08 | 2024.06.19 | change (2 month) | 2024.12.16 | change (6 month) |
| --------------------------------------------------------- | ---------- | ---------- | ---------------- | ---------- | ---------------- |
| [/r/rust](https://www.reddit.com/r/rust/)                 | 286K       | 297K       |  3.8%            |  325K      |  9.4%            |
| [/r/learnrust](https://www.reddit.com/r/learnrust/)       |  25K       |  27K       |    8%            |   32K      | 18.5%            |
| [/r/rust_gamedev](https://www.reddit.com/r/rust_gamedev/) |  38K       |  38K       |                  |   40K      |  5.2%            |

The 3.8% increase in 2 months is roughly 25% annually. Very nice.

The 8% increase in 2 months is, what, 55% a year? I did not compute it, but also there might be some extreme rounding error as we only have the numbers in thousands so the real change is somewhere between 1,000 and 3,000 members.

The scary part is that on the Gamedev both the number of subscribers as the number of people currently online were the same as 2 months ago.

See [Rust on Reddit](/rust-on-reddit)

## X Twitter

Followers

| account                                                | 2024.01.06 | 2024.06.19 | 2024.12.16 |
| ------------------------------------------------------ | ---------- | ---------- | ---------- |
| [Rustlang](https://twitter.com/rustlang)               | 136.9K     | 143.9K     | 148.9K |
| [Mara Bos](https://twitter.com/m_ou_se)                | 41.3K      |  43.3K     |  44.7K |
| [rust_foundation](https://twitter.com/rust_foundation) | 35.6K      |  37.7K     |  39.0K |
| [RustTrending](https://twitter.com/RustTrending)       | 29.5K      |  32.4K     |  34.8K |
| [ThisWeekInRust](https://twitter.com/ThisWeekInRust)   | 31.9K      |  32.3K     |  32.3K |
| [RustConf](https://twitter.com/rustconf)               | 15.3K      |  16.4K     |  17.4K |
| [rust_analyzer](https://twitter.com/rust_analyzer)     | 13.5K      |  13.5K     |  13.1K |
| [rustembedded](https://twitter.com/rustembedded)       | 11.2K      |  11.9K     |  12.3K |
| [tokio_rs](https://twitter.com/tokio_rs)               | 11.6K      |  11.8K     |  11.6K |
| [SurrealDB](https://twitter.com/SurrealDB)             | 7,498      |  8,345     |  8,524 |
| [RustFest](https://twitter.com/RustFest)               | 7,825      |  7,742     |  7,505 |
| [rustjobs_dev](https://twitter.com/rustjobs_dev)       | 4,499      |  5,568     |  6,035 |
| [crates.io status](https://twitter.com/cratesiostatus) | 5.039      |  5,543     |  5,665 |
| [RustSecurity](https://twitter.com/RustSecurity)       | 3,964      |  4,611     |  3,885 |
| [rustlab_conf](https://twitter.com/rustlab_conf)       | 3,816      |  4,163     |  4,674 |
| [AstraKernel](https://twitter.com/AstraKernel)         | 1,925      |  3,611     |  4,436 |
| [Orhun Parmaksız](https://twitter.com/orhundev)        |            |  3,336     |  4,200 |
| [RustDiscussions](https://twitter.com/RustDiscussions) | 2,549      |  3,285     |  3,808 |
| [RustNationUK](https://twitter.com/RustNationUK)       | 2,157      |  2,738     |  3,215 |
| [RustLondon_](https://twitter.com/RustLondon_)         | 2,522      |  2,699     |  2,838 |
| [euro_rust](https://twitter.com/euro_rust)             | 2,201      |  2,503     |  2,915 |
| [letsgetrusty](https://twitter.com/letsgetrusty)       | 1,418      |  2,387     |  3,029 |
| [rustoftheday](https://twitter.com/rustoftheday)       |   482      |  1,513     |  1,806 |
| [ratatui_rs](https://twitter.com/ratatui_rs)           |    79      |    839     |  1,586 |
| [**RustMaven**](https://twitter.com/RustMaven)         |     1      |     48     |     31 |

Orhun Parmaksız is the maintainer of Ratatui.rs


## Instagram

2023.12.07

| account                                                                           | posts | followers | following |
| --------------------------------------------------------------------------------- | ----- | --------- | --------- |
| [rust_programming_language](https://www.instagram.com/rust_programming_language/) | 12    | 2,374     |  16       |
| [rustlanguage](https://www.instagram.com/rustlanguage/)                           |  7    | 1,223     |   0       |
| [rustacean.dev](https://www.instagram.com/rustacean.dev/)                         | 14    |   576     |   9       |
| [rust_language_](https://www.instagram.com/rust_language_/)                       | 19    |   283     | 434       |
| [rust_lang_](https://www.instagram.com/rust_lang_/)                               |  1    |    75     |   6       |
| [Rust Maven on Instagram](https://www.instagram.com/rust_maven/)                  |  0    |     3     |   0       |

2024.06.19

| account                                                                           | posts | followers | following |
| --------------------------------------------------------------------------------- | ----- | --------- | --------- |
| [rust_programming_language](https://www.instagram.com/rust_programming_language/) | 21    | 2,709     |   5       |
| [rustlanguage](https://www.instagram.com/rustlanguage/)                           |  7    | 1,245     |   1       |
| [rustacean.dev](https://www.instagram.com/rustacean.dev/)                         | 14    |   624     |   9       |
| [rust_language_](https://www.instagram.com/rust_language_/)                       | 19    |   330     | 434       |
| [rust_lang_](https://www.instagram.com/rust_lang_/)                               |  1    |    93     |   6       |
| [Rust Maven on Instagram](https://www.instagram.com/rust_maven/)                  | 34    |    64     |   6       |

2024.12.16

| account                                                                           | posts | followers | following |
| --------------------------------------------------------------------------------- | ----- | --------- | --------- |
| [rust_programming_language](https://www.instagram.com/rust_programming_language/) | 24    | 4,203     |   3       |
| [rustlanguage](https://www.instagram.com/rustlanguage/)                           |  7    | 1,273     |   1       |
| [rustacean.dev](https://www.instagram.com/rustacean.dev/)                         | 14    |   652     |   9       |
| [rust_language_](https://www.instagram.com/rust_language_/)                       | 19    |   335     | 432       |
| [rust_lang_](https://www.instagram.com/rust_lang_/)                               |  1    |   100     |   6       |
| [Rust Maven on Instagram](https://www.instagram.com/rust_maven/)                  | 36    |    59     |   6       |



## Rust on Telegram

There might be other groups, but I am not aware of them and I've just created the Rust Maven telegram group so let me put it here:

Number of members.

|  group                                        | 2024.06.19 | 2024.12.16 |
| --------------------------------------------- | ---------- | ---------- |
| [Rust Maven](https://t.me/+5P2gCQIWFaBkYmI0)  |  0         |  16        |

And you are invited to join!

If you know of other groups, let me know so I can list them here.

## WhatsApp

Are there any Rust groups on WhatsApp that you would like me to include in my report?

## The popularity of Rust

The [Stack Overflow  survey 2024](https://survey.stackoverflow.co/2024/) indicates that Rust is the most admired language by a large gap at 82.2%  (the next is Elixir at 76.8%). That is a lot of people love Rust. However this gap was much bigger last year.

On the other hand it is only 6th most desired language with 28.7%. Ahead of it are SQL and HTML/CSS that are, well, different. Also TypeScript, Python, and JavaScript. As I understand this indicates how many job openings are.

In a nutshell, this seem to indicate that many people will want to use Rust, but the number of open positions are relatively low.

However, being on the 6th place is also extremely good.

[PYPL -  PopularitY of Programming Language](https://pypl.github.io/PYPL.html)

Indicates that Rust is number 9 with 2.66% share and +0.5% 1-year trend. (up from number 10 six months ago)

Python is number 1 with 29.71% share and 1.5% 1-year trend.

C/C++ (bundled together) is number 4 with 7.06%  with 0.3% 1-year trend. (up from number 5 six months ago)


[TIOBE](https://www.tiobe.com/tiobe-index/)

Rust is number 14 (up from number 18 a year ago) with 1.29% ratings and +0.48% change.

Python is number 1 with 23.84% ratings and +9.98% change.

C++ is number 2 with 10.82% ratings and 0.81% change.

Java is number 3 with 9.72% ratings and 1.73% change.

C is number 4 with 9.10% ratings and -2.34% change.



