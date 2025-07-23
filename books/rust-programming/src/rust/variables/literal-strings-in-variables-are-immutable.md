# Literal strings in variables are immutable

Similar to numbers, if we declare a variable and assign a string to it as in the following example, that variable is going to be immutable.

{% embed include file="src/examples/variables/immutable-string/src/main.rs" %}

We get the following compilation error:

{% embed include file="src/examples/variables/immutable-string/out.out" %}

What happens if we make the variable mutable?

