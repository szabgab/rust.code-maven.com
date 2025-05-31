# Clap - set default value if other argument provided

* default_value_if
* ArgPredicate
* IsPresent

* This is a similar example but now we say that if the user provides a name for the logfile then we'll automatically turn on logging to that file.
* This might be more logical than the previous one, but I am not sure I like this either.

{% embed include file="src/examples/clap/default-value-if-ispresent/src/main.rs" %}

```
$ cargo run -q
Args: Cli { log_to_file: false, log_file: "my.log" }

$ cargo run -q -- --log-to-file
Args: Cli { log_to_file: true, log_file: "my.log" }

$ cargo run -q -- --log-file other.log
Args: Cli { log_to_file: true, log_file: "other.log" }

$ cargo run -q -- --log-to-file  --log-file other.log
Args: Cli { log_to_file: true, log_file: "other.log" }
```


