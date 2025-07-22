# Debugging print

For debugging purposes, however, there is a better solution. Instead of using the `println!' one could use the [dbg! macro](https://doc.rust-lang.org/std/macro.dbg.html).
Not only will it print the content of the variable, it will also print the name of the file, the line number and the name of the variable.

The only drawback, or maybe that is another advantage?) might be that everything is being printed to to the Standard Error channel `STDERR`.

{% embed include file="src/examples/intro/debugging-print/src/main.rs" %}

Output on STDERR:

{% embed include file="src/examples/intro/debugging-print/out.out" %}


---

* dbg!

