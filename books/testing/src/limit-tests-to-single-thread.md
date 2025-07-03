# Limit tests to single thread



* [Testing with tempfiles and environment variables](./testing-with-environment-variables.md)
* Using change-dir in a test which is also process-wide


On the command line:

```
cargo test -- --test-threads=1
```


Create the `.cargo/config.toml` file and put the following in it:

```
[env]
RUST_TEST_THREADS = "1"
```

