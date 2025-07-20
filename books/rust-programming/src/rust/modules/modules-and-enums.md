# Modules and enums

* Internally in the helper module we can use the `enum`.
* If we return it in a public function (leaking it) we get a compiler warning.
* If we try to use the public function in the outside world, we get a compile error.
* We need to declare the `enum` to be `pub`. Then all its variants also become public.


{% embed include file="src/examples/modules/modules-and-enums/src/main.rs" %}


