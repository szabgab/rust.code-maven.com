# How to find code that might overflow / underflow?

* [arithmetic_side_effects](https://rust-lang.github.io/rust-clippy/master/index.html#/arithmetic_side_effects)

```
cargo clippy -- --deny clippy::arithmetic_side_effects
```

```
error: arithmetic operation that can potentially result in unexpected side-effects
```


* See also: [overflow_check_conditional](https://rust-lang.github.io/rust-clippy/master/index.html#/overflow_check_conditional)
* and [implicit_saturating_add](https://rust-lang.github.io/rust-clippy/master/index.html#/implicit_saturating_add)


