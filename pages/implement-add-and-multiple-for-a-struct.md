---
title: Implement add and multiply for a struct in Rust
timestamp: 2024-02-10T12:40:01
author: szabgab
published: true
description: We can define the basic mathematical operations for structs as well.
tags:
    - impl
    - self
    - Self
    - Add
    - Mul
    - Output
todo:
    - implement matrix multiplication
---

How to define the basic mathematical operations for structs in Rust?

## Define Vector struct and create an instance of it

We can defined a 2 dimensional vector by the two coordinates it points to:

```rust
struct Vector {
    x: i32,
    y: i32,
}
```

We can then create an instance of this Vector:

```rust
let a = Vector {
    x: 2,
    y: 3,
};
```

## Print the struct - derive from Debug

We cannot print the Vector using this `println!("a: {}",a);` as `Display` is not implemented by default.

We cannot even use the debugging print `println!("a: {:?}",a);` because `Debug` is not implemented by default.

We can implement both by ourselves, but it is easier to just `derive` the `Debug` trait. So for this we'll have

```rust
#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}
```

and then we can use


```rust
println!("a: {:?}",a);
```

or

```rust
dbg!(&a);
```

However, Rust will complain that we have not used the `x` and `y` attributes of the vector. In real code we will probably access those and we won't need to silence this warning, but
here we do. So let's allow for dead code using `#[allow(dead_code)]`:

```rust
#[derive(Debug)]
#[allow(dead_code)]
struct Vector {
    x: i32,
    y: i32,
}
```

We can use the debugging printing in three different ways:

```rust
println!("a: {:?}",a);
println!("a: {a:?}");
dbg!(&a);
```

## Adding two structs together

Given two vectors:

```rust
let a = Vector {
    x: 2,
    y: 3,
};

let b = Vector {
    x: 4,
    y: 5,
};
```

We can try to add them together:

```rust
let c = a + b;
```

This won't compile as Rust does not know how to add two arbitrary structs together. Rust will complain: **must implement `Add`**.

No need to worry, we can implement the [Add trait](https://doc.rust-lang.org/std/ops/trait.Add.html).

```rust
impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

The lowercase `self` will be the variable  that is on the left-hand-side of the operation and the `other` will be on the right-hand-side.
This `other` is expected to be the `Vector` type which is represented here by `Self` (with capital S).

The add function returns a value of the same type (again using `Self` the generic name instead of the specific `Vector`,


Now we can have:


```rust
let c = a + b;
println!("c: {c:?}");
```


## Multiply two structs

In the case of these vectors multiplying one such vector by another vector does not make much sense so I won't implement it.

We can have another example where we implement matrix-multiplication on them, but this isn't that article.


## Multiply a struct with a number

This seemed to make more sense. The idea here is to multiply each coordinate by the same number.

We would like to be able to write code like this, where on the left-hand-side of the operator there is a `Vector` and the right-hand-side is an `i32` number.

```rust
let d = c * 2;
println!("d: {d:?}");
```

For this to work we need to implement the [Mul trait](https://doc.rust-lang.org/std/ops/trait.Mul.html):

```rust
impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
```

However this will only allow `c * 2` it won't handle the `2 * c` case. For that we need to implement that too:

Instead of duplicating all the "calculation", this implementation swaps the two operands and make the code reuse the above implementation.


```rust
impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}
```

## Full example

![](examples/2d-vectors/src/main.rs)

## Output

```
a: Vector { x: 2, y: 3 }
b: Vector { x: 4, y: 5 }
[src/main.rs:55:5] &a = Vector {
    x: 2,
    y: 3,
}
c: Vector { x: 6, y: 8 }
d: Vector { x: 12, y: 16 }
e: Vector { x: 24, y: 32 }
```


