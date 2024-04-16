---
title: Set the fileextension in Rust
timestamp: 2024-04-16T12:30:01
author: szabgab
published: true
description: The set_extension isn't always the right tool to set the extension of a file.
tags:
    - PathBuf
    - set_extension
todo:
    - TODO
---

The [PathBuf](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html) struct has a method called [set_extension](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.set_extension).
One would thing that it is the tool that we always need to use to, well, set the extension of a file.

Unfortunately it has some caveats. Specifically that if the file has a dot in it, the `set_extension` will consider the part after the last dot as the current extension and it will replace it with the new one.

So when I was working on the [MSRV](https://rust-digger.code-maven.com/msrv) report of the [Rust Digger](https://rust-digger.code-maven.com/) this created a mess.

It took me some time to understand the problem and then went for the axe, or at least the `format!` macro that you can also see in the examples:


{% include file="examples/set-file-extension/src/main.rs" %}


