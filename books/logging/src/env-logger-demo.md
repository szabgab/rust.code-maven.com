# Simple use of env_logger

{% embed include file="src/examples/logging/env-logger-demo/Cargo.toml" %}

{% embed include file="src/examples/logging/env-logger-demo/src/main.rs" %}


By default this sets the log-level to `ERROR` so if we run the code we get a single log:


```
$ cargo run

[2025-04-26T04:59:00Z ERROR env_logger_demo] This is a sample error.
```

The user can control the level of the logging by setting the `RUST_LOG` environment variable.

On Linux and macOS you can do it this way:

```
$ RUST_LOG=trace cargo run

[2025-04-26T04:59:57Z TRACE env_logger_demo] This is a sample trace.
[2025-04-26T04:59:57Z DEBUG env_logger_demo] This is a sample debug.
[2025-04-26T04:59:57Z INFO  env_logger_demo] This is a sample info.
[2025-04-26T04:59:57Z WARN  env_logger_demo] This is a sample warn.
[2025-04-26T04:59:57Z ERROR env_logger_demo] This is a sample error.
```

In Windows in CMD

```
> set RUST_LOG=trace
```

In PowerShell

```
> [Environment]::SetEnvironmentVariable("RUST_LOG", "trace", "Process")
```



