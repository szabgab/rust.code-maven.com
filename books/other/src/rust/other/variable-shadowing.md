# Variable shadowing

* let

* We can declare the same variable multiple time using the `let` keyword.
* The 2nd and subsequent declarations hide (shadow) the previous ones.
* If this happened inside a block then when the shadowing version of the variable goes out of scope the previous value becomes visible again. (Observe, the last line being 6.)

{% embed include file="src/examples/other/shadowing/src/main.rs" %}
{% embed include file="src/examples/other/shadowing/out.out" %}


