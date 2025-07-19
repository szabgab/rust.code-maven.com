# Compare structs for ordering (sorting) - Ord

* Ord
* PartialOrd

* In order to be able to decide which object is "bigger" or "smaller" than the other one we need the `Ord` trait that requires `PartialOrd` trait and the `Eq` and `PartialEq` traits.
* This will allow use to `sort` the values.
* Comaring the fields happen in the order the appear in the definition of ths struct. In our case Rust forst compares the 'number' fields. The 'name' fields are only compared if the 'number' fields are equal.

{% embed include file="src/examples/struct/compare-structs-for-ordering/src/main.rs" %}
{% embed include file="src/examples/struct/compare-structs-for-ordering/out.out" %}



