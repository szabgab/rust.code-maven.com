# Manually implement ordering

* PartialOrd
* PartialEq
* partial_cmp
* eq

* In rare case we might want to have an ordering that is not the same as the default implementation of `PartialOrd`. In these cases we can implement it ourselves.
* For this we need to implement both `PartialEq` and `PartialOrd`.
* In our case the functions only take into account the `height` field.
* the `#[allow(dead_code)]` is only needed in this example because we never access the `id` and `name` fields.

{% embed include file="src/examples/struct/implement-ordering/src/main.rs" %}

{% embed include file="src/examples/struct/implement-ordering/out.out" %}



