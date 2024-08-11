# Logging
{id: logging}

## Simple Logger new 
{id: simple-logger-new}
{i: log}
{i: simple_logger}

![](examples/logging/simple_logger_new/Cargo.toml)

![](examples/logging/simple_logger_new/src/main.rs)

![](examples/logging/simple_logger_new/out.out)


## Simple Logger init
{id: simple-logger-init}

![](examples/logging/simple_logger_init/src/main.rs)

![](examples/logging/simple_logger_init/out.out)

## Simple Logger level
{id: simple-logger-level}

![](examples/logging/simple_logger_init_with_level/src/main.rs)

![](examples/logging/simple_logger_init_with_level/out.out)


## Logging with log4rs
{id: logging-with-log4rs}
{i: log}
{i: log4rs}

* [log4rs](https://crates.io/crates/log4rs)

Dependencies

![](examples/logging/try-log4rs/Cargo.toml)

The code:

![](examples/logging/try-log4rs/src/main.rs)

The configuration

![](examples/logging/try-log4rs/log4rs.yaml)

* [configuration](https://docs.rs/log4rs/latest/log4rs/index.html)

* [encoders](https://docs.rs/log4rs/latest/log4rs/encode/index.html)
* [patterns](https://docs.rs/log4rs/latest/log4rs/encode/pattern/index.html) for the encoders.

Standard output:

![](examples/logging/try-log4rs/out.out)

The `all.log` file:

![](examples/logging/try-log4rs/all.log)



