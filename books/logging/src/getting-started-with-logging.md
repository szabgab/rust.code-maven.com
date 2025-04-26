# Getting started with logging in Rust

Mastering logging can help you save a lot of time both during development and later when there are problems in production.

- log
- trace!
- debug!
- info!
- warn!
- error!
- simple_logger
- init_with_env

One of the ancient debugging techniques is adding print-statements to the code, running the code and observing which part of the code is
executed and what are the values of various variables. The big problem with this is that once the issue was fixed we need to remove, or
at least comment-out the print statements.

Then, when we encounter a new issue we will have to add back or re-enable the print-statements. A lot of waste of time.

In addition, we don't have any of the print statements while the code is in production.

## What is Logging

Logging is a simple technique that helps solving this problem. Basically we add conditional print-statements to the code that we can
then enable or disable externally. It goes beyond that simple on/off. In logging we can also control the **verbosity** of the program.
In logging have **levels** and we can control which levels are printed. So we can have **debug!** messages that contain a lot of very detailed information.
We can have **info!** messages with more high-level information. We can also have **warn!** or **error!** messages that would give warnings or
report errors.

The levels are in this order:

* error!
* warn!
* info!
* debug!
* trace!

Besides controlling which level of messages to print, in logging we can also control where to print them. On the screen? In a file? Into a database? etc.

## Add log messages

The logging system in Rust has two major parts. One part is implemented in the [log](https://crates.io/crates/log) crate.
It provides the macros that you can use to embed the logging statements in your code. For this you need to add the **log crate**
to your code either by running

```
cargo add log
```

or by editing the **Cargo.toml** file and adding the dependency manually:

```
[dependencies]
log = "0.4"
```

Of course by the time you read this, there might be a newer version of the crate, so set the number accordingly.

Then we can add the logging calls to our code in the `src/main.rs` file.

We can either use the full name of the macros:


```rust
log::trace!("This is a sample trace.");
log::debug!("This is a sample debug.");
log::info!("This is a sample info.");
log::warn!("This is a sample warn.");
log::error!("This is a sample error.");
```

Or we can import the necessary macro:

```rust
use log::error;
```

Or the necessary macros, if we need more of them:

```rust
use log::{debug, info};
```

Then we can use the short name of the macros that we have imported:

```
debug!("Another debug.");
info!("Another info.");
error!("Another error.");
```

Now we can run our program

```
cargo run
```

and we will see that "nothing happens". More precisely, we'll see the "Hello, world!" on the screen,
but none of the logging statements.

That's because we still need to add a logger implementation. Several of these implementations are mentioned
on the page about the [log](https://crates.io/crates/log) crate.

## The code with log only

{% embed include file="src/examples/logging/log-demo/Cargo.toml" %}

{% embed include file="src/examples/logging/log-demo/src/main.rs" %}


## Simple logger

Add the [simple_logger](https://crates.io/crates/simple_logger) by running:

```
cargo add simple_logger
```

or by manually editing the `Cargo.toml` file to include both `log` and `simple_logger`.


```
[dependencies]
log = "0.4"
simple_logger = "4.3"
```

Also edit the `src/main.rs` file and add

```rust
simple_logger::SimpleLogger::new().init().unwrap();
```

as the first statement of the `main` function.

If we run

```
cargo run
```

again we get the following output:

```
Hello, world!
2023-12-04T07:42:02.858Z TRACE [simple_log_demo] This is a sample trace.
2023-12-04T07:42:02.858Z DEBUG [simple_log_demo] This is a sample debug.
2023-12-04T07:42:02.858Z INFO  [simple_log_demo] This is a sample info.
2023-12-04T07:42:02.858Z WARN  [simple_log_demo] This is a sample warn.
2023-12-04T07:42:02.858Z ERROR [simple_log_demo] This is a sample error.
2023-12-04T07:42:02.858Z DEBUG [simple_log_demo] Another debug.
2023-12-04T07:42:02.858Z INFO  [simple_log_demo] Another info.
2023-12-04T07:42:02.858Z ERROR [simple_log_demo] Another error.
```

In each line we can see the time of logging, the logging level in capital letters (TRACE, DEBUG, etc.)
then the name of our crate (I used a crate called `simple_log_demo` for this example) and then the actual
logging message.

If your terminal supports colors then each one of the capitalized logging levels will have a different color.

By default all the lines are printed to Standard Output (STDOUT) together with the regular output of the code.
This is usually not very good as it does not allow the user to separately redirect the regular output
and the logging messages, but this is the default.

## Change the output to STDERR

We can enable the **stderr** feature by changing the Cargo.toml to include

```
simple_logger = { version = "4.3", features = ["stderr"] }
```

Running `cargo run` we won't see a difference, but we can now redirect the logging to a file:

```
cargo run 2> log.txt
```

This will only print "Hello, world!". All the logging messages were saved in the "log.txt" file.


## Set the minimum logging level

In the `src/main.rs` file we can replace the code that initialized the **simple_logger** by this line:

```rust
simple_logger::init_with_level(log::Level::Warn).unwrap();
```

This too will initialized the simple_logger, but will set the minimum logging level to be **warnings**.
This means that every log message of warning and higher (that is **error**) will be printed.
The lower level messages won't.

This is what we'll see:

```
Hello, world!
2023-12-04T08:04:53.482Z WARN  [simple_log_demo] This is a sample warn.
2023-12-04T08:04:53.482Z ERROR [simple_log_demo] This is a sample error.
2023-12-04T08:04:53.482Z ERROR [simple_log_demo] Another error.
```

Setting the minimum level hard-coded in the code is not really ideal. Luckily there are alternatives:

## Set the minimum using environment variables

Simple logger provides another way to initialize the logging level using an environment variable.

For this we need to replace the line that initializes the logger with this line:


```rust
simple_logger::init_with_env().unwrap();
```

Then we can set the minimum level using the `RUST_LOG` environment variable.

On **Linux** and **macOS** this would set the minimum log level to warning:

```
RUST_LOG=warn cargo run
```

On Windows you need to write this:

```
set RUST_LOG=warn
cargo run
```


## Set minimum level as a command line option

For some reason setting the minimum level with an environment variable feels strange.
I think this is especially the case when the application starts to have a number of command
line parameters. Then it feels more natural to provide the log level too as a command line
parameter. So I add a little function that can help me do this.

The initialization code now looks like this:

```rust
simple_logger::init_with_level(get_log_level()).unwrap();
```

and the `get_log_level` function looks like this:

```rust
use std::str::FromStr;

fn get_log_level() -> log::Level {
    log::Level::from_str(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .unwrap_or(&String::from("trace"))
            .to_lowercase()
            .as_str(),
    )
    .unwrap_or(log::Level::Trace)
}
```

Now we can run the code using

```
cargo run
```

and get trace-level logging or we can run using


```
cargo run warn
```

and we get warnings and errors.


How to implement something similar with [clap](https://crates.io/crates/clap) is left as an exercise for the reader.

## The code with several cases

{% embed include file="src/examples/logging/simple-log-demo/Cargo.toml" %}

{% embed include file="src/examples/logging/simple-log-demo/src/main.rs" %}

## Conclusion

There are tons of other things we need to know in order to take full advantage of logging, but this can already get you started.

A few things I'll need to address:

* How to allow the user to save the log messages to a file - without the need for redirection?
* How to focus the logging on a given crate? Many of the crates we use might have log messages, but when we focus on our code we probably only want to see the log messages from our crate. Or maybe we would want to have trace-level logging from our crate, and warn-level logging from other crates.
* How to change the format of the log messages? For example to include the filename and the line number where the log was created.

