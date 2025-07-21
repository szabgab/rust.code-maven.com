# Demo None handling

One of the nice features of Rust is the way `None` (null, undefined, etc.) values are handled. Having variables with undefined values is a major issue in any programming language.
In some languages if you declare a variable, but don't initialize it then there is garbage in the variable, in others there is a special symbol called one of these names: None/null/undefined.
Compilers might be happy with this and you will only find yourself in trouble during runt-time when the code tries to access the content of this variable.
If this code is in some conditional, then it might happen rarely and only at an hour extremely inconvenient for your support team.

Let's see how Rust deals with this issue?

In this example the first row is just some instruction to make the linter happy in this contrived example. You can disregard it for now.

In the `main` function We declare a variable called `answer` without assigning any value to it. Later, most likely after several lines of code, we assign a value to it and try to print it.

This code works. There is not problem with this.

{% embed include file="src/examples/introduction/no-null/src/main.rs" %}

However, what would happen if we enabled the `println!` statement before the assignment to `answer`?

## Use variable before initialization?

{% embed include file="src/examples/introduction/no-null-compilation-error/src/main.rs" %}


In this case when Rust compiles the code it will notice that we are trying to use a variable before we initialized it:

{% embed include file="src/examples/introduction/no-null-compilation-error/compile.out" %}
