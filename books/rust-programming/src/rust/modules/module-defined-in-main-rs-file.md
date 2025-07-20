# Module defined in the main.rs file

* mod
* pub

* In order to divide our code into logical units we can use modules.
* First step is to define a module using the `mod` keyword in the `main.rs` file and inside the module we can define functions, structs, enums etc.
* However, in order to be able to call the function from the code outside of the module, we need to make the function public using the `pub` keyword.

* We can access the public function in two different ways.

{% embed include file="src/examples/modules/inline-module/src/main.rs" %}
{% embed include file="src/examples/modules/inline-module/out.out" %}


