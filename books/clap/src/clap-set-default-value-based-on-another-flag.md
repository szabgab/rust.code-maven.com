# Clap - set default value based on another flag


* `mode` is required argument.
* The value of `log_level` is based on the `mode`.
* We can also set the `log_level` manually.

```
$ cargo run -q -- --mode devel
Args: Cli { mode: Devel, log_level: 1 }

$ cargo run -q -- --mode test
Args: Cli { mode: Test, log_level: 2 }

$ cargo run -q -- --mode release
Args: Cli { mode: Release, log_level: 0 }

$ cargo run -q -- --mode release --log-level 10
Args: Cli { mode: Release, log_level: 10 }
```

{% embed include file="src/examples/clap/default-value-if-equals-multiple-values/src/main.rs" %}



