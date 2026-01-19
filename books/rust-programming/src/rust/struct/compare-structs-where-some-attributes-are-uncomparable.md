# Compare structs where some attributes are not comparable


We can compare primitive values just as numbers using the standard comparison operators.
For a custom struct, we need to derive the `PartialEq` and `PartialOrd` traits to enable comparison.

We cannot compare some of the more complex types such as HashMaps or Vectors directly using these operators.
For those types, we would need to implement custom comparison logic.

Also if we have a struct with fields that do not implement `PartialEq` or `PartialOrd`, we cannot derive these traits for the struct, but we can implement them manually if needed.


{% embed include file="src/examples/struct/order-struct/src/main.rs" %}

