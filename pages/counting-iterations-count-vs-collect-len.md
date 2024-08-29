---
title: Counting iterations - count() vs collect().len()
timestamp: 2024-08-29T09:30:01
author: szabgab
published: true
description: If we don't care about the content of the iterator juts the number of iterations, we can use the count method.
tags:
    - count
    - collect
    - len
    - next
    - drop
---

Given an iterator, how can we know how many elements it returns?

Unlike with vectors if we have an iterator we cannot just simply fetch the number of iterations it has.
We actually need to iterate over all the iterations and count somehow.

Doing so will consume the iterator so if later on we do want to go over the values as well then we'll have to create a new iterator.

However there are cases when you don't care the actual content, you are only interested in the number of iterations.
For example I was writing an [implementation of wc in Rust](/implementing-wc-in-rust) where I had to count the number of words.

I wrote this:

```rust
let words = text.split_whitespace().collect::<Vec<_>>().len();
```

but I was told using `count` instead of `collect().len()` is much better:

```rust
let words = text.split_whitespace().count();
```

Let's see why?


## Ways to count iterations

There are several ways to count the number of iterations.
In the following example there are 3 ways.

In order to be able to demonstrate what's going on behind the scenes each iteration returns a struct and there is a print-statement both in the `new`
method and in the `drop` method of the struct. That way we will be able to see when each struct is created and when is it destroyed.

### For-in loop

This is probably the most natural for many people coming from other languages. We use a `for-in` loop to iterate over the elements
and we use a separate variable called `counter` to count the number of iterations.

```rust
let mut counter = 0;
let range = Range::new(3, 5);
for num in range {
    counter += 1;
    println!("{num:?}");
}
println!("for loop: {counter}");
```

The output looks like this:

```
Creating 3
Number { num: 3 }
Dropping Number! 3
Creating 4
Number { num: 4 }
Dropping Number! 4
Creating 5
Number { num: 5 }
Dropping Number! 5
for loop: 3
```

That is, in every iteration Rust creates a struct and then destroys it. That means we always have only one copy of the struct in the memory.

### Using collect().len()

The second way uses [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) to create a vector and then calls `len` on the vector.

```rust
println!("collect.len: {}", Range::new(3, 5).collect::<Vec<_>>().len());
```

This is the output:

```
Creating 3
Creating 4
Creating 5
collect.len: 3
Dropping Number! 3
Dropping Number! 4
Dropping Number! 5
```

That means first we go over all the iterations and create all the structs and only when we are done with the `len()` call only then
the structs are dropped. That means the number of structs we hold in the memory is the same as the number of iterations.
With 3 iterations it is not a big issue, but if we have a 1,000,000 and if each struct is big then it will require a lot of memory.

### Using count()

In the 3rd case we use the [count](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) method on the iterator.

```rust
println!("count: {}", Range::new(3, 5).count());
```

This is the output:

```
Creating 3
Dropping Number! 3
Creating 4
Dropping Number! 4
Creating 5
Dropping Number! 5
count: 3
```

This looks like the first case when after creating each struct it is immediately dropped.
That means at every time we only have one copy of the struct in the memory regardless of the number of iterations.

## Conclusion

In the generic case we can see that, if we don't care about the content of the iterations, just the number of them, then
the most compact and most efficient solution is using the `count()` as it only need memory allocation for one struct at a time.
It still goes through all the iterations and still creates and drops each struct, but at least it only needs a relatively
small amount of memory.

## Conclusion about our case

If we look at our case, there is not struct involved, it is just splitting up a string. If we use the `collect()` call without the
`len()` call then we can observer (at least in the IDE) that the returned vector contains `&str`-s. (`Vec<&str>`).

```rust
let words = text.split_whitespace().collect::<Vec<_>>();
```

That means the parts of the original string are not copied, they are referenced to in the vector. It still requires memory allocation
of the references though. So even in this case it better to use the `count()`.

It seems both the `collect().len()` and the `count()` will do the same amount of memory allocation, but the `count()` method
will reuse the memory while the `collect().len()` will need all the memory at the same time.


## Full code

{% include file="examples/count-vs-collect-len/src/main.rs" %}

## Output

{% include file="examples/count-vs-collect-len/out.txt" %}

