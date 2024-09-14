---
title: Replace function instead of mocking for testing
timestamp: 2024-09-13T12:30:01
author: szabgab
published: true
description: How to mock a function for testing without and mocking library?
tags:
    - cfg
    - test
---

There are plenty of case when we testing would be expensive or slow or not deterministic if we were using the system as it was intended.

For example if part of the system needs to access some external API to fetch data and rely on that data for computation.
Accessing that external API might be relatively slow, or it might cost some money that we would like to avoid.

During testing we might want to avoid that.

One of the common solutions is to **mock** the API call and have a fixed set of responses to a fixed set of requests use in the tests.
Mocking can be also used to fake errors in the API so we can test how our code behaves in case of such failure.

In Rust one of the ways we can do that is by using configuration attributes and replacing the function implementing the API call
by a totally different function. The string that is new to me that in this case the mocking unction is also part of the code-base.

We can mark functions to be compiled into the code only when testing and other functions to be compiled when we are not testing. That is development or production mode.

If we mark a function with

```rust
#[cfg(test)]
```

then it will be compiled in the code only for testing.

If we mark a function with

```rust
#[cfg(not(test))]
```

then it will be compiled in the code every time when we are not testing.

## The code that has a function we might want to avoid calling during testing

This is the code. `get_temp` is a function that calls an external API.
As I did not want to really implement an API call here I just used the `sleep` function to
pretend it is a slow-running function.

We would like to test the `diff` function that calls the `get_temp` twice and then does some "complex" computation.

Now, in a better written application maybe `diff` function would already receive the results we got from `get_temp`,
but I am not such a good programmer.

At the bottom of the file we can see how to test this code.

{% include file="examples/function-to-avoid-during-test/src/lib.rs" %}

You can also observer how the function is being used in the `main.rs`:

{% include file="examples/function-to-avoid-during-test/src/main.rs" %}

`cargo run` takes 8 seconds because we have 4 API calls, but that's unavoidable, that's how the application works:

```
$ time cargo run -q
-5
1

real	0m8.389s
user	0m0.188s
sys	0m0.214s
```

The issue is that the tests are also slow and might expensive if the API provider charges money.

The single test takes 4 seconds because it has 2 API calls.


```
$ time cargo test
...
real	0m4.102s
user	0m0.044s
sys	0m0.056s
```

How can we avoid the API calls while testing?


## How to avoid calling the slow and expensive API call during the tests?

{% include file="examples/avoid-function-for-test/src/lib.rs" %}

The trick is to replace the long-running or expensive function with another function. We can call it mocking it, but this works in a slightly different
way than I used in Python and Perl.

We add `#[cfg(not(test))]` to the function we would like to replace so this version of the code will be only used when **not testing**.

We add another version of the same function: the same name, the same signature, but inside it is just simple look-up table to map input to fixed output.
We mark the function with `#[cfg(test)]` so it will compiled in the application only during testing.

There rest of the code does not have to change. Neither the tests, nor the way the library is used.

Here is also the `main.rs` which is identical to the previous version. Regular use of the code is not impacted.

{% include file="examples/avoid-function-for-test/src/main.rs" %}

`cargo run` will still take 8 seconds. There is nothing we can do about that.

`cargo test` however now runs 0.167s. That's because now the `diff` function will called the **mocked** version of the **get_temp** function
that returns immediately.


## Conclusion

This can solve the issue when one of our functions has to be mocked.

It is rather strange to me and I am not sure how much I like the fact that we don't even compile the real code during tests and that the mocking code
is part of the application. True it won't be compiled into the released binary, but it still feels strange that it is rather far from the test.

I am not sure how would I make it return different value for different test-cases.

I don't know if this could be used to mock function that are in dependencies.

AS it is compiled into the code whenever I run tests I don't know how could I test the real function as it is never present when we `cargo test`.


On the other hand this is a very simple and readily available solution.
