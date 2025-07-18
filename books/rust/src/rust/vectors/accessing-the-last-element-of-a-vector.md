# Accessing the last element of a vector

* len
* last
* TODO

* Unlike Python and Perl, rust won't let us use a negative index in a vector so we won't be able to access the last element using `vector_name[-1]`
* We can either use `vector_name.len()-1` or
* We can use `vector_name.last()`, but in this case we get an `Option` that can be `None` as well.

* If we access a seemingly arbitrary element that we calculated using `vector_name.len()-1` then either we get back a value or Rust will panic if we gave an index too big.
* On the other hand using `last` we are more protected. In that case we either get a value or `None` if the vector was empty.

{% embed include file="src/examples/vectors/last-vec-index/src/main.rs" %}
{% embed include file="src/examples/vectors/last-vec-index/out.out" %}


