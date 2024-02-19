---
title: "Boolean not !"
timestamp: 2023-12-06T07:00:01
author: szabgab
published: true
description:
tags:
    - "!"
    - not
    - bool
    - boolean
    - "true"
    - "false"
---

There are several cases when we might want to negate the value of a boolean expression.

Two are shown here, let me know if you think about more distinct cases I should include here.


{% include file="examples/boolean-not/src/main.rs" %}


## Negate a boolean value

In the first case we have a method called [is_empty](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty) that will return `true` if
the string is empty. What if we would like to check for `not empty` ? There is no such function but we can just negate the value returned by
`is_empty`:

```rust
if ! string.is_empty() {
```

## Toggle

This is a `bool`, a boolean value. If it was `true` then after this operation it will be `false` and if it was `false` then
after this operation it will be `true`.

I use such toggle in a game where I have various modes and each one can be turned `on` and `off` separately.
So each one will have such a variable.

```
on = ! on;
```

