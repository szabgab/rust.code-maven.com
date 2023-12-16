---
title: Wrap text in Rust
timestamp: 2023-11-23T01:30:01
published: true
description: When you have a long text that you'd like to spread on several shorter rows.
tags:
    - textwrap
    - wrap
---

Using [textwrap](https://crates.io/crates/textwrap) it is very easy to take a line of text and split it up into chunks that are all less then a certain number of characters.

I needed this in the project creating the banners and the images that are generated from the title of each page on this site to be used when the pages are shared on social networks.


Create a new crate

```
cargo new wrap
cd wrap
```

Add the `textwrap` crate:

```
cargo add textwrap
```

This is the code:

![](examples/wrap/src/main.rs)

This is the result, pretty-printed using `#` to make it easier to see the new rows:

```
This is some long text that we need to wrap to fit in a given width
[
    "This is some long text that",
    "we need to wrap to fit in a",
    "given width",
]
[
    "This",
    "is",
    "some",
    "long",
    "text",
    "that",
    "we",
    "need",
    "to",
    "wrap",
    "to",
    "fit",
    "in a",
    "give",
    "n",
    "widt",
    "h",
]
```


