# Logging with log4rs

* log
* log4rs

* [log4rs](https://crates.io/crates/log4rs)

Dependencies

{% embed include file="src/examples/logging/try-log4rs/Cargo.toml" %}

The code:

{% embed include file="src/examples/logging/try-log4rs/src/main.rs" %}

The configuration

{% embed include file="src/examples/logging/try-log4rs/log4rs.yaml" %}

* [configuration](https://docs.rs/log4rs/latest/log4rs/index.html)

* [encoders](https://docs.rs/log4rs/latest/log4rs/encode/index.html)
* [patterns](https://docs.rs/log4rs/latest/log4rs/encode/pattern/index.html) for the encoders.

Standard output:

{% embed include file="src/examples/logging/try-log4rs/out.out" %}

The `all.log` file:

{% embed include file="src/examples/logging/try-log4rs/all.log" %}



