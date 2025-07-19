# Command line program with a single file parameter

* eprintln!
* exit
* std::process::exit

{% embed include file="src/examples/args/some-tool/src/main.rs" %}

```
cargo run

Usage: target/debug/some-tool FILENAME
```

```
cargo run data.csv

We are working on file 'data.csv'
```


