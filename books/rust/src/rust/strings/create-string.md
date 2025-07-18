# Create a String

* println!
* String
* from
* to_string
* to_owned

* The first one is a reference to a string embedded in the program. We can only chnage this in the editor. Not while the program is running.
* `String::from` can create a "real" string from an embedded string. This is also what we get from any input. (STDIN, ARGS, file, network etc.)
* [to_string](https://doc.rust-lang.org/std/string/trait.ToString.html) is the method to stringify any value. It works here as well, but we can be clearer with `to_owned`.
* [to_owned](https://doc.rust-lang.org/std/string/trait.ToOwned.html) convert a reference to an owned string. More about ownership later.

{% embed include file="src/examples/strings/create/src/main.rs" %}
{% embed include file="src/examples/strings/create/out.out)


