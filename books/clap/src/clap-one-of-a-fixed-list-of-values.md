# Clap: one of a fixed list of values (enumerated)

* enum
* ValueEnum

Using Clap we can define a parameter to accept only values from a fixed list of possible values.
For this we create an `enum` where all variants of the enum correspond to the values we would like to accept.
The `enum` has to derive from `clap::ValueEnum` and also from `Clone` and `Copy`.

We can also use the `alias` to allow alternate values to be mapped onto specific variants of the enum.


{% embed include file="src/examples/clap/enumerated/src/main.rs" %}

```
cargo run -q -- --color blue
cargo run -q -- --color red

cargo run -q -- --color green
cargo run -q -- --color Green
cargo run -q -- --color verde
```



