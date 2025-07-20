# Pattern matching with guards on a number

Rust does not take in account the guard conditions when checking if the arms cover all the
possible values.
In other words Rust cannot see that our first 3 conditions are exhaustive and that the
last condition will never be reached.
Thus Rust requires the catch-all _ to be present in order to compile the code

{% embed include file="src/examples/other/match-numbers/src/main.rs" %}

