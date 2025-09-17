# Hello World in async

```
cargo add tokio -F full
```

Or manually edit the `Carto.toml` file to add the dependency.

{% embed include file="src/examples/tokio/hello-world/Cargo.toml" %}

Later you might fine-tune which features to include, but `full` is a good start.

## The code

We prefix our async functions with the `async` keyword and we decorate the `main` function with `#[tokio::main]` to convert that function to be our runtime.

There are other ways to use `tokio`, but this is probably the most common and also the easiest way as well.

We also need to add the `await` at the end of our async function calls to make them actually work.

{% embed include file="src/examples/tokio/hello-world/src/main.rs" %}

