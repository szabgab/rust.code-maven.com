---
title: Get the time elapsed since the epoch using only std
timestamp: 2024-02-10T10:30:01
author: szabgab
published: true
description: Get the nanoseconds elapsed since January 1, 1970, the beginning of the (UNIX) world.
tags:
    - time
    - SystemTime
    - UNIX_EPOCH
    - epoch
todo:
    - same using chrono
    - link to chrono
---


In this example using only `std`, the standard library we fetch the time elapsed since the epoch (which is 1970 January 1 on Unix/Linux systems).

![](examples/epoch/src/main.rs)


Running on Linux this is the output:

```
$ cargo run -q

start:           SystemTime { tv_sec: 1707553108, tv_nsec: 951575643 }
since_the_epoch: 1707553108.951575643s
as_nanos:        1707553108951575643
as_micros:       1707553108951575
as_millis:       1707553108951
as_secs:         1707553108
as_secs_f32:     1707553200
as_secs_f64:     1707553108.9515758
```


