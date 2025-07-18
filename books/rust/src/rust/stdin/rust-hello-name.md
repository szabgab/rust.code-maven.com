# Rust - read input from STDIN


* std::io
* String
* stdin
* read_line
* expect

* Module [std::io](https://doc.rust-lang.org/std/io/)
* [String::new()](https://doc.rust-lang.org/std/string/struct.String.html#method.new) - Creates a new mutable empty string
* `&mut name` passes a reference of the variable to the function as mutable variable. `&` indicates it is a reference.
* `read_line` reads a line from the command line

{% embed include file="src/examples/stdin/hello-name/src/main.rs" %}

{% embed include file="src/examples/stdin/hello-name/out.out" %}

Two problems:
* The response looks broken. It has a newline after the name.
* After the prompt the program waits on a new line.


