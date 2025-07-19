# Rust - flush STDOUT - read STDIN

* print!
* Write
* std::io::Write

* We use `print!` and not `println!`
* We use the [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) trait that includes the `flush` method.

{% embed include file="src/examples/stdin/hello-name-chomp-flush/src/main.rs" %}


