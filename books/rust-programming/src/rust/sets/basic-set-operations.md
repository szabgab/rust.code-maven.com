# Basic Set operations in Rust

* set
* HashSet
* insert
* remove
* contains

* A [HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html) can be used for the mathematical SET operations.
* We can `insert` values. The HashSet will only contain one copy of each value regardless the number of times we `insert` it.
* We can `remove` values.
* We can get the number of elements in the set using `len`.
* We can check if a set `contains` a certain value.
* There is no order among the elements so when we print them they might be in any order.

{% embed include file="src/examples/sets/basic-set-operations/src/main.rs" %}
{% embed include file="src/examples/sets/basic-set-operations/out.out" %}


