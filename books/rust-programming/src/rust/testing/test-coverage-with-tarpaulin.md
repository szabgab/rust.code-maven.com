# Test coverage report with tarpaulin

* coverage
* tarpaulin

* [Tarpaulin](https://github.com/xd009642/tarpaulin)

{% embed include file="src/examples/testing/test-coverage/src/lib.rs" %}

```
cargo install cargo-tarpaulin
```

```
cargo tarpaulin
```

Exclude the test functions from the report:

```
cargo tarpaulin --ignore-tests
```

* Generate HTML report

```
cargo tarpaulin --ignore-tests -o Html
```


