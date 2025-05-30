# env_logger with Builder

Using the `env_logger::Builder` we can define the formatting of each log-line.

We can also set the default log-level, if the user does not set one via the `RUST_LOG` environment variable.


{% embed include file="src/examples/logging/env-logger-builder/Cargo.toml" %}

{% embed include file="src/examples/logging/env-logger-builder/src/main.rs" %}

