---
title: 3 ways to handle number overflow or underflow in Rust
timestamp: 2024-01-07T19:50:01
published: true
description: One needs to decide what to do if a number is about to outgrow the current variable type - overflow or underflow.
tags:
    - overflow
    - underflow
    - saturating_add
    - checked_add
    - unchecked_add
    - add_with_overflow
---

In the web application in some field I am expecting a non-negative integer number. Obviously I need to verify that the user does not supply a string or a floating point number.
Back in my previous life when I was writing mostly Perl and Python I would probably use a regex for this. Now with Rust I just use `parse()` into the appropriate unsigned integer
and check if it was successful or not. I need to decide which unsigned integer.

As this web application is supposed to be one of the [counter examples](https://code-maven.com/counter) I'd take the number, increment by one and return to the client.

As I was thinking about this it occurred to me the incremented number might be too big for the selected unsigned integer.
In another article I listed the [minimum and maximum values of the numeric types of Rust](/minimum-and-maximum-values-of-numeric-types). So I thought I joke about this and I wrote a post:

> Since I started to write #Rust I started to see edge cases where I've never seen earlier. e.g. What if the "like" count on my post reaches 340282366920938463463374607431768211455 and another person "likes" it? It will overflow the u128...
> How should I handle it?
> #rustlang makes you anxious

I though it is funny as it is quite unlikely (pun intended) that I will receive that many likes to my posts, but some people responded seriously.

That's how I learned about **saturating add** then I found out about [saturation arithmetic](https://en.wikipedia.org/wiki/Saturation_arithmetic).

## What should be the result?

My question did not make much sense, but in the general case one needs to decide what to do.

* Should MAX_VALUE+1 become 0?
* Should MAX_VALUE+1 stay MAX_VALUE?
* Should we warn the user?
* Should we return an error to the user?
* Should we report to the operator of the system?

There is no one right answer. It depends on the use-case.

For example with "likes" I'd probably stay on MAX_VALUE, maybe with an extra message or maybe even an extra flag saying "more than MAX_VALUE". E.g. the way LinkedIn show the number of connections. Up till 500 it shows you the exact number, above 500 it just says "more than 500".

In extreme case one can even have a second (and third) variable and act as if we are in base MAX_VALUE. We could even have a vector of these numbers, but even then we'd have some limit...

## Let's see the possibilities in Rust

In order to make it easier to follow I am going to use `u8` variables, but the idea is the same with any [integer type](/minimum-and-maximum-values-of-numeric-types).

* Just increment without any special treatment

```rust
let mut count: u8 = 254;
println!("count: {}", count);
for _ in 1..=3 {
    count += 1;
    println!("count: {}", count);
}
```

In debug mode we get a `panic!`:

```
$ cargo -q run

count: 254
count: 255
thread 'main' panicked at src/main.rs:5:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In release mode it silently overflows and we get 0.

```
$ cargo -q run --release

count: 254
count: 255
count: 0
count: 1
```


## Saturating add

There is a method called [saturating_add](https://doc.rust-lang.org/std/intrinsics/fn.saturating_add.html)

```rust
let mut count: u8 = 254;
println!("count: {}", count);
for _ in 1..=3 {
    count = count.saturating_add(1);
    println!("count: {}", count);
}
```

It will return the the sum, or the max value of the given type:

```
$ cargo -q run
count: 254
count: 255
count: 255
count: 255
```

Same result in release mode.

### Checked add

We can use the [checked_add](https://doc.rust-lang.org/std/primitive.u8.html#method.checked_add) method to decide what to do and
even do some extra work.

```rust
let mut count: u8 = 254;
println!("count: {}", count);
for _ in 1..=3 {
    match count.checked_add(1) {
        Some(val) => count = val,
        None => eprintln!("Too much!"),
    };
    println!("count: {}", count);
}
```

Same result in release mode.

It does not change the value and it returns an [Option](https://doc.rust-lang.org/std/option/enum.Option.html).
In the `Some` part we get the incremented number. In the `None` part we can decide what to do.
Keep the max value as we have here, set `count` to 0, warn on STDERR as we do here.

```
$ cargo -q run

count: 254
count: 255
Too much!
count: 255
Too much!
count: 255
```

## overflow_add

[overflow_add](https://doc.rust-lang.org/std/primitive.i8.html#method.overflowing_add) is another strange way to handle the situation. I am not sure when would I use this.

It returns a tuple, the first element being the incremented number, the second element beeing a `bool` indicating if there was an overflow.

```rust
let mut count: u8 = 254;
println!("count: {}", count);
for _ in 1..=3 {
    let (new_count, overflow) = count.overflowing_add(1);
    count = new_count;
    println!("count: {:3} {}", count, overflow);
}
```

The result:

```
$ cargo run -q

count: 254
count: 255 false
count:   0 true
count:   1 false
```


I could also totally desregard the second value:

```rust
let mut count: u8 = 254;
println!("count: {}", count);
for _ in 1..=3 {
    (count, _) = count.overflowing_add(1);
    println!("count: {}", count);
}
```

The result:

```
$ cargo run -q

count: 254
count: 255
count:   0
count:   1
```


## Unchecked add

There is also an [unchecked_add](https://doc.rust-lang.org/std/primitive.u8.html?search=Some#method.unchecked_add), but it is experimental so I won't try it now.


## Add with overflow

There is also [add_with_overflow](https://doc.rust-lang.org/std/intrinsics/fn.add_with_overflow.html) which is also experimental.

## All the source code:

![](examples/overflow/src/main.rs)

## The 20-year-old bug in binary search caused by overflow

John Corbett pointed at the  "Implementation Issues" section of [Binary search algorithm](https://en.wikipedia.org/wiki/Binary_search_algorithm) where it is discussed that
an overflow bug has existed in most of the implementations of the Binary search for way too many years. Very interesting discussion.

## Conclusion

One needs to think hard what **should** happen when a variable holding an integer is changed to an unsupported value and then there are several ways to handle them.

I have not checked what is the runtime impact of using either of the above solutions.


