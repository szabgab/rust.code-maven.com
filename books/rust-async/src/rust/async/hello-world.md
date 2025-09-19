# Hello World in async

Let's see the standard "Hello World!" example using async.

Just remember, in this example, although we have the async code it does not actually do anything asynchronous. It only shows the basic syntax, the basic mechanism.

First we create a new crate called demo and change to its directory.

```
cargo new demo
cd demo
```

In order to write `async` code using the `tokio` crate we need to add it to our project:

```
cargo add tokio -F full
```

Alternatively we can manually edit the `Cargo.toml` file to add the dependency:

{% embed include file="src/examples/tokio/hello-world/Cargo.toml" %}

We use the `full` feature here as that makes it easy to use all the features.  Later you might fine-tune which features to include. However, `full` is a good start.

## The code

We prefix our async functions with the `async` keyword and we decorate the `main` function with `#[tokio::main]` to convert that function to be our runtime.
Actually in reality the real main function cannot be `async` and using this small syntactic sugar tokio will wrap our main function with a real main function, but let's not worry about it now.

There are other ways to use `tokio`, but this is probably the most common and also the easiest way as well.

We also need to add the `await` at the end of our async function calls to make them actually work.

{% embed include file="src/examples/tokio/hello-world/src/main.rs" %}

## Running the code

There is nothing special in running the program we use the regular `cargo run` with the silencer:

```
$ cargo run -q
Hello, world!
Hello, async world!
```

---

* tokio::main
* async
* await
