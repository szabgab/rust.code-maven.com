# Hello World

* fn
* main
* println!

* Rust files must have an extension of `.rs`.
* The main Rust file must have a function called `main`.
* `println!` is a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) (it looks like function, it generates some Rust code during compilation).

{% embed include file="src/examples/intro/hello/src/main.rs" %}

```
rustc src/main.rs
```

```
./main
```


