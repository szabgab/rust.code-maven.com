# Basic TCP echo server

Based on the example in the README of tokio.

Start with:

```
cargo run
```

From another terminal:

```
telnet localhost 8080
```


{% embed include file="src/examples/tokio/basic-tcp-echo-server/src/main.rs" %}

{% embed include file="src/examples/tokio/basic-tcp-echo-server/Cargo.toml" %}
