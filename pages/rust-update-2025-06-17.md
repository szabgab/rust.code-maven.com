---
title: Rust social status update 2025.06
timestamp: 2025-06-17T20:30:01
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

* The [report on 2024.12.17](/rust-update-2024-12-17).
* The [report on 2024.06.17](/rust-update-2024-06-17).

## Rust User Groups

There are mostly Meetup groups in [the list](/user-groups), though some use different platforms. In some cases the number of members cannot be determined.

Overall:

| date       | no. groups | members | change |
| ---------- | ---------- | ------- | ------ |
| 2023.12.11 |  99        | 59,629  |        |
| 2024.03.17 | 113        | 65,200  |  5,571 |
| 2024.06.17 | 116        | 68,126  |  2,926 |
| 2024.12.17 | 132        | 77,688  |  9,562 |
| 2025.06.17 | 120        | 79,588  |  1,900 |

In the last 6 months several groups closed down. Probably due to the price-hike of Meetup (the new price is 3 times of the old one). Some might use another platform, but I don't know where.
If you do, please let me know!

These are the groups that were closed:
* Rust Chennai in Chennai, India. 1,204
* Rust AKL in Auckland, New Zealand. 672
* Finland Rust-lang Group in Helsinki, Finland. 602
* Rust-Saar in Saarbrücken, Germany. 425
* Rust Chinese Community in Beijing, China. 163
* Ottawa Rust Language Meetup in Ottawa, ON, Canada. 154
* Hack-away with Rust in Espoo, Finland. 125
* RustSchool Rotterdam in Rotterdam, Netherlands. 104
* Rust Bordeaux in Bordeaux, France. 95
* Rust Triangle in Durham, NC, USA. 89
* Rust Bristol in Bristol, United Kingdom. 86
* Orange County Rust in Irvine, CA, USA. 72
* TOTAL: 3791

the number of members grew only by 2%, but 3,791 were in the closed groups. If we only count the groups that remained open then the growth is 7%. Less than in the previous 6 months, but still impressive.

* In the previous 6 months the number of members grew by 14%. Thats over 30% annually.
* In the 3 months before that the growth was 4.4% or roughly 20% annually.

