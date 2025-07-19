# Empty string and zero as default values

* derivable_impls

* In this special case the default values we set in the implementation of the Default trait happen to be the default values of each primitive type. (empty string for strings and 0 for numbers).
* In this case **clippy** will indicate that we don't need to implement Default by ourselves.
* In this example we silences clippy, in the next example we derive from Default.

{% embed include file="src/examples/struct/default/src/main.rs" %}


