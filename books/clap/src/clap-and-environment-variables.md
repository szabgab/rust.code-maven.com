# Clap and environment variables

* env

In order for this feature to work we need to enable both the `derive` and the `env` features of Clap so we start with:

```
cargo add clap -F derive -F env
```

If we don't enable the `env` feature we'll get an error: **no method named `env` found for struct `Arg` in the current scope** with **method not found in `Arg`**.

{% embed include file="src/examples/clap/environment-variable/src/main.rs" %}


In this case we can pass the hostname either as a command line flag or by setting the appropriate environmnet variable.

```
cargo run -- --hostname localhost
DEMO_HOSTNAME=localhost cargo run
```

Setting the envronment can be done on the same row (on linux and macOS) as we see in our example, but it can be also set earlier.


