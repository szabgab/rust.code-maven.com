---
title: Automatically fixing issues in Rust code
timestamp: 2024-01-18T10:30:01
author: szabgab
published: true
description: Using the --fix flag Clippy can automatically fix some of the issues it finds in our code.
tags:
    - Clippy
    - fix
todo:
    - TODO
---

I admit I am still a bit scared of this, but as quite a few people have suggested it, I thought I'd give it a try.

Rust and Clippy can help fixing issues in our code. More specifically we can can run `cargo clippy --fix` and it will try to fix the issues Clippy reports.

Let's see a simple example.

We have this code that we have already used when we discussed [using the clippy::pedantic lints and setting priority](/simple-case-of-pedantic-lints).

![](examples/uninlined-format-args/src/main.rs)


## Clean
We can run the following

```
cargo clippy -- -Dclippy::pedantic -Aclippy::no_effect_underscore_binding  -Aclippy::uninlined_format_args
```

and it will report that everything is fine.


## Error

We can enabled the `uninlined_format_args` lint (by removing the appropriate allow flag) and we'll get the error:

```
cargo clippy -- -Dclippy::pedantic -Aclippy::no_effect_underscore_binding
```

and we'll get the error:

```
    Checking uninlined-format-args v0.1.0 (/home/gabor/work/rust.code-maven.com/examples/uninlined-format-args)
error: variables can be used directly in the `format!` string
 --> src/main.rs:4:5
  |
4 |     println!("Hello, {}!", name);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args
  = note: `-D clippy::uninlined-format-args` implied by `-D clippy::pedantic`
  = help: to override `-D clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`
help: change this to
  |
4 -     println!("Hello, {}!", name);
4 +     println!("Hello, {name}!");
  |

error: could not compile `uninlined-format-args` (bin "uninlined-format-args") due to previous error
```

## Fix

I assume your code is in git. So before running the next line make sure your workspace is clean. There are no changes that have not been committed yet.

Not the `--fix` flag that comes before the `--`.

```
cargo clippy --fix -- -Dclippy::pedantic -Aclippy::no_effect_underscore_binding
```

This will report that:

```
Fixed src/main.rs (1 fix)
```

We can check the fix using `git diff`:


```diff
$ git diff

diff --git a/examples/uninlined-format-args/src/main.rs b/examples/uninlined-format-args/src/main.rs
index 08f12e1..0ead540 100644
--- a/examples/uninlined-format-args/src/main.rs
+++ b/examples/uninlined-format-args/src/main.rs
@@ -1,5 +1,5 @@
 fn main() {
     let name = "Rust";
     let _x = 1;
-    println!("Hello, {}!", name);
+    println!("Hello, {name}!");
 }
```

## Can't fix everything

If we remove the `no_effect_underscore_binding` from the command line and try to fix that:


```
cargo clippy --fix --allow-dirty -- -Dclippy::pedantic  -Aclippy::uninlined_format_args
```

Clippy will just report the failure and won't make any changes to the code.

So as we can see Clippy can't fix every issue.


## What can be fixed automatically?

It seems to be quite logical in this example that one of the issues can be fixed by Clippy and the other can't.

The fix for the `uninlined_format_args` lint, is trivial. Just inline the variables.

However, the fix for the `no_effect_underscore_binding` lint is a bit unclear.
After all there might be some good reason I have this line of code. (Besides trying to demo how to use Clippy.)

```rust
let _x = 1;
```

It might be some left-over code that could be removed, but it also might be the preparation for some new code that I did not have time to finish yet.

In the former case the fix is to remove the code.
In the latter case the right fix is to implement whatever I've started to write.

There might be other reasons as well that I can't think of now, but it is already clear that the fix is not obvious or trivial.

## How to reduce the anxiety?

So Clippy can "fix" my code, but how can I make sure it did not break anything? How can I reduce my fear and anxiety?

I will "eyeball" the changes, but that's not enough.

Writing automtated tests (and running them !) (also in a CI) can bring us a lot closer to being relaxed. After all if we have extensive test coverage of
our code and the possible data, then we can be reasonably sure that breaking changes will not creep in silently.

Neither by myself, nor by my co-workers, nor by Clippy.

## Conclusion

I am still a bit scared of `--fix` and besides at this point I'd like to manually fix the issues reported by Clippy so I can learn about the issues
and about Rust more, but I think I got a lot more confidence and I might even start to employ this flag.

After writing some more tests.


