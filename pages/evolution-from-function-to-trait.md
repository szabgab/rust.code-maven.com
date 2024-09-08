---
title: Evolution from function to trait in 4 steps
timestamp: 2024-09-08T10:10:01
author: szabgab
published: true
description: External function, external function for struct, method of struct, trait.
tags:
    - struct
    - impl
    - trait
---

In our application we would like to be able to do some computation. E.g. we would like to calculate the area of a rectangle.

We see 4 different implementations

## Simple function

The most simple way is to implement a function that will receive the width and length of the rectangle and return the area.
Besides calling the function `area` there is nothing special about this function. It only multiplies two numbers.

The two parameters can be any two numbers (of the given type). They might be named width and length, but nothing really indicates
that are measures of a rectangle.

This is a very generic solution, but does not convey much meaning.

{% include file="examples/area-function/src/lib.rs" %}

## Struct with external function

A better representation might be to define a `struct` called **Rectangle** that has two attributes for width and length.
We could then call the area function from the previous implementation like this:

```rust
let r = Rectangle { width: 2, length: 3 };
let result = area(r.width, r.length);
```

It is probably cleared if the area function receives an instance of `Rectangle` (or better yet a reference to an instance)

{% include file="examples/area-struct-and-function/src/lib.rs" %}

This brings us to a generic function floating in our namespace with a very specific application. It can only be used with a Rectangle.

## Struct with method

Instead of defining a function outside of the struct we could define it as a method of the `Rectangle` struct.
We need to wrap the function definition it with `impl Rectangle { }` and replace the parameter that was `rect: &Rectangle` by `&self`
which is the generic representation of an instance of the current struct.

Also in the test just as in general use-case, instead of passing in a reference to the function `area(&r)`, we use the dot-notation to call it as a method: `r.area()`.

{% include file="examples/area-struct-and-method/src/lib.rs" %}


## Struct and Trait

To allow for easy generalization of the `area` method we could define a `Trait` that defined the signature of the area function:
It receives a reference to an instance of the current struct and returns a `u64`.

```rust
pub trait Area {
    fn area(&self) -> u64;
}
```

Then we implement the trait using the `impl Area for Rectangle` expression instead of the `impl Rectangle` expression we had earlier.

The rest of the code does not need to change, but now we defined the idea of having a method called `area` which we'll be able to implement
for any shape and convey the message that thy have something common.

{% include file="examples/area-struct-and-trait/src/lib.rs" %}



