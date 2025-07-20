# factorial function, runtime panic

* panic!

* In this series of examples we'll see how a function we write can report error and how the caller can handle the error.
* In the first example we implement a function to calculate factorial recursively. The specific task is not important, just that if the user supplies a negative number then this code will crash. More precisely, it will have a panic when our program reaches stack overflow.

{% embed include file="src/examples/errors/factorial/src/main.rs" %}
{% embed include file="src/examples/errors/factorial/out.out" %}



