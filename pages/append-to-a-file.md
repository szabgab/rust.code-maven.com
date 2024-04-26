---
title: Append to a file
timestamp: 2024-04-26T17:30:01
author: szabgab
published: true
description: We usually write a file from the start to the end, but sometime, for example in the case of log files we only wish to append to the end of the file.
tags:
    - options
    - append
    - create
    - open
    - writeln!
---

We already know how to [write to a file](/write-to-a-file), but that way we overwrite the whole file losing any content it had earlier.
In most cases that's what we really want. Sometimes, however, especially in the case of log-files, we would want to keep the original
content and append something to the end of the file.

We can accomplish this by opening the [File](https://doc.rust-lang.org/std/fs/struct.File.html) in `append` mode.
We do that by using the [options](https://doc.rust-lang.org/std/fs/struct.File.html#method.options) method.
We can stack the various modes on each other.

In this case we set both the `append` and the `create` mode to `true`.

The actual writing is the same as in the other case using the [writeln!](https://doc.rust-lang.org/std/macro.writeln.html) macro.


{% include file="examples/files/append-to-file/src/main.rs" %}


## Usage of the example:


```
cargo run -q hello
cargo run -q "second row"
cargo run -q "3rd row"
```

The result will be a file called "messages.txt" with all those lines.





