---
title: Evolution of error handling in Rust
timestamp: 2024-03-26T16:00:01
author: szabgab
published: true
description: How to write a function that will let the errors bubble up the call stack.
tags:
    - unwrap
    - match
    - "?"
    - Result
    - Ok
    - Err
    - ParseIntError
---

We are going to see a case where a function we write might encounter errors. This will be a relatively simple case when every error has the same type.

We implement a very simple and rather useless function, but one that can demonstrate the problem. This function accepts two strings that each contains an integer.
The function converts the string to real integers `i32` and then adds them together.

The conversion looks like this:

```rust
a.parse::<i32>();
```

but the problem is that this might fail so this statement will return a `Result` enum. We have to somehow handle this.

## Use unwrap to sweep it under the carpet and let the code panic!

In our very first attempt we call `unwrap` on the `Result`. In case the parse succeeds this will return the parsed `i32` value,
but if the `parse` fails this will `panic!`.

I've also included a test-case. Unfortunately, because the function panics on bad input we cannot test that case.

{% include file="examples/add-strings-unwrap/src/main.rs" %}

As you can see in this output, if we supply values that cannot be parsed into `i32` we get runt-time `panic!`.

```
$ cargo run -q 2 8
10

$ cargo run -q 2.1 8
thread 'main' panicked at src/main.rs:11:30:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo run -q 2 8.2
thread 'main' panicked at src/main.rs:12:30:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Use match and exit in case of failure

Instead of using `unwrap` we can also use one of the other error handling methods.
The ultimate solution, however is to use `match` and handle the two arms of the `Result`
explicitly.

This needs a lot more code. We could hide that code in a macro, but that just adds to
the level of difficulty understanding the code.

{% include file="examples/add-strings-or-exit/src/main.rs" %}

The nice thing about this solution is that the application does not `panic!` any more.
The function just prints an error message and exits.

```
$ cargo run -q 3 7
10

$ cargo run -q 3.1 7
Failed converting the value '3.1': invalid digit found in string

$ cargo run -q 3 7.2
Failed converting the value '7.2': invalid digit found in string
```


Unfortunately we still cannot test the cases with the invalid input.
Well, we could if we ran the tests as separate processes, but that's
more work than one would want to invest in such small program.


## Return a `Result` and use `?` to shorten the code in the function.

In the 3rd solution we decided to propagate the errors to the caller.


Luckily (well, that's how the example was created) all the errors that might be
generated in the function have the same type. So we change the signature of our function
to also return a `Result` that contains either an `i32`, if everything was fine, or it
will contain an [ParseIntError](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)
if something went wrong.

This is the definition of the return value:

```rust
Result<i32, std::num::ParseIntError>
```

Inside the function we don't need to use `unwrap`, nor do we need to use `match`.
Instead we add a `?` at the end of each function call that returns a `Result`.

This means that if the `parse` function call is successful the `i32` will be assigned to
the respective variables and the code will go on running.
If `parse` returns an Error then the function will be immediately terminated and the error
will be returned.

We can do this easily because each error inside the function has the exact same type.

We also have to wrap the actual result our function returns in an `Ok()`.

In the caller, in our case in the `main` function we'll now have to handle the returning `Result`.
However now it is only one place, instead of each internal function call, and it is one level
higher up in the call-stack.

{% include file="examples/add-strings-return-result/src/main.rs" %}


Finally, now that the function does not `panic!` and does not `exit` we can write test for
the invalid input case as well.

The output from running the code will also work without crashes:

```
$ cargo run -q 4 6
10

$ cargo run -q 4.1 6
invalid digit found in string

$ cargo run -q 4 6,2
invalid digit found in string
```

## Conclusion

Ok, so this was a relatively easy case as all the internal errors were of the same type. In another article
I'll review the case when we have different error types.

