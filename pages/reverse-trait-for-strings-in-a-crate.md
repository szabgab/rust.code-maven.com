---
title: Implement the reverse trait for strings in a reusable crate
timestamp: 2024-01-09T20:40:01
published: true
description: If we would like to make the trait we created usable by others as well we can make it reusable by putting it in a stand-alone crate.
tags:
    - trait
    - pub
    - impl
---

In another article we saw how to [implement a trait to reverse a string](/reverse-trait-for-strings). It turns out the implementation was not perfect
and the [unicode_reverse](https://crates.io/crates/unicode_reverse) crate would do a better job at the task, but for showing how to add a trait
this is a nice example.

In this article we'll see two crates. One implements the trait, the other uses it.


## Crate implementing a trait

* We added the `pub` keyword in-front of the word `trait`.
* We also removed the `main` function as that will be in the crate that uses this one.
* We also renamed the file from `src/main.rs` to `src/lib.rs`.
* Lastly we added some tests. This is not required, but it is a good practice to have tests. (I should have added some tests to the previous article as well, but I was lazy.)

![](examples/reverse-trait-for-strings-public/src/lib.rs)

## Add the crate as a dependency

Because the above crate is local only, `Cargo.toml` needs to point to the folder where it is located. (and the full name of the above crate is `reverse-trait-for-strings-public`.

![](examples/reverse-trait-for-strings-use-public/Cargo.toml)

## Using the trait from the other crate

This is basically the same code we had in the previous article when we had the [trait implementation in the same file](/reverse-trait-for-strings).
The only difference is that we have the first line importing the trait:

```rust
use reverse_trait_for_strings_public::Reverse;
```

![](examples/reverse-trait-for-strings-use-public/src/main.rs)

## Conclusion

It is quite easy to create a reusable crate even if we only want to reuse it locally in other crates without publishing it to the whole world.


