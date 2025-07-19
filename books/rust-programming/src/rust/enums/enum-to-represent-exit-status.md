# Enum to represent exit status

* We define a **type** called `ExitCode` that has two **variants**: `Success` and `Failure`.
* We can then, based on the results of our porgram set the value of exit.

* However, this is a bit limited as we can only indicate failure without the details we are used to - which is a number.

* By default Rust does not implement the `Debug` trait for an arbitrary `enum` so we `derive` from the `Debug` trait to be able to print the values using the `:?` placeholder.
* We can also observe that `if`-statements can have a return value.

{% embed include file="src/examples/enums/exit-status/src/main.rs" %}


