---
title: Compiling and running rust without Cargo - a shell script
timestamp: 2023-08-30T14:30:01
description: When you just want to experiment with some Rust code it might be easier to use this wrapper around rustc than to create a new crate.
tags:
    - rustc
    - Bash
---

In the [Hello World](/hello-world) article we first saw an example compiling some Rust code using `rustc` and then running the resulting binary manually.
I mentioned it is not really the common way to compile and run rust and then we saw how to use `Cargo` for that.

However, and I guess this might be slightly controversial, as I experiment with Rust I write lots of small programs. As long as I don't use an external crates, I don't really need to use cargo. I can just do the compilation and running as above.

But I am lazy and I have written way too much **perl** and **python** code to be able to execute two commands just to compile and run my code.

So I wrote a small **Bash** script that will do the two steps for me.

![](examples/rust.sh)

I'd use it like this:

```
cd examples
./rust.sh hello.rs
```

It will compile the Rust file and call the binary file `myexe`, then it will try to run that program.

Here is the Hello World program in case you'd like to try it.

![](examples/hello.rs)

## Conclusion

I know it does not have a lot of uses and I am sure there are various issues with this script, but it is quite useful for me.
Maybe you can use the idea and even improve on it.

