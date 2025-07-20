# Iterator: all the elements

* all
* iter
* into_iter

* `all` - calls a closure on every element of the iterator and if the closure returns `true` for every element then the expression returns `true`.
* See the documentation of the [all](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all) method.

* `iter` iterates over borrowed references and thus we need to dereference the variables with `*`, but we can continue useing the original vector.
* `into_iter` iterates over the real values and thus we cannot use the original vector any more.

* The last example shows that the iteration will stop when it encounters the first `false`.

{% embed include file="src/examples/iterators/all/src/main.rs" %}
{% embed include file="src/examples/iterators/all/out.out" %}


