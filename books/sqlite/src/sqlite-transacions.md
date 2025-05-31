# SQLite Transaction

The default behavior of SQLite is that statement that change the database (e.g. INSERT, UPDATE, DELETE) only prepare the action,
but the user still needs to issue `COMMIT` command to have the statement take effect.

The default connection of the Rust SQLite client uses `autocommit`, meaning that using this crate we don't need issue the `COMMIT`commend. This is easier, however it raises another issue.

What if we would like to have 2 or more statements to be atomit. That is either all of them succeed to all of them fail.
For example if we implement a bank a money transfer from user Mary to use Jane would need 2 statement: deduct the money from
Mary and add the money to Jane. What if there is some error after the deduction (e.g. a panic, or someone reboots the computer, or there is a power failure?) Then the system lost the money that was deducted from Mary.

The solution is to use transactions. A transaction starts with the `BEGIN` statement and ends with a `COMMIT` statement. In between the two you can have any number of actions that would change the database. If the code does not reach the `COMMIT` statement (e.g. because of the said panic, then none of the actions will take place. The money will stay in its original location.

The `BEGIN` statement turns off the autocommit mode and the `COMMIT` turns it on again.

In this example we tried to imitate the issue.

{% embed include file="src/examples/sqlite/transaction/Cargo.toml" %}

{% embed include file="src/examples/sqlite/transaction/src/main.rs" %}


```
$ cargo run -q show
Jane :    0
Mary : 1000
Ann  : 1000
Total: 2000
-----

$ cargo run -q plain
Jane :  100
Mary :  900
Ann  : 1000
Total: 2000
-----

$ cargo run -q panic
thread 'main' panicked at src/main.rs:81:9:
Problem
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo run -q show
Jane :    0
Mary :  900
Ann  : 1000
Total: 1900
-----

$ cargo run -q transaction
BEGIN
thread 'main' panicked at src/main.rs:81:9:
Problem
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo run -q show
Jane :    0
Mary : 1000
Ann  : 1000
Total: 2000
-----
```


