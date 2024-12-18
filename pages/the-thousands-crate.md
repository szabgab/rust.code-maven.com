---
title: "Rust Source Code Reading: The thousands crate"
timestamp: 2024-12-17T09:30:01
author: szabgab
published: true
description:
---

The [thousands](https://crates.io/crates/thousands) crate is a small crate that will help "commafy" a number. That is, put commas between triplets of digits to make the number more readable. So it will convert 1234567 to 1,234,567.

During this even we go over the implementation of this crate.

{% youtube id="ITTj7ByNStE" file="2024-12-17-rust-code-reading-the-thousands-crate-english.mp4" %}

Part of the [live events](/live).


Yesterday I had the first live code-reading event. I attempted to understand how the "thousands" crate works.  🦀 It can commafy a number converting a number such as 1234567 into a string like this "1,234,567" to make it more readable.


A couple of observations, and let me emphasize, this is not a (negative) critique of the crate or the author.


🦀 If there are tests, starting from them can help understanding the flow of the code. Unfortunately in some cases the tests themselves are a bit too complex. (This was not such case.)


🦀 If there is a nice and simple example of how to use the crate, we can start from that code trying to explore the implementation.


🦀 We could observe (or rather guess) the evolution of the crate from specific solution (dealing with digits) to more generic solution (dealing with any set of characters). Some variable/function/etc. names were using the generic vocabulary, some other were still in the original way talking about digits.


🦀 There are multiple ways to define type-constraints (e.g. immediately after the type-name or in a "where"-clause. Mixing them can confuse Gabor. (Maybe others as well. )


🦀 The audience in such code-reading can help.


🦀 One hour was not enough even for such small crate and even after I already invested some time trying to understand it.


----


🦀 Watch the recordings [here](https://youtu.be/ITTj7ByNStE)

🦀 If you are interested in earlier (and future) live events check them out [here](/live).




