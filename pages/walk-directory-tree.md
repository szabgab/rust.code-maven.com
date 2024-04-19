---
title: Walk directory tree - traverse the filesystem
timestamp: 2024-04-19T15:40:01
author: szabgab
published: true
description: Walk the directory tree and print everything
tags:
    - walkdir
---

I needed this for the [Rust Digger](https://rust-digger.code-maven.com/), but actually I bumped into the [Walkdir](https://crates.io/crates/walkdir) crate
totally by chance while I was looking for the crates with the biggest number of transitive reverse dependencies (popularity) on [stats page of lib.rs](https://lib.rs/stats).
I saw [memchr](https://crates.io/crates/memchr) and looked at the looked at the [GitHub Sponsors page of Andrew Gallant](https://github.com/sponsors/BurntSushi), the author of both.

Anyway, the crate has some very nice and clear examples and what you see here is basically the same, but I had to try it myself before I start using it in the application.


## Dependency

{% include file="examples/walk-directory-tree/Cargo.toml" %}

## Code

{% include file="examples/walk-directory-tree/src/main.rs" %}


The first example, that is actually commented out in the code will traverse the whole tree starting from the root we provide.

```rust
for entry in WalkDir::new(root) {
    let entry = entry.unwrap();
    println!("{}", entry.path().display());
}
```

The second example will skip some things, specifically it will skip the "target" folder.


This is the output I get in the folder of the crate I used for the example:

```
$ cargo run -q .
.
./Cargo.lock
./Cargo.toml
./src
./src/main.rs
```


