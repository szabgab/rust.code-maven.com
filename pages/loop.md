---
title: loop in Rust
timestamp: 2024-04-30T12:30:01
author: szabgab
published: true
description:
tags:
    - loop
    - rand
    - random
---

Sometimes you have a loop that where the exit condition of the loop is complex, or for other reason you don't want to put it in the [while](/while-loop) condition.

In some languages there is a `do while` loop that executes the first iteration and only after that checks the condition for the first time.

In Rust you could write something like:

```rust
while true {
    ...
    if end-condition {
        break;
    }
}
```

but it is better written with the [loop](https://doc.rust-lang.org/std/keyword.loop.html) keyword.

```rust
loop {
    ...
    if end-condition {
        break;
    }
}
```


The `loop` loop is different from the [while loop](/while-loop) or from the for loop in that it can also return a value.
So we can write:


```rust
let result = loop {
    ...
    if end-condition {
        break 42;
    }
}
```

Let's see a full example. This is quite similar to what we had showing the [while loop](/while-loop), but this time the `break` returns
the last number.

## The dependency

In this example we use the [rand](https://crates.io/crates/rand) crate.

{% include file="examples/loop-loop/Cargo.toml" %}

## The code

{% include file="examples/loop-loop/src/main.rs" %}

## Output

On every run we'll get a different series of numbers:

```
$ cargo run -q
7
16
24
27
37
46
48
58
The result is 58
```


## Infinite loop

I used to call such loops "infinite loops" and the [rust documentation of loop](https://doc.rust-lang.org/std/keyword.loop.html) uses the expression **Loop indefinitely**,
but recently I started to feel that it isn't the right description. Of course there might be cases when you would like to loop indefinitely,
I just have never encountered one. Every time I used this construct I had some kind of exit-condition so I used `break` at least once.

That's why these days I use the term "loops where the condition is checked in the body of the loop". It is longer than "infinite loops",
but I feel it describes the real-world use-cases better.


