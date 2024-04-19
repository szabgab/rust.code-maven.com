---
title: "Disk usage: size of file or that of a directory tree using Rust"
timestamp: 2024-04-19T17:30:01
author: szabgab
published: true
description: Measure the size of a file or the size of all the files in a directory tree.
tags:
    - walkdir
    - path
    - is_file
    - metadata
todo:
    - also measure the actual disk usage
---

One of the things I wanted to measure for the [Rust Digger](https://rust-digger.code-maven.com/) project is the size of the Crates and the the
size of the repositories that are used for the development of crates. For this I needed some code that can [traverse a directory tree](/walk-directory-tree)
and then I needed to get the size of each file.

This is going to be the "net size" of the project. The actual disk-space it needs is probably bigger as files are stored in block on the disk,
but for now that was not interesting for me.

## The dependencies for the code

{% include file="examples/disk-usage/Cargo.toml" %}

## The code

{% include file="examples/disk-usage/src/main.rs" %}


As we saw earlier using the [walkdir](https://crates.io/crates/walkdir) it is actually quite easy to traverse a directory tree.
We can then get the [Metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html) struct and from that we can get the size
of a file. I am not sure if I really need this, but I went for the safe side and set the returned number to be a `u128`
which can contain a number up to 340,282,366,920,938,463,463,374,607,431,768,211,455  according to the
table of [minimum and maximum values of the numerical types](/minimum-and-maximum-values-of-numeric-types) I put together earlier.

OK, I guess I could have used u64 and probably u32 would be enough as that's 4Gb.
