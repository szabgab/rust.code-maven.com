# Clap - set default value if other flag provided

* default_value_if
* ArgPredicate
* Equals
* OsStr

* If `log_to_file` is true then `log_file` will get a default value.

* Actually I am not sure this is such a good example for `default_value_if` as setting the default even if the `log_to_file` is `False` would not be a problem. The code just would not use it.

* [ArgPredicate](https://docs.rs/clap/latest/clap/builder/enum.ArgPredicate.html)
* [default_value_if](https://docs.rs/clap/latest/clap/struct.Arg.html#method.default_value_if)

{% embed include file="src/examples/clap/default-value-if-equals/src/main.rs" %}

```
$ cargo run -q
Args: Cli { log_to_file: false, log_file: None }

$ cargo run -q -- --log-to-file
Args: Cli { log_to_file: true, log_file: Some("my.log") }

$ cargo run -q -- --log-to-file --log-file other.log
Args: Cli { log_to_file: true, log_file: Some("other.log") }

$ cargo run -q -- --log-file other.log
Args: Cli { log_to_file: false, log_file: Some("other.log") }
```


