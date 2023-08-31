---
title: Enumerate and match in Rust
timestamp: 2023-08-31T12:50:01
description: Create an enum (enumerate) in Rust to be able to identify distinct values from a set of values.
todo:
    - Extend this article
    - Show the error message of a match where not all the arms were defined.
    - Show the use of _ in a match.
    - Explain the derive entries and allow dead_code
tags:
    - enumerate
    - enum
    - match
    - dead_code
    - Debug
    - PartialEq
---

The `enum` keyword in Rust allows us to enumerate a number of values and then differentiate among them.
E.g. The days of the week, the months of the year, distinct colors, etc.

Once you defined an enum you can assign the individual values of the enum to variables and then you can compare the variable to the possible values of the enum.

There are two major ways to compare the values. One of them is dircetly using the `==` operator. In this case we compare to a specific individual value.

The other option is to use a `match` where the "arms" of the match must cover all the possibilities as you can see in the following example:

![](examples/enumerate_and_match.rs)

The output of the above code is:

```
Blue
Red
is background red?  false
is background blue? true
red
blue
```

## More about enums

Read the [defining enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) part of the Rust book, see the [Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) part of the Rust by example, or read the documentatin on [enum](https://doc.rust-lang.org/stable/std/keyword.enum.html).


