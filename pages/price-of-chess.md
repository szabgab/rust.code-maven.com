---
title: The price of Chess is 18446744073709551615 grains of rice
timestamp: 2023-11-22T11:00:01
published: true
description:
tags:
    - u128
    - pow
    - commafy
    - chess
---

Back when I was a kid [my father](https://en.wikipedia.org/wiki/L%C3%A1szl%C3%B3_Szab%C3%B3_(chess_player)) told me the anecdote
about the man who invented chess and then sold it to the Persian Shah. He did not ask for much. Just one grain of rice on the first
square. Twice as many on the 2nd square and so on. According to the story the Shah laughed how little the man asked, but then could
not pay it.

When I was a kid I always tried to figure out how many grains of rice that is. My father told me that it is 2^64-1, but that was meaningless to
me. I remember I had tons of pieces of paper where I wrote down the calculations like this:

```
       1
       2
       4
       8
      16
      32
      64
     128
     ...
```

I don't remember ever reaching the end.

No with Rust it is easy, I just need to use the **pow** function:

![](examples/price-of-chess/src/main.rs)

```
price: 18446744073709551615
price: 18,446,744,073,709,551,615
weight in gram: 461168601842738800
weight in kg: 461168601842739
weight in metric ton: 461168601843
weight in metric ton: 461,168,601,843
```

That's a lot of rice. As I found on some places a grain of rice is about 0.02-0.04 gram

ps. Writing this post I found out that one [ton](https://en.wikipedia.org/wiki/Ton) is not always 1,000 kg. Oh.


