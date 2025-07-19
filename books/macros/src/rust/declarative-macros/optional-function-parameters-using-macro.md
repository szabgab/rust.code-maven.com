# Optional function parameters using macros

* Rust does not allow the declaration of the same function-name with different signatures as many other languages.
* Rust does not allow the definition of a function with optional parameters and/or with default values.

* We can use declarative macros to overcome this limitation.

* We could define a macro "only for the case with the optional value" or, as we do in this example, we can create a macro for both cases.

{% embed include file="src/examples/macros/optional-parameter/src/main.rs" %}
{% embed include file="src/examples/macros/optional-parameter/out.out" %}


