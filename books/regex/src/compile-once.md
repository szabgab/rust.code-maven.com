# Compile once


If we have a function that has a regex in it, then every time we call that function the [regex](https://crates.io/crates/regex) engine has to "compile" the regex to its internal format.
This takes time and it is rather unnecessary waste of time. After all the same regex will compile the same way no matter how many times the function is called.

The multiple compilation can be avoided by using the [once_cell](https://crates.io/crates/once_cell) crate.


## Cargo.toml


```toml
{{#include examples/regex-compile-once/Cargo.toml }}
```

## Code

```rust
{{#include examples/regex-compile-once/src/main.rs }}
```

In the output we can see that it was compiled only once.

```
Compiling regex
Is match: true
Is match: false
```
