---
title: Add the Display trait to a vector of strings in Rust
timestamp: 2023-12-21T09:50:01
published: true
description: How to implement a trait for a built-in type or for a type-alias for a built-in type?
tags:
    - Rust
    - type
    - struct
    - Display
    - error code
---

## The problem

Let's say we have a vector of strings, how can we print them easily?

```rust
let animals: Vec<String> = vec![String::from("snake"), String::from("camel"), String::from("crab")];
```

Trying to use the simplest way won't work:

```rust
println!("{}", animals);
```

We get the following error:

```
error[E0277]: `Vec<String>` doesn't implement `std::fmt::Display`
  --> src/main.rs:24:20
   |
24 |     println!("{}", animals);
   |                    ^^^^^^^ `Vec<String>` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Vec<String>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

further referring us to [E0277](https://doc.rust-lang.org/error_codes/E0369.html)

# We can do what was suggested:

```rust
println!("{:?}", animals);
```

will yield

```
["snake", "camel", "crab"]
```

and

```rust
println!("{:#?}", animals);
```

will yield:

[
    "snake",
    "camel",
    "crab",
]
```

Those are actually not bad, but I wanted to show them in a different way.

Primarily as an experiment to see how to implement **[Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)** in general for a vector.

## Implement Display for Vec<String>

The implementation should look something like this:

```rust
impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Something comes here")
    }
}
```

However, this will give us a compiler error:

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  --> src/main.rs:23:1
   |
23 | impl fmt::Display for Vec<String> {
   | ^^^^^^^^^^^^^^^^^^^^^^-----------
   | |                     |
   | |                     `Vec` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
```

Referencing us to [E0117](https://doc.rust-lang.org/error_codes/E0117.html).

## How can we implement the Display trait for a vector in Rust?

We cannot as we can only implement traits to our own data types.

What about creating an alias for this type?  Would crating a type-alias work?

## How to implement traits for type aliases in Rust?

```rust
type Words = Vec<String>;

impl fmt::Display for Words {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Something comes here")
    }
}
```

```rust
let animals: Words = vec![String::from("snake"), String::from("camel"), String::from("crab")];
println!("{:?}", animals);
```

We can define the type-alias and we can even use it in the declaration of variables, but we cannot
implement the Display trait on a type-alias to a standard type. Type-aliases are just that. Aliases.
They are not separate types.



## Using a one-element tuple-based struct

A working solution is to create a one-element tuple-based struct. Because it is a totally new type we can implement the Display trait for that.

```rust
struct Words(Vec<String>);

impl fmt::Display for Words {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(","))
    }
}
```

This is then how to use it:

```rust
let animals: Words = Words(vec![String::from("snake"), String::from("camel"), String::from("crab")]);
println!("{}", animals);
```

It will print:

```
snake,camel,crab
```

The drawback is that this means we need to reference to the 0th element in the tuple so we must write `animals.0` in order to access the real vector:


```rust
println!("{}", animals.0[0]);
println!("{}", animals.0[1]);
```

to print:

```
snake
camel
```

I don't think I like this.

## The full example

![](examples/display-vector-of-strings/src/main.rs)


## Conclusion

We could not implement **Display** for a vector of strings in a nice way.
If we frequently need to print out a whole vector of strings other than for debugging,
then we might want to write a separate function or a macro for that.



