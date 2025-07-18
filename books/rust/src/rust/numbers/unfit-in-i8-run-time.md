# unfit in i8 - run time - overflow

{% embed include file="src/examples/numbers/overflow/src/main.rs" %}

```
cargo run
```

{% embed include file="src/examples/numbers/overflow/out.out" %}

* In debug mode `panic!`


```
cargo run --release
```

{% embed include file="src/examples/numbers/overflow/release.out" %}


