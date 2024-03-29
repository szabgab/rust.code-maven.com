---
title: Commanfy using the thousands crate
timestamp: 2024-03-29T16:00:01
author: szabgab
published: true
description: Put a comma after every 3 digits from the right-hand side to make the numbers more readable.
tags:
    - commafy
    - thousands
    - separate_with_commas
---

A while ago I wrote an example showing how to [commafy a number](/commafy), that is how to make numbers more readable
by separating every 3 digits? e.g. Instead of 1234567 it is much more readable to have this: 1,234,567.

I was pointed to the [thousands](https://crates.io/crates/thousands) crate that provides a trait called `separate_with_commas`
that does exactly this.

Here is how to use it:

## See Cargo.toml for the dependencies

{% include file="examples/commafy-thousands/Cargo.toml" %}

## See the code:

{% include file="examples/commafy-thousands/src/main.rs" %}

## See the output

```
     12345     12,345
       123        123
    -12345    -12,345
      -123       -123
   -1234.5   -1,234.5
```
