# Iterate over both index and value of a vector (enumerate)

* for
* iter
* enumerate

* Instead of getting the index of the current element of Rust, we can either iteratore over the indices or use `enumerate`.
* First example: iterating over the values.
* Second example: iterating over the indices and getting the value. This triggers a `needless_range_loop` suggesting the third solution:
* Third example: Creating an iterator out of the vector and calling `enumerate` on it. This will allow us to iterate over the index-value pairs.

{% embed include file="src/examples/vectors/enumerate/src/main.rs" %}
{% embed include file="src/examples/vectors/enumerate/out.out" %}


