# Rust and print

A little overview of the built-in print-like macros:

* [print!](https://doc.rust-lang.org/std/macro.print.html)    to STDOUT.
* [println!](https://doc.rust-lang.org/std/macro.println.html)  to STDOUT with a trailing newline.
* [eprint!](https://doc.rust-lang.org/std/macro.eprint.html)   to STDERR.
* [eprintln!](https://doc.rust-lang.org/std/macro.eprintln.html) to STDERR with a trailing newline.
* [dbg!](https://doc.rust-lang.org/std/macro.dbg.html)      to STDERR with bells and whistles.
* [format!](https://doc.rust-lang.org/std/macro.format.html)  returns formatted string. This is behind the printing macros.

{% embed include file="src/examples/intro/print/src/main.rs" %}

Output:

{% embed include file="src/examples/intro/print/out.out" %}


---

* print!
* println!
* eprint!
* eprintln!
* dbg!
* format!



