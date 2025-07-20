# factorial create panic

In order to avoid this stack overflow our function needs to check the input and then if it is a negative number then report it.
(I know, instead of i64 we could have used u64 which is an unsigned integer that would only allow non-negative numbers. but in other functions we might not be able to use the type-system for that. e.g. what if we expect a positive whole number larger than 2?

One way to avoid reaching stack overflow is to call panic! ourselves.

{% embed include file="src/examples/errors/factorial-create-panic/src/main.rs" %}
{% embed include file="src/examples/errors/factorial-create-panic/out.out" %}


