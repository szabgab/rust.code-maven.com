---
title: Change a single character in a string in Rust
timestamp: 2024-04-03T10:46:01
author: szabgab
published: true
description: We can use the replace_range method to replace a single character in a string
tags:
    - replace_range
    - macro_rules
    - range
---

In order to replace a string character in a string, first we need a **mutable** ([mut](https://doc.rust-lang.org/std/keyword.mut.html))  [String](https://doc.rust-lang.org/std/string/struct.String.html).

In addition we need the [replace_range](https://doc.rust-lang.org/std/string/struct.String.html#method.replace_range) method.

As you might understand from its name, it is designed to replace a **range** of characters, but that range can consist of a single
character as well.

In this example we use a macro created using `macro_rules!` to print out the length, capacity, and the memory address of the string. See more about them
in [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html).

The first call to `replace_range` will reaplce the range between character 10 and 12 (both edges included) where we have "cat" by the word "dog".

In the second call of `repalce_range` you can see what you came for, replacing a single character with another character. (lower-case 'd' by upper-case 'D').

In the first two calls neither the length, nor the capacity of the string changed and the memory address also remained the same.

In the 3rd call to `replace_range` we replaced a singel character, in the range (10..=10), by 3 characters ('qqq`). This made Rust replace that single character
by 3 characters. This meant the string had to enlarged. So the lengths is bigger, the capacity is bigger and the memory address has changed!
This also means for this operation Rust had to copy the whole string to a new location in memory.

In the 4th call we did the opposite. Replaced 3 characters by 1. This made the string shorter (len decresed from 37 to 35, but the allocated capacaity remaind the same
and the memory location remaind the same.

In the next example we used some emojies. As you might know while each latin letter takes up 1 byte, and each emoji takes up 4 bytes.

In the 5th call we had to replace a range of 4 bytes - a single emoji character. That's how we replace a strange-looking snake (I think it looks more like a green duck),
but let's assume it a python with a friendly crab.

In the next call we take again 4 bytes that contain the emoji of a dog and replace it with the word "dog". Because the latter only has 3 bytes the length of the string
changed from 20 to 19.

Finally we try to replace the range of bytes 10..=15 by a crab. However, that range does not correspond to a real character that causes Rust to panic!.

In other words, **don't do that**! Make sure you always replace full Unicode charactes by whatever you'd like to replace them with.

## The code and the output

{% include file="examples/change-character-in-string/src/main.rs" %}

```
len: 35 capacity: 35 0x5ae3e8463ae0 "The black cat climed the green tree"
cat
len: 35 capacity: 35 0x5ae3e8463ae0 "The black dog climed the green tree"
len: 35 capacity: 35 0x5ae3e8463ae0 "The black Dog climed the green tree"
len: 37 capacity: 70 0x5ae3e8463ba0 "The black qqqog climed the green tree"
len: 35 capacity: 70 0x5ae3e8463ba0 "The black zog climed the green tree"

len: 20 capacity: 20 0x5ae3e8463bf0 "🐈🦮🦍🐍🐪"
🐍
len: 20 capacity: 20 0x5ae3e8463bf0 "🐈🦮🦍🦀🐪"
len: 19 capacity: 20 0x5ae3e8463bf0 "🐈dog🦍🦀🐪"
thread 'main' panicked at /rustc/ae7/library/alloc/src/string.rs:1908:29:
assertion failed: self.is_char_boundary(n)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

