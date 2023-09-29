---
title: Print double quotes in a string in Rust (")
timestamp: 2023-09-29T13:30:01
description:
tags:
    - r
    - #
    - \
    - escape
---

We can include basically any character in a string in Rust, but we cannot include a double-quote (**"**) as that marks the end
of the string. There are two ways: escaping the character or using [raw strings](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals)

In the following example we try to print some text and include the content of the variable. The first print statement works as expected.

What if we would like to put that value in quotes? There is no problem in putting single-quotes around that value as single-quotes
have no special treatment in a string in Rust.

However, if we would like to put the value in double-quotes we need to work a bit harder.

In the 3rd example we put a back-slash in-front of the quote character to tell Rust to forget about the special meaning of the double quote. We **escaped** it.

The back-slash `\` is called the escape character in Rust, just as it is in probably every other programming language.

In the 4th example we can see the use of [raw string literal](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals)

We start the string with `r#"` and terminate it with `"#`. We could have used any number of the pound-characters there (`#`) between 1-255 we only need to make sure the numbers are matching on both sides of the string. You can see this in the 5th example.

Now that we changed the end-of-string from a single `"` to a series of characters, we can freely use the double-quote `"` inside the string.

This is especially valuable as we need to add more and more of these double-quotes to our string.

## The code

![](examples/print-quotes/src/main.rs)

## The output

```
Hello, Foo Bar, how are you?
Hello, 'Foo Bar', how are you?
Hello, "Foo Bar", how are you?
Hello, "Foo Bar", how are you?
Hello, "Foo Bar", how are you?
```


