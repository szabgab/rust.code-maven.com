# Derive Debug for struct

* derive
* Debug

* We don't need to implement the `fmt` method of the `Debug` trait ourselves. We can `derive` it:

{% embed include file="src/examples/struct/debug-struct/src/main.rs" %}

{% embed include file="src/examples/struct/debug-struct/out.out" %}



