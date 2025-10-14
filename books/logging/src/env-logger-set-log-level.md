# Set log level on the command line

It is nice that one can set the log-level using the `RUST_LOG` environment variable, but if your project already has various command-line options then it seem to make sense to be able to set
the log level via a command-line flag as well. Here is one way to do it uisng the [env_logger](https://crates.io/crates/env_logger) and the [log](https://crates.io/crates/log) crates.


{% embed include file="src/examples/logging/env-logger-set-log-level-on-the-command-line/src/main.rs" %}

Try some of the following commands:


The default log level is "warn":

```
cargo run
```


Set the log level to `debug` using the command line:

```
cargo run -- --log debug
```

Set the log level to `info` using the environment variable:
```
RUST_LOG=info cargo run
```

Set the log level to `debug` using the command line:
The command-line option overrides the environment variable.

```
RUST_LOG=info cargo run -- --log debug
```

Invalid log-levels will be stopped when parsing the command-line argument list:

```
cargo run -- --log duck
```


{% embed include file="src/examples/logging/env-logger-set-log-level-on-the-command-line/Cargo.toml" %}

