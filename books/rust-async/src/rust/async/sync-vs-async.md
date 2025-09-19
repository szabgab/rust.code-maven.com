# sync vs async

Before we continue down the rabbit whole of async, let's see the difference in the usage of regular (synchronous) and async functions.

In this example we hav a regular (synchronous) function brilliantly named `sync_func` and and an async function called `async_func`.
You can observe that the only difference between these two functions in the `async` prefix on the async function.
This won't be the case once we really startt to explore async, but for this example we want to have two functions that behave identically once they start to run.

The question what is the difference in the way we call these functions?

We sprinkled the `main` function with print-statements so we'll see what gets executed when.

When we call the `sync_func()` it gets executed immediately and returns 42, the value that the function returns.

When we call the `async_func()` it returns a Future, that is something that has the [Future](https://doc.rust-lang.org/std/future/trait.Future.html) trait.
At this point the body of the function is not executed and thus we can observe that nothing is printed between "Before calling async_func" and "Before await".

We can then write `future.await`. It is not really a function call as one can observe by the lack of parentheses `()` at the end of of `await`. It is an instruction to the async runtime (that is tokio in our case) to stop execution the current function and let some other task running. In our case the only other task that was already initiated is the content of the `async_func`. So at this point it will get executed.

The expression with the `await` will terminate when the `async_func` ends and it will return whatever the `async_func` returns. In this case 42.

{% embed include file="src/examples/tokio/sync-vs-async/src/main.rs" %}

## The output:

{% embed include file="src/examples/tokio/sync-vs-async/out.txt" %}

## Cargo.toml

{% embed include file="src/examples/tokio/sync-vs-async/Cargo.toml" %}

