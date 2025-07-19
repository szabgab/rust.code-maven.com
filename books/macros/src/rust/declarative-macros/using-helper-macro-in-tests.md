# Using a helper macro vs helper function in tests

* If we use helper functions then the assert failure will be reported in the function and it will be harder for us to know which call to that function triggered the error.
* If we use macro, then we get the line where the macro is called.

* Run `cargo test` to see the difference in error reporting.

{% embed include file="src/examples/macros/test-failure-report/src/lib.rs" %}
{% embed include file="src/examples/macros/test-failure-report/out.out" %}


