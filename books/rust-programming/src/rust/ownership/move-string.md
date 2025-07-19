# Move strings

* We can assign a variable that contains a "mutable string" to another variable
* But this is called a "move" as we move the ownership of the data to the new variable.
* After that we cannot use the old variable any more.
* This will be fun when would like to change one of the variables or if we pass it to a function.

* The variable "owns" the data.
* If we assign the variable to another variable, we pass (move) the ownership.

{% embed include file="src/examples/ownership/move-string/src/main.rs" %}
{% embed include file="src/examples/ownership/move-string/out.out" %}
{% embed include file="src/examples/ownership/move-string/err.out" %}



