# todo! for a match

* todo!

{% embed include file="src/examples/macros/todo-match/src/main.rs" %}

```
$ cargo run -q foo
We are handling foo

$ cargo run -q bar
We are handling bar


$ cargo run -q qqrq
thread 'main' panicked at src/main.rs:15:17:
not yet implemented: We still need to implement for qqrq
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


