# A really mutable string

If we would like to have a string that can really be change we need to make sure or code will allocate some space in the memory where we can really change things.

For this Rust provides a type called [String](https://doc.rust-lang.org/std/string/struct.String.html).

In this example we take a literal string which is going to be embedded in the binary and using the [String::from](https://doc.rust-lang.org/std/string/struct.String.html#method.from-3) function we create a copy of the string in an area of the memory (in the heap) that we can actually change.
We also need to mark the variable to be mutable using the `mut` keyword if we really want to change it.


The [push_str](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str) method will append another string to the one we have.


{% embed include file="src/examples/variables/really-mutable-string/src/main.rs" %}

Output:

{% embed include file="src/examples/variables/really-mutable-string/out.out" %}


---

* String
* from
* push_str


