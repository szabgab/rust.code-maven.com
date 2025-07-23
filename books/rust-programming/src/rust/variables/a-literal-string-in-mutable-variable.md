# Literal string in a mutable variable can be replaced


In this case too we can use the `mut` keyword to mark a variable as mutable.
This will allow us to replace the string by another string.

{% embed include file="src/examples/variables/mutable-string/src/main.rs" %}

Output:

{% embed include file="src/examples/variables/mutable-string/out.out" %}


## A little about memory

As we start talking about string we encounter the question of memory management. Something we probably have not thought about when writing Perl, Python, PHP or Ruby code
and something that is also different from both C and C++.

If we declare a variable and assign a string in double-quotes to it, that string is going to be compiled into the binary excutable and will remain there.
That means we cannot actually change the content of that string.

In other words, literal strings are embedded in the binary.

We can change the variable to point to another string embedded in the binary, but that does not change the string itself.

