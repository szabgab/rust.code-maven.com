---
title: Create a zip file (tarball) from a folder
timestamp: 2024-05-22T16:00:01
author: szabgab
published: true
description: Create a tarball (a single, compressed file) from the content of a directory.
tags:
    - tar
    - zip
    - gzip
    - flate2
    - compress
---

In the following example we can see how to create a `tar.gz` file from the content of a folder.


## Dependencies

{% include file="examples/create-zip-file/Cargo.toml" %}


## Code to create a tarball from the content of a folder

{% include file="examples/create-zip-file/src/main.rs" %}



