# From and Into for String and &str

* &str
* String
* from
* into

* We can create a `String` from an `&str` by using the `String::from()` method because `String` implements the From trait for `&str`.
* We can also use the `into` method, but then we must tell Rust the expected type.
* For some reason we cannot use the Turbofish syntax.

{% embed include file="src/examples/struct/from-str-into-string/src/main.rs" %}


