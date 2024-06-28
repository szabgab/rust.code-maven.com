---
title: Memory allocation and usage in Rust
timestamp: 2024-03-30T08:30:01
author: szabgab
published: true
description: How to measure the total memory used and the total free memory of a computer?
tags:
    - Rust
todo:
    - memory usage of current process?
---

I wanted to check and report how much memory my process uses. So far I could not find a way to get that informations so I settled the second best thing,
to report how much memory is used and how much is free in the whole computer.

## Dependencies in Cargo.toml

For this I used the [sysinfo](https://crates.io/crates/sysinfo/) crate.
I also used the [thousands](https://crates.io/crates/thousands) crate to [commafy the big numbers](/commafy-thousands) to make them more readable.


{% include file="examples/memory-allocation/Cargo.toml" %}

## The full code

{% include file="examples/memory-allocation/src/main.rs" %}


The user can provide a number and the code will allocate 1,000 times that many bytes.

First we measure and display the total size of the memory in the computer.

Then we show the size of the used memory. This is the baseline.

Then we create a string containing the 1,000 bytes long filler times the number the user provided.
This will take up some memory in the computer. We measure the size of the used memory inside the
`allocate` function while the variable with the long sting is still in scope.


Finally, after leaving the `allocate` function the variable goes out of scope and thus Rust can deallocate
the memory use by that long string. At this point we measure the used memory again.

After we have done this we show how much did the used memory grow with the data allocation and how much did it shrink
after the deallocation.

## Result

My computer has 32 Gb memory though `htop` and the `free` command only repor 27Gb. Strange.

```
$ free
               total        used        free      shared  buff/cache   available
Mem:        28491496     4068548    23020620      166596     2005024    24422948
Swap:        8388604     3938792     4449812
```

Allocating too much data will crash the process:

```
$ cargo run -q 40000000
Size: 40,000,000 filler: 1000 total: 40,000,000,000 bytes

Total memory:                            29,175,291,904
Used memory before allocation:            4,136,079,360
memory allocation of 40000000000 bytes failed
Aborted (core dumped)
```

I am not sure why in one case it prints `Aborted (core dumped)` and in another case it prints `Killed`.


```
$ cargo run -q 35000000
Size: 35,000,000 filler: 1000 total: 35,000,000,000 bytes

Total memory:                            29,175,291,904
Used memory before allocation:            4,168,376,320
Killed
```

IF the data fits into the memroy then we can see the changes by the allocation and deallocation of data
both being round 20 Gb.


```
$ cargo run -q 20000000
Size: 20,000,000 filler: 1000 total: 20,000,000,000 bytes

Total memory:                            29,175,291,904
Used memory before allocation:            3,206,164,480
Used memory after allocation:            23,201,665,024
Used memory after deallocation:           3,182,731,264
Memory used by allocation (diff):        19,995,500,544
Memory freed by deallocation (diff):     20,018,933,760
```


## Strange results

When we try to allocate only a few bytes we get some really strange results.

```
$ cargo run -q 2
Size: 2 filler: 1000 total: 2,000 bytes

Total memory:                            29,175,291,904
Used memory before allocation:            3,296,440,320
Used memory after allocation:             3,276,918,784
Used memory after deallocation:           3,279,728,640
Memory freed after allocation (diff):        19,521,536
Memory used by deallocation (diff):           2,809,856
```

In this example we only allocate `2*1000` bytes, but that still means our process
is using memory. Despite that it seems that while we are using more memory, the total
used memory got smaller and while we free our 2,000 bytes the system started to use more
memory.

That seems to be totally incorrect.

I can think of two explanations:

1. Either our measurements are not exact and so this is a measuring error.
2. Other processes that also run in the system (e.g. my browser, dropbox, or even the terminal) also allocate and deallocate memory while my experiment runs.

## Conclusion

The `sysinfo` crate can help us understand the size of the used and the free memory of the system, which can be very useful, but we have to be carful
not conclude much about the memory usage of our current process as other processes will impact the numbers.

We might be able to use this to avoid filling the memory of the computer and thus crashing, but it would be better to have a way to know
how much memory our current process is using.

