---
title: "Debug formatting using :?, :#?, and dbg!"
timestamp: 2024-03-12T12:30:01
author: szabgab
published: true
description: There are various ways to easily format the content of variables to be shown for debuggng purposes.
tags:
    - ":?"
    - ":#?"
    - dbg!
    - format!
    - println!
    - Debug
todo:
    - Show :#?
---


If we would like to print the content of a variable we can use the `print!` or `println!` macros, or, sepecially if we need it for debugging purposes
we can use the `dbg!` macro. We could also use ther `format!` macro if we only wanted to format the string without immediately printing it to the screen.

* The placeholder `{}` can be used for any primitive types, such as string, integer, floating point, boolean.
* The placeholder `{:?} can be used for both orimitive types and complex types such as tuple, vector, or struct. In order to be able to use it for a `struct` we need to add the `Debug` trait to the struct.
* the placeholder `{:#?}` can be used to pretty-print the content of any variable.
* `dbg!` will print the valu in the same way as `{:#?}` does, but it will also include the name of the variable, the filename and the row-number. It will print to STDERR as would `eprint!` and `eprintln!` do.

See the example code:


{% include file="examples/format-debug/src/main.rs" %}

See the result from the example code:

```
Hello, world! 42 true
"Hello, world!" 42 true
"Hello, world!" 42 true

("text", 42)
["Aunt Em", "Uncle Henry", "Betsy Bobbin"]
Person { name: "Mary Jane", birthyear: 1997, addresses: ["home", "work"] }

(
    "text",
    42,
)
[
    "Aunt Em",
    "Uncle Henry",
    "Betsy Bobbin",
]
Person {
    name: "Mary Jane",
    birthyear: 1997,
    addresses: [
        "home",
        "work",
    ],
}

[src/main.rs:37:5] text = "Hello, world!"
[src/main.rs:38:5] integer = 42
[src/main.rs:39:5] boolean = true
[src/main.rs:40:5] tuple = (
    "text",
    42,
)
[src/main.rs:41:5] vector = [
    "Aunt Em",
    "Uncle Henry",
    "Betsy Bobbin",
]
[src/main.rs:42:5] person = Person {
    name: "Mary Jane",
    birthyear: 1997,
    addresses: [
        "home",
        "work",
    ],
}
```
