# Rust - read STDIN - remove trailing newline (trim, chomp)

* trim_end
* to_owned

* [trim_end](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_end) removes trailing whitespace.
* `to_owned` Converts the `&str` to `String` to be able to assign to the `name` variable again.

{% embed include file="src/examples/stdin/hello-name-chomp/src/main.rs" %}


