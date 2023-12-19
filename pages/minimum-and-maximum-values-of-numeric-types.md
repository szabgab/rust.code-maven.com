---
title: Minimum and maximum values of Rust numeric types
timestamp: 2023-10-03T07:00:01
published: true
description: To avoid overflowing or underflowing the numeric types one can check against the minimum and maximum possible values.
tags:
    - MIN
    - MAX
    - u8
    - i8
    - u128
    - i128
    - f32
    - f64
    - isize
    - usize
---

If you'd like to make sure that your computations don't result in a number that cannot fit into the numeric type you are using,
you can always makes some comparison before the computation to the minimum and maximum values of the numeric types.

Here you can see the MIN and MAX values of most of the numeric types of Rust:

![](examples/min-max-values/src/main.rs)

![](examples/min-max-values/src/out.txt)


