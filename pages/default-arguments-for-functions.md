---
title: Default arguments for functions in Rust using macros
timestamp: 2023-12-19T17:00:01
author: szabgab
published: true
description: Rust does not allow the declaration of default values to function parameters, but we can create macros for that.
tags:
    - macro
    - macro_rules!
    - expr
todo:
    - an introduction to macros
---

Unlike Python and several other programming languages, Rust does not allow setting default values to function parameters.
Luckily with a little bit of macro-writing we can imitate it.

Here is the whole example

![](examples/default-arguments/src/main.rs)

When we run this we get the following output:

```
prompt 'What is your secret?' 3 times
prompt 'Still with me?' 4 times
prompt 'What is the default?' 5 times
```

## Explanation

```rust
fn prompt(text: &str, count: u8) {
    println!("prompt '{}' {} times", text, count);
}
```
We have a function that accepts two parameters. A text and a number. If really implemented it I'd have ask the user with the prompt up to count times to answer the question, but
we are not here for that.

We would like to have a default value for the `count` parameter, but Rust does not have that feature.


## Use of the function

In the `main` function above you can see how can we use the function:

```rust
prompt("What is your secret?", 3);
```

## Use of the macro

You can also see that we can use the `prompt!` macro both with and without the 2nd argument.

```rust
prompt!("Still with me?", 4);
prompt!("What is the default?");
```

## Write a macro

```rust
macro_rules! prompt {
    ($text: expr, $count: expr) => {
        prompt($text, $count);
    };

    ($text: expr) => {
        prompt($text, 5);
    };
}
```

The line `macro_rules! prompt` means that our new macro will be called `prompt` and we will be able to use it as `prompt!`. That [exclamation mark](/exclamation-mark)
at the end of the name indicates that this is a macro. This is also good, because this way we can use the same name as our function.

Inside the macro definition there are two rules.


The first rule expects 2 expressions that will be called `$text` and `$count`.
We could really call them anything, but using the same names as are used in the function seemed to make sense.
If the compiler encounters `prompt!` with two parameters, this rule will be matched and the macro will be replaced by a call to the `prompt` function passing the two parameters to it.


```rust
($text: expr, $count: expr) => {
    prompt($text, $count);
};
```

The second rule expects only one parameter we called `$text` and then replaces the macro with a call to the `prompt` function with the
parameter we received and the number 5 which is now the default value of the `count` parameter.

```rust
($text: expr) => {
    prompt($text, 5);
};
```

## Conclusion

Macros can be really simple and useful.


