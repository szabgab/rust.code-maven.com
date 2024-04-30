---
title: An almost infinite Fibonacci Iterator
timestamp: 2024-04-30T17:30:01
author: szabgab
published: true
description: Create an iterator that will return the Fibonacci series
tags:
    - Iterator
    - next
    - Self
    - Option
    - None
    - Item
    - struct
    - new
    - checked_add
    - for
---

Rust provides a nice way to create an iterator based on a struct. In this example we are creating an iterator that will allow us to iterate over the Fibonacci numbers.

Some iterators have a clear range of things they iterate over, this one is unbounded. That is it will never stop unless we break out from the iteration rule.

While theoretically this could go on forever there is one issue that we have to solve. What happens if the series outgrowth the variables that supposed to store the value?
What should happen when we reach [the maximum value](/minimum-and-maximum-values-of-numeric-types) and the variable is about to overflow.
There are number of [ways to handle number overflow](/how-to-handle-overflow). In this implementation I decided I'd like to stop the iteration.

I also used the `u8` value type to store the Fibonacci numbers so it will be easy to demonstrate the overflow as the max value of u8 is 255.


In order to implement the iterator we need th struct that will hold the parameters of the iterator. e.g. the current value, or current values, the limit, if there is one.
In our case we needed two attributes for the last two values of the Fibonacci series and we don't have a limit.

```rust
#[derive(Debug)]
struct Fibonacci {
    next_value: u8,
    current: u8,
}
```


It is also recommended to implement the `new` method of the struct. This is not a requirement, but it is a cleaner
way to initialize the attributes and thus the whole iterator. `Self` here represents the current struct which is `Fibonacci`,
but it is cleaner to write `Self` as it allows us to rename the struct without changing in the definition of
the function as well.

```rust
impl Fibonacci {
    fn new() -> Self {
        Self { next_value: 1, current: 0 }
    }
}
```

The most important part is to implement the [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait for the struct.
Inside the trait we need to define the `type Item` and we need to implement the `next` function that will calculate the next element
of the iteration.


The `next`function must return an `Option<Self::Item>` that must contain either the next value in `Some(value)` form or `None`
if the iterator ran out of values. In this example we don't have a user-provided upper limit, but we wanted to gracefully end
the iteration just before the value overflows. So we used the [checked_add](https://doc.rust-lang.org/std/primitive.u8.html#method.checked_add)
method that returns and `Option` containing either the sum of the two numbers or `None` if there would be an overflow.


```rust
impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_value) = self.current.checked_add(self.next_value) {
            self.current = self.next_value;
            self.next_value = next_value;
            Some(self.current)
        } else {
            None
        }
    }
}
```

## How to use the iterator?

There are two examples showing how to use the iterator. In both we use the [for in loop](/for-in-loops) to iterate
over the values, but in the first case we `break` out if we read a limit. In the second case we let it run till the overflow.


```rust
    for fb in Fibonacci::new() {
        println!("{fb}");
        if fb > 30 {
            break;
        }
    }

    for fb in Fibonacci::new() {
        println!("{fb}");
    }
```


## The full code

{% include file="examples/iterators/fibonacci/src/main.rs" %}

## Output


```
$ cargo run -q
1
1
2
3
5
8
13
21
34
------
1
1
2
3
5
8
13
21
34
55
89
144
```


