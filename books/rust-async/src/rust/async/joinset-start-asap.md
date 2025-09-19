# JoinSet scheduling

Earlier it was mentioned that a task starts running when it is added to a JoinSet using the `spawn` function, but I wanted to have an example to really see it.


So in this example we launch to tasks using `spawn` and then we call `sleep` to let some time pass. Looking at the output you can see that faster running task
(the one that uses a 1 second sleep as a fake wait for IO) finished before the first 2-second sleep is over.

Then we sleep another 2 seconds and by that time the task that takes 3 seconds has also finished.

{% embed include file="src/examples/tokio/joinset-scheduling/src/main.rs" %}

## The output

{% embed include file="src/examples/tokio/joinset-scheduling/out.txt" %}

## Cargo.toml

{% embed include file="src/examples/tokio/joinset-scheduling/Cargo.toml" %}

---

* JoinSet
* spawn
* join_all
