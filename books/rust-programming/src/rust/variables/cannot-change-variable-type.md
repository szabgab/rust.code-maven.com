# Cannot change the type of a variable

So far when declared variables we have not declared the types of the variables, but behind the scenes Rust maintaines a type-system
and enforces it. When we first assign a value to a variable - usually when we declare the variable - Rust will infere the type of the
variable from the value we assigned to it.

Then when we try to change the value Rust will make sure we can only change it to a value of the same type.

In this example when we declared the variable `answer` we assigned a (literal) string to it.

If late we try to assign a number, Rust will stop us with a compilation error.

{% embed include file="src/examples/variables/cannot-change-type/src/main.rs" %}

The compilation error:

{% embed include file="src/examples/variables/cannot-change-type/out.out" %}


