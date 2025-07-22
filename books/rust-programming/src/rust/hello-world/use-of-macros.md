# Use of macros with parentheses, square brackets, or curly braces

In our first example we used the `println!` macro with parentheses: `println!()` that made it look like a function call.

Rust allows us to use macros with square brackets or with curly braces.

In most cases people use regular parenthese, but for example the [vec! macro](https://doc.rust-lang.org/std/macro.vec.html) is usually used with square brackets.

You can even leave spaces between the exclamation mark and the opening brace, but I don't think I have seen that. I'd probably avoid that.

The default formatting style of Rust is having no space when parenthese or square brackets are used and having a single space when curly brace is used.

{% embed include file="src/examples/intro/use-macro/src/main.rs" %}

* [Macros - the reference](https://doc.rust-lang.org/reference/macros.html)

