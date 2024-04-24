---
title: drop, the destructor of Rust structs
timestamp: 2024-04-24T18:30:01
author: szabgab
published: true
description: We can add a method called drop to each struct and it will be called when the struct is destroyed.
tags:
    - drop
    - impl
    - struct
    - mut
---

In Object Oriented Programming languages you can usually define a special method that will be called when the instance goes out of scope or otherwise destroyed.
The generic name of this method is "destructor" though in different programming languages the specific name might be different.

Rust does not have classes and object in the same way as OOP languages have, but it has **structs** and we can implemented a method called `drop` that will be the
destructor of that struct. Meaning it will be called when the struct is destroyed.

It is described in the [Running Code on Cleanup with the Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html) chapter of the Rust book, but as always I was interested
to see it myself. I was also wondering, how does this behave in case of a `panic!`?

This is a very simple and rather stupid example in which the user needs to supply two numbers and we divide one by the other. This is a nice way to allow the user
to create a `panic!` without explicitly calling `panic!` which might have other impacts.

We also have two instances of the `Thing` struct that does not do much, but has a `drop` method implemented.


{% include file="examples/drop/src/main.rs" %}

## When there is no panic!

The first time I run the code I verify that 10 divided by 2 is 5. That works nicely.
You can see that basically after the `main` function ends so when the variables `a` and `b` go out of scope the program calls the `drop` method of the struct that prints the two messages.

You can also notice that the messages are printed in the reverse order how the variables were created.

```
$ cargo run -q 10 2
10 / 2  =  5
drop for second
drop for first
```


## When there is panic!

So what happens if we try to divide by 0?

```
$ cargo run -q 10 0
10 / 0  =  inf
drop for second
drop for first
```

Oups, there is no panic! Apparently Rust can divide by 0. Well, at least floating point numbers. If I replace the `f64` by some integer, eg. `u64` then we would get a `panic!` here.


Ok, but no all is lost we can try something else:

```
$ cargo run -q 10 zero
thread 'main' panicked at src/main.rs:24:42:
called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
drop for second
drop for first
```

That's much better. Now the `unwrap` panicked.

As you can see the `drop` method was still called.

That's the same if I run with `--release`

```
$ cargo run -q --release 10 zero
thread 'main' panicked at src/main.rs:24:42:
called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
drop for second
drop for first
```

## Whet if we replace the content of the variable? (shadowing)

At the end of the code we have two special cases.

In the fist case, if the `dividend` is 42 then we create a variable and then we replace it with another instance of the same struct.

Because this what Rust refers to as **shadowing** the first occurrence of `c` is not destroyed when the new `Thing` is assigned to it.
(Despite the fact that in this example they will go out of scope together.)
That means the `Thing` which is called **apple** is dropped only at the end of the if-block after the one with the **banana** is dropped.


```
$ cargo run -q --release 42 2
42 / 2  =  21
after the apple was created
after the banana was created
drop for banana
drop for apple
drop for second
drop for first
```


## What if we replace the mutable variable?

In the 2nd special case we mad the variable `d` mutable. In this case the result is slightly different.

The struct called **cat** is dropped immediately when the struct called **dog** is created.

```
$ cargo run -q --release 2 42
2 / 42  =  0.047619047619047616
after the cat was created
drop for cat
after the dog was created
drop for dog
drop for second
drop for first
```



