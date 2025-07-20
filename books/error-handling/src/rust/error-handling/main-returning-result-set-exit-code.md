# main returning Result - Set exit code

* Result
* Box
* dyn
* Error
* Ok
* ?

{% embed include file="src/examples/errors/error-in-main/src/main.rs" %}

A number of invocations:

```
cargo run
cargo run data.txt
cargo run data.txt 10
cargo run data.txt abc
cargo run data.txt 0
```


