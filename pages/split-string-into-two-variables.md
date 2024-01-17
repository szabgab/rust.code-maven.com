---
title: Split a string into two variables
timestamp: 2023-12-13T09:40:01
author: szabgab
published: true
description: Splitting a string into parts is easy. Making sure it had the correct number of parts makes the code longer.
tags:
    - split
    - split_once
    - collect
    - turbofish
    - Some
---

We have a string that has two parts separated by a character. e.g. a colon like this: **"localhost:5000"**. How can we assign the two parts to two separate variables?

In this example you'll see 4 solutions. Each one has some advantages and some disadvantages.

The shortest solution would be this one:

```rust
if let Some((hostname, port)) = text.split_once(':') {
    println!("hostname: {hostname}");
    println!("port: {port}");
}
```

However if we cannot be sure that the original string will be of the correct format, then this might give us an invalid result.
Especially if the input string has more than one `:` characters then everything that is after the first `:` will end up in the variable called `port.

So if the input is `localhost:5000:garbage` then the `port` will contain `5000:garbage`.

## Proper solutions

The solutions somehow need to handle the cases when the original string has more than one separators (`:`) and when it has none.

In the first solution, called `with_split`, we split the string into parts at the delimiter (`:`) using `split`, `collect` with [turbofish](/turbofish). We check if we got exactly two values,

In the second solution, called `short_with_split_once_and_some`, we use the `split_once` method. This means that if we have more than one `:` then the second variable will contain the rest of the string.
In this solution we did not deal with the problem.

The 3rd solution, called `with_split_once_and_some`, is the same as the previous, but now we check if we could split the string properly and report if we could not.

The 4th solution, called `with_split_once_and_unwrap`, will `panic!` if there are more than one colons `:`.

![](examples/split-string-into-two-variables/src/main.rs)

## Conclusion

If you we confident that the input has the correct format then we can be lazy and write shorter code, but if we want to be robust then we might need to write more code.

