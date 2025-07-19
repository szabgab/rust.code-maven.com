# Enumeration and manual comparision

* PartialEq

We can also compare variables holding enum variants, but for that to work we also need to derivede from the [ParialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) trait.
Basically we need to implement the operation that allows use to compare two values of this type.

{% embed include file="src/examples/enums/weekdays-manual-comparision/src/main.rs" %}
{% embed include file="src/examples/enums/weekdays-manual-comparision/out.out" %}


