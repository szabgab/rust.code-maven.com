# Run async functions

What happens when we call async functions?

In order to demonstrate this we have a function called `say` that gets some text and a number. It sleeps for the given number of seconds using an asynchronous sleep.
This imitates some IO operation our code waits for. In a real application instead of that sleep we might have a call to download a page via HTTP, we might access an API, or a database.
One the waiting is done we print the string so we can observe the order of what happens.

Then we have a number of function each one calling the `say` function with the exact same parameters. We'll observe what happens in each case.

If you'd like to see the behaviour yourself you can run the program using `cargo run` and providing one of the functions as argument:

```
cargo run call
```

## call

```rust
async fn call_say() {
    let _ = say("Hello", 2);
    let _ = say("Hi", 1);
}
```

In the `call_say` case we just call `say` twice but don't `await` the functions, they don't run at all and Rust even warns us about it:

**note: futures do nothing unless you `.await` or poll them**

The total elapsed time is effectively 0 seconds.


## await

```rust
async fn await_say() {
    say("Hello", 2).await;
    say("Hi", 1).await;
}
```

The first call waits for 2 second and prints then the second call waits for 1 second and prints.
Althought we see all the bells and whistles of async code, in reality the calls run sequentially.

The total elapsed time is 3 seconds.


## spawn

```rust
async fn spawn_say() {
    tokio::spawn(say("Hello", 2));
    tokio::spawn(say("Hi", 1));
}
```

In this case we hand over the await-ing to the tokio runtime using the [spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) function.

Both functions start to run at the same time, however, our program might finished before the functions can print anything. This is the case in our example.
The total elapsed time is 0 seconds and nothing is printed. This also means that our program does not wait for the (faked) remote call to return.
Probably not what we want.

## wait

```rust
async fn wait_say() {
    tokio::spawn(say("Hello", 2));
    tokio::spawn(say("Hi", 1));
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
}
```

In this case, after we launch the asynchronous functions calls using spawn we explicitely wait 4 seconds.
In this case too both function calls start at the same time.
The calling function waits long enough to see their output.

The total elapsed time is 4 seconds because we explicitely waited for 4 second. In reality, knowing the code we could have waited for 2 seconds only and most likely
both function calls would have finished, but we can't be sure. Maybe we would want to wait for 2.1 just to make sure and even then we might miss the second call if for some
reason it is stuck before it finishes.

Not ideal as we have to guess how long to wait and even with that we can never be really sure.

## join

```rust
async fn join_say() {
    tokio::join!(
        say("Hello", 2),
        say("Hi", 1),
    );
}
```

Here, we use the [join](https://docs.rs/tokio/latest/tokio/macro.join.html) macro that will spawn the 2 tasks and wait for both of them to finish.
Both functions start at the same time. One of them finishes after 1 second, and the other one after 2 seconds.
The total elapsed time is 2 seconds.

This is much better than what we had previously, but the drawback is that we can do this only if we know all the tasks at compile time.

## join_set

```rust
async fn join_set_say() {
    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(say("Hello", 2));
    tasks.spawn(say("Hi", 1));
    tasks.join_all().await;
}
```

Using [JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html) we can spawn any number of tasks dynamically and wait for all of them to finish.
The functions start as soon as we add them to the JoinSet and finish when they finish. The [join_all](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.join_all)
method will wait till all of them finishes.

In our case the total elapsed time is 2 seconds.


{% embed include file="src/examples/tokio/run-asynchronous-functions/src/main.rs" %}


{% embed include file="src/examples/tokio/run-asynchronous-functions/Cargo.toml" %}


---

* tokio::main
* async
* await
* tokio::spawn
* tokio::join!
* tokio::task:JoinSet
* JoinSet.spawn
* JoinSet.join_all


