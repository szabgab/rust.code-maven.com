# Handle variants of an enum

We have en `enum` that has 2 or more variants probably with differen attributes. We would like to be able to display the each variant in its own way.

Ideally we would probably be able to use a `match` inside the templates.

This is a solution that might be overengineered.


{% embed include file="src/examples/liquid/show-enum/src/main.rs" %}

