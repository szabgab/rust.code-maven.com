---
title: How much memory does my Rust process use?
timestamp: 2024-06-28T14:30:01
author: szabgab
published: true
description: Can we reliably measure the memory footprint of the current process?
tags:
    - memory-stats
    - thousands
todo:
    - threadded versions
---

In my quest to be able to report and analyze the memory footprint of my process I found the [memory-stats](https://crates.io/crates/memory_stats/) crate.
In the documentation you can see how this crate works. At least on Linux it relies on the memory accounting of the Operating System.

I also used the [thousands](https://crates.io/crates/thousands) crate to [commafy big numbers](/commafy-thousands).

See also the article [memory allocation and usage](/memory-allocation-and-usage) where I checked the total memory used and the available free
memory in the computer.

## Dependencies

{% include file="examples/using-memory-stats/Cargo.toml" %}

## The Code

Basically we fetch the

* "Physical" Memory, which corresponds to the Resident Set Size on Linux and MacOS and the Working Set on Windows.
* "Virtual" Memory, which corresponds to the Virtual Size on Linux and MacOS and the Pagefile Usage on Windows.

both before and after creating a variable with many characters in it and the print the differences.

{% include file="examples/using-memory-stats/src/main.rs" %}

## The results

I ran the process 3 times to see how consistent are the results. (Acaully I ran many more times, but let me show only 3 results here):

For 10,000 and 100,000 bytes I got 0 change in 2 of the executions and got a single 131,072  change for the 3rd run.
Starting from 1,000,000 bytes the results were fairly consisten accross the 3 runs and they also indicate a change in used memory
similar to the siez of the created string.

```
$ cargo run -q
Physical memory usage:       1,966,080
Virtual memory usage:        3,338,240
Bytes            Physical memory   Virtual memory
         10,000               0               0
        100,000               0               0
      1,000,000       1,048,576       1,003,520
     10,000,000       9,961,472      10,002,432
    100,000,000      99,876,864     100,003,840
  1,000,000,000     999,948,288   1,000,001,536
 10,000,000,000   9,999,876,096  10,000,003,072

$ cargo run -q
Physical memory usage:       1,966,080
Virtual memory usage:        3,338,240
Bytes            Physical memory   Virtual memory
         10,000               0               0
        100,000               0               0
      1,000,000       1,048,576       1,003,520
     10,000,000       9,961,472      10,002,432
    100,000,000      99,876,864     100,003,840
  1,000,000,000     999,817,216   1,000,001,536
 10,000,000,000   9,999,876,096  10,000,003,072

$ cargo run -q
Physical memory usage:       1,835,008
Virtual memory usage:        3,338,240
Bytes            Physical memory   Virtual memory
         10,000         131,072               0
        100,000               0               0
      1,000,000       1,048,576       1,003,520
     10,000,000       9,961,472      10,002,432
    100,000,000      99,876,864     100,003,840
  1,000,000,000     999,948,288   1,000,001,536
 10,000,000,000   9,999,876,096  10,000,003,072
```


