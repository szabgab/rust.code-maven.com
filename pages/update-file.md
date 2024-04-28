---
title: Update file
timestamp: 2024-04-28T10:30:01
author: szabgab
published: false
description:
tags:
    - Rust
todo:
    - TODO
---

There are several ways to "update a file".

In one case we open the file for reading read the whole file into memory then separately open the file for writing and write the content.


Another way is to open the file for both reading and writing at the same time, read the content, rewind the file pointer, truncate the file to 0
and then write the new content.

In this example we open the file using the [options](https://doc.rust-lang.org/std/fs/struct.File.html#method.options) by setting the 3 flags:

* read
* write
* create



{% include file="examples/files/update-file/src/main.rs" %}


