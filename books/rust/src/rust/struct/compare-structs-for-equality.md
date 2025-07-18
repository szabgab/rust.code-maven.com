# Compare structs for Equality

* Eq
* PartialEq

* Each data type in Rust either implements Eq or PartialEq to allow users to check if two objects of the same type are equal using either the `==` operator or the `eq` method.
* When creating a struct it does not automatically implement these traits, but we can add them.
* Primitive data types such as `integers` and `strings` implement both Eq and PartialEq.
* `float` on the other hand only implements PartialEq as a float can also be NaN that would break Eq.

* We can add the `Eq` trait to any struct and if all the elements of the struct implement `Eq` then we can add that too:
* It will automatically provide us with the possibility to use `==` or `eq` (or `!=` or `ne` for that matter) on the values of that type.
* However `Eq` is mostly just an indication to the compiler, the actual implementation is in `PartialEq` so we need to add that too.

* In order for two objects of this type to be equal, all the fields have to be equal.

{% embed include file="src/examples/struct/compare-structs-for-equality/src/main.rs" %}

{% embed include file="src/examples/struct/compare-structs-for-equality/out.out" %}


