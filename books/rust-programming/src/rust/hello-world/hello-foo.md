# Rust - Hello Foo

We use the `let` keyword to declare a varible. Thouh we don't have to, we can already assign a value to it using a single equal sign `=` as it is done in most programming languages.

In order to print to the sceeen we can use a pair of curly braces `{}` as a placeholder for the content of a variable in the [println! macro](https://doc.rust-lang.org/std/macro.println.html). Behind that macro is another one called the [format! macro](https://doc.rust-lang.org/std/fmt/) where you can read more about the various options.

{% embed include file="src/examples/intro/hello-foo/src/main.rs" %}

The output will look like this:

{% embed include file="src/examples/intro/hello-foo/out.out" %}

---

* let
* println!
* format!

