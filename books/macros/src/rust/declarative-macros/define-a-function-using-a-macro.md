# Define a function using a macro


* We can use both `tt` and `ident` in the macro parameter definition.
* We can use the macro both inside and outside functions.
* The former will create functions scoped to the block where they were created. The latter will be global.

{% embed include file="src/examples/macros/define-function/src/main.rs" %}


