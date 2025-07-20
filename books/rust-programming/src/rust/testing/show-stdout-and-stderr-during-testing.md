# Show STDOUT and STDERR during testing

* nocapture

In this example there are print-statements both in the code and in the test function.

{% embed include file="src/examples/testing/nocapture/src/lib.rs" %}

If we run `cargo test` we don't see any of this as the tester captures them.

If we run `cargo test -- --nocapture` then we'll see the output of all the 4 print-statements.


