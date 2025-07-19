# Enum to represent exit code

* We can also define `Failure` variant of the `ExitCode` type to have an number - a small number holding a value between 0-255.
* We can use a `match` statement to extract the actual number from the `Failure`.


{% embed include file="src/examples/enums/exit-code/src/main.rs" %}

* Apparently the standard library of Rust uses a struct to represent an [ExitCode](https://doc.rust-lang.org/std/process/struct.ExitCode.html).


