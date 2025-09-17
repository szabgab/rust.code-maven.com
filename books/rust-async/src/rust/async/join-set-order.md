# JoinSet random order of completition

We can combine several tasks in a [JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)
and then we can calle [join_all](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.join_all) to wait for all of them to finish
that might be in any order.


{% embed include file="src/examples/tokio/join-set-order/src/main.rs" %}

{% embed include file="src/examples/tokio/join-set-order/out.txt" %}

{% embed include file="src/examples/tokio/join-set-order/Cargo.toml" %}