The three largest Rust groups are:
* [Rust NYC](https://www.meetup.com/rust-nyc/) (3,523 members. It had 3,192)
* [Rust London User Group](https://www.meetup.com/rust-london-user-group/) (3,493 members. It had 3,357)
* [Rust Berlin](https://www.meetup.com/rust-berlin/) (4,435 members. It had 3,123).

These are also the only groups with more than 3,000 members.

My [**Code-Mavens**](https://www.meetup.com/code-mavens/) group has 3,841 members (up from 3,213). I use it to organize virtual events in English about Rust, Python, and Perl.
I did not include it in the list.

The [Rust Tel Aviv](https://www.meetup.com/rust-tlv/) group that I organize has 1,343 members (up from 1,011 6 months ago and up from 950 members from when I inherited it).

For full details, see of [Rust User Groups](/user-groups).

I updated the figures in several commit that will allow someone to check the changes more easily. Here are the commits.

| action                                 | no. groups | members | commit |
| -------------------------------------- | ---------- | ------- | ------ |
| Before                                 | 132        | 77,688  |        |
| Update members                         | 132        | 79,588  | [commit](https://github.com/szabgab/rust.code-maven.com/commit/5c0fe32e2af01d7b39c52ddcd1d106985596ed02) |
| Add group I did not know about earlier | 120        | 79,588  | [commit](https://github.com/szabgab/rust.code-maven.com/commit/3bfae81a1c9e4a46fbe77434bac5c28d752805e8) |
| Sort groups                            | 120        | 79,588  | [commit](https://github.com/szabgab/rust.code-maven.com/commit/d4258bba98b9aad3cff83d540799228f8e5b4279) |



## Rust on LinkedIn

| group                                                                                                        |  2024.04.08 | 2024.06.17 | change (2 months) | 2024.12.17 | change (6 month) | 2025.06.17 |
| ------------------------------------------------------------------------------------------------------------ | ----------- | ---------- | ----------------- | ---------- | ---------------- | ---------- |
| [Rust Programming Language](https://www.linkedin.com/groups/4973032/)                                        | 23,261      | 24,139     |  3.7 %            | 26,186     |   8.4%           |     27,539 |
| [Rust Developer Community (Rust Lang)](https://www.linkedin.com/groups/12537155/)                            |  3,668      |  4,048     | 10.3 %            |  5,068     |  25.1%           |      6,202 |
| [Rust Developers](https://www.linkedin.com/groups/6931877/)                                                  |  1,311      |  1,365     |  4.1 %            |  1,455     |   6.5%           |      1,538 |
| [Rust (Programming Language)](https://www.linkedin.com/groups/12566531/)                                     |  1,308      |  1,489     | 13.8 %            |  2,025     |  35.9%           |      2,509 |

[Rust on LinkedIn](/rust-on-linkedin)

## Rust on Facebook

| Group                                                                                | 2024.04.08 | 2024.07.19 | change | 2024.12.17 | 2025.06.17 |
| ------------------------------------------------------------------------------------ | ---------- | ---------- | ------ | ---------- | ---------- |
| [Rust Developers (inactive)](https://www.facebook.com/groups/1412062792318164/)      | 8.8K       | 9.0K       | 2.27%  |  9.5K      | 9.8 K |
| [**Rust Programming Language**](https://www.facebook.com/groups/872919370237098/)    | 3.0K       | 3.4K       | 13%    |  4.1K      | 4.7 K |
| [Rust-lang.tw](https://www.facebook.com/groups/rust.tw/)                             | 2.5K       | 2.6K       | 4%     |  2.7K      | 2.8 K |
| [Programming Rust](https://www.facebook.com/groups/programming.rust/)                | 1.9K       | 2.0K       | 5%     |  2.3K      | 2.5 K |
| [Rust Developers Vietnam](https://www.facebook.com/groups/rustdevelopersvietnam/)    | 1.8K       | 2.2K       | 22%    |  2.7K      | 2.9 K |

The first group listed is inactive, but still people become members.

Full report on [Rust on Facebook](/rust-on-facebook)

## Rust on Reddit

Subredit members

| subredit                                                  | 2024.04.08 | 2024.06.19 | change (2 month) | 2024.12.16 | change (6 month) | 2025.06.17 | change |
| --------------------------------------------------------- | ---------- | ---------- | ---------------- | ---------- | ---------------- | ---------- | ------ |
| [/r/rust](https://www.reddit.com/r/rust/)                 | 286K       | 297K       |  3.8%            |  325K      |  9.4%            |  351K      |  8%    |
| [/r/learnrust](https://www.reddit.com/r/learnrust/)       |  25K       |  27K       |    8%            |   32K      | 18.5%            |   38K      |  18.7% |
| [/r/rust_gamedev](https://www.reddit.com/r/rust_gamedev/) |  38K       |  38K       |                  |   40K      |  5.2%            |   42K      |  5%    |

All 3 groups grow nicely.

See [Rust on Reddit](/rust-on-reddit)

## X Twitter

Counting followers

| account                                          | 2024.01.06 | 2024.06.19 | 2024.12.16 |  2025.06.17 |
| ------------------------------------------------ | ---------- | ---------- | ---------- | ----------- |
| [Rustlang](https://x.com/rustlang)               | 136.9K     | 143.9K     | 148.9K     | 152.1K      |
| [Mara Bos](https://x.com/m_ou_se)                | 41.3K      |  43.3K     |  44.7K     |  45.2K      |
| [rust_foundation](https://x.com/rust_foundation) | 35.6K      |  37.7K     |  39.0K     |  39.8K      |
| [RustTrending](https://x.com/RustTrending)       | 29.5K      |  32.4K     |  34.8K     |  36.0K      |
| [ThisWeekInRust](https://x.com/ThisWeekInRust)   | 31.9K      |  32.3K     |  32.3K     |  32.4K      |
| [RustConf](https://x.com/rustconf)               | 15.3K      |  16.4K     |  17.4K     |  17.7K      |
| [rust_analyzer](https://x.com/rust_analyzer)     | 13.5K      |  13.5K     |  13.1K     |  12.7K      |
| [rustembedded](https://x.com/rustembedded)       | 11.2K      |  11.9K     |  12.3K     |  11.9K      |
| [tokio_rs](https://x.com/tokio_rs)               | 11.6K      |  11.8K     |  11.6K     |  11.3K      |
| [SurrealDB](https://x.com/SurrealDB)             | 7,498      |  8,345     |  8,524     |  8,718      |
| [RustFest](https://x.com/RustFest)               | 7,825      |  7,742     |  7,505     |  7,261      |
| [rustjobs_dev](https://x.com/rustjobs_dev)       | 4,499      |  5,568     |  6,035     |  6,491      |
| [crates.io status](https://x.com/cratesiostatus) | 5.039      |  5,543     |  5,665     |  5,715      |
| [AstraKernel](https://x.com/AstraKernel)         | 1,925      |  3,611     |  4,436     |  5,300      |
| [Orhun Parmaksız](https://x.com/orhundev)        |            |  3,336     |  4,200     |  4,954      |
| [rustlab_conf](https://x.com/rustlab_conf)       | 3,816      |  4,163     |  4,674     |  4,721      |
| [Rust Weekly](https://x.com/RustDiscussions)     | 2,549      |  3,285     |  3,808     |  4,200      |
| [RustSecurity](https://x.com/RustSecurity)       | 3,964      |  4,611     |  3,885     |  3,774      |
| [RustNationUK](https://x.com/RustNationUK)       | 2,157      |  2,738     |  3,215     |  3,684      |
| [letsgetrusty](https://x.com/letsgetrusty)       | 1,418      |  2,387     |  3,029     |  3,286      |
| [euro_rust](https://x.com/euro_rust)             | 2,201      |  2,503     |  2,915     |  3,092      |
| [Daily Rust](https://x.com/rustoftheday)         |   482      |  1,513     |  1,806     |  3,020      |
| [RustLondon_](https://x.com/RustLondon_)         | 2,522      |  2,699     |  2,838     |  2,991      |
| [Guillaume Gomez](https://x.com/imperioworld_)   |            |            |            |  2,774      |
| [ratatui_rs](https://x.com/ratatui_rs)           |    79      |    839     |  1,586     |  2,195      |
| [**RustMaven**](https://x.com/RustMaven)         |     1      |     48     |     31     |     40      |

* Orhun Parmaksız is the maintainer of [Ratatui.rs](https://ratatui.rs/).
* Guillaume Gomez, rustdoc team leader, [mdbook](https://rust-lang.github.io/mdBook/) contributor, Rust Paris organizer.



## Instagram

| account                                                                           | posts | followers | following |
| --------------------------------------------------------------------------------- | ----- | --------- | --------- |
| [rust_programming_language](https://www.instagram.com/rust_programming_language/) | 24    | 4,520     |   3       |
| [rustlanguage](https://www.instagram.com/rustlanguage/)                           |  7    | 1,356     |   1       |
| [rustacean.dev](https://www.instagram.com/rustacean.dev/)                         | 14    |   677     |   9       |
| [rust_language_](https://www.instagram.com/rust_language_/)                       | 19    |   333     | 430       |
| [rust_lang_](https://www.instagram.com/rust_lang_/)                               |  1    |   106     |   6       |
| [Rust Maven on Instagram](https://www.instagram.com/rust_maven/)                  | 36    |    62     |   6       |

See [Rust on Instagram](/rust-in-instagram).

## Rust on Telegram

There might be other groups, but I am not aware of them and I've just created the Rust Maven telegram group so let me put it here:

Number of members.

|  group                                        | 2024.06.19 | 2024.12.16 | 2025.06.17 |
| --------------------------------------------- | ---------- | ---------- | ---------- |
| [Rust Maven](https://t.me/+5P2gCQIWFaBkYmI0)  |  0         |  16        |  17        |

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

Indicates that Rust is number 8 with 2.97% share and +0.4% trend (up from 9 with 2.66% share and +0.5% 1-year trend six month ago) (up from number 10  twelve months ago)

Python is number 1 with 30.63% share and 1.1% trend. (was 1 with 29.71% share and 1.5% 1-year trend six month ago)

C/C++ (bundled together) is number 4 with 7.02% with +0.7% trend (was number 4 with 7.06%  with 0.3% 1-year trend six month ago). (up from number 5 twelve months ago)


[TIOBE](https://www.tiobe.com/tiobe-index/)

Rust is number 18 (dowm from 14 six month ago) (up from number 18 a year ago) with 1.29% ratings and +0.48% change.

Python is number 1 25.87% +10.48% (was 1 with 23.84% ratings and +9.98% change).

C++ is number 2 10.68%, +0.65%  (was 2 with 10.82% ratings and 0.81% change.)

C is number 9.47% +0.24% (was 4 with 9.10% ratings and -2.34% change.)

Java is number 4 8.84% +0.44% (was 3 with 9.72% ratings and 1.73% change).

Also on TIOBE:

* ADA jumped from 25 to 11 in 1 year
* Perl jumped from 17 to 13 in 1 year
* R jumped from 21 to 14 in 1 year

Even if earlier you thought that TIOBE is real this can show you that it has some very serious flaws. I doubt ADA and R would jump that much in a year, but I know that nothing in the Perl world explains this surge in Perl. I think I can say that as I am the editor of the [Perl Weekly newsletter](https://perlweekly.com/) so I think I have some undertanding about the popularity of Perl.

