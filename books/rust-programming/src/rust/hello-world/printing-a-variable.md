# Printing a variable

One thing that annoys me about `println!` macro is that we cannot just put a variable in it and get it printed. We need to include a formatting string with placeholders for the values or with interpolation as we saw it earlier.

The following code will NOT compile. At least the compiler will give you a semi-understandable hint how to fix it.

{% embed include file="src/examples/intro/formatting-required/src/main.rs" %}

The error message:

{% embed include file="src/examples/intro/formatting-required/out.out" %}

Coming from languages such as Perl and Python I used do way too much debugging by sprinkling `print`-statements around the code.
Even in those languages after a while, as the program grows I always switch to logging, but still.

Given that Rust is a compiled language where the compilation takes a lot longer than in Perl or Python, this strategy is less useful, but still, I'd like my simple printing for debugging back. Luckily Rust provides it.

