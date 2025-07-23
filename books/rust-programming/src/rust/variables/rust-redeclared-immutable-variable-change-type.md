# Redeclare immutable variable and change type - Shadowing

The fact that we cannot change the type of a variable is probbaly obvious to people who are arriving to Rust from C, C++, Java, or other strict-typed languages, but it is
rather annoying for people who arrive from the dynamically typed languags such as Perl or Python.

There is at least one use-case where it seems natural to want to assign a different type to the same variable name.

When we get an input from the outside world (keyboard, file, network, etc.) at the low level it is always a string.
What if we are expecting a number? We need to first get the input. Store it in a variable. Then convert it to a number and store it in....
and at that point we need to come with another variable name which is basically the same as the previous one.

In Rust there is a feature called "shadowing" when we decalre a variable a second time. We effectively hide the first variable from view.
It does not get destroyed, it just becomes inaccesible while the new, the shadowing variable is in effect.

This can be useful if you need to make a few changes and then later no more changes.

As we are not really chaning the variable, we don't even need to make it mutable.


In this example we declare the variable `answer` 3 times using the `let` keyword. The first two times we assign strings to it the 3rd time it is a number:
In order to demonstrate it is really a number I even added 1 to it.


{% embed include file="src/examples/variables/change-type/src/main.rs" %}

Rust is happy to play along:

{% embed include file="src/examples/variables/change-type/out.out" %}


