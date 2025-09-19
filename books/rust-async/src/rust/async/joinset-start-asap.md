# JoinSet scheduling

Let's show that each async job starts as soon as we add it to the JoinSet.

{% embed include file="src/examples/tokio/joinset-scheduling/src/main.rs" %}

{% embed include file="src/examples/tokio/joinset-scheduling/out.txt" %}

{% embed include file="src/examples/tokio/joinset-scheduling/Cargo.toml" %}
