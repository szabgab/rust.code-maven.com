---
title: Rocket - access custom configuration in the routes
timestamp: 2024-01-22T20:40:01
author: szabgab
published: true
description: It is quite easy to add custom configuration fields to Rocket.toml and access them in the routes.
tags:
    - Rocket
    - info!
    - debug
    - rocket::Config
    - figment
    - extract_inner
todo:
    - how to add other profiles?
    - how to pass configuration values to a test
---

The [Rocket web framework](/rocket) configuration system allows you to override the default values and provide additional keys and values in the `Rocket.toml` file located in the root directory
of the project.

There is a whole page about [Rocket configuration](https://rocket.rs/v0.5/guide/configuration/), At first I could not fully understand it, but I got [help](https://github.com/rwf2/Rocket/discussions/2709).

In this example we'll see how we can access custom configuration values in the routes.

## Dependencies

{% include file="examples/rocket/configuration/Cargo.toml" %}

## The Rocket.toml configuration file

{% include file="examples/rocket/configuration/Rocket.toml" %}


## Logging the configuration value

In this example we use the [logging facility of Rocket](/rocket-logging) we have already seen.

Get the `profile` field of the configuration. This will be `debug` when we run `cargo run` (or cargo test) and it should be `release` when we run `cargo run --release` or `cargo test --release`.
We compare it to the `debug` string, so this should print `true` in debug mode and `false` in release mode. This is how we can check in a given field contains a specific value we are expecting.

```rust
rocket::info!("profile is debug: {:?}", rocket::Config::default().profile == "debug");
```

## Custom configuration fields

Besides the configuration fields that Rocket defined we can add our own fields to `Rocket.toml` and we can access them in the routes.

In the main route you will see the recommended way. I also included a route called `/bad` where I show the "brilliant", but apparently not too good solution I had in the original version of this post.


## The recommended way

1. Creat a `struct` that defines the fields we are expecting with the types of the fields and optionally add default values to each field.
2. Make the `struct` Deserialize-able using `serde`.


We have this, the name of the struct does not matter.

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct MyConfig {
    #[serde(default = "get_default_custom_in_default_section")]
    custom_in_default: String,

    #[serde(default = "get_default_custom_a")]
    custom_a: String,

    #[serde(default = "get_default_custom_b")]
    custom_b: String,
}

fn get_default_custom_in_default_section() -> String {
    String::from("some other default")
}

fn get_default_custom_a() -> String {
    String::from("some default for a")
}

fn get_default_custom_b() -> String {
    String::from("some default for b")
}
```


3. Tell Rocket to include the AdHoc configuration by attaching it:


```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, bad, defaults])
        .attach(AdHoc::config::<MyConfig>())
}
```

4. In the routes where we would like to access the configurations add it to the list of parameters:

```rust
#[get("/")]
fn index(config: &State<MyConfig>) -> &'static str {
```

5. In the route access it via the function parameter


```rust
rocket::info!("custom_a {:?}", config.custom_a);
```


## A bad implementation

Now look a the route handline the `/bad` path:

In the first example we define the type on the variable (and hence we really need a variable here). We also used `unwrap_or` to set a default value in case the field
is not in `Rocket.toml`. There are several other [unwrap-like methods](/unwrap-or) for other ways to handle this situation.

```rust
let custom_a: String = rocket::Config::figment().extract_inner("custom_a").unwrap_or(String::from("some default"));
rocket::info!("home {:?}", custom_a);
```

In this example we used the [Turbofish](/turbofish) syntax to tell Rust what is the expected type of the field.
As we did add this field to the `Rocket.toml` you will see that the value we get is the "some default".

```rust
let custom_b = rocket::Config::figment().extract_inner::<String>("custom_b").unwrap_or(String::from("some default"));
rocket::info!("home {:?}", custom_b);
```

Finally I just wanted to see all the configuration fields in one blob:

```rust
rocket::info!("default: {:#?}", rocket::Config::default());
```

## Running the app seeing the logs

We can now run the app using

```
cargo run
```

or

```
cargo run --release
```

and as we access `http://localhost:8000/` we can see the log messages on the console where we executed the `cargo run ...` command.

We can then also access   `http://localhost:8000/bad` to see the results provided by the "bad" implementation and `http://localhost:8000/defaults` to see the defaults or Rocket.

## Running the test seeing the logs

We don't even need to run the app to see the log messages. We can see them while running the tests.

By default if we run `cargo test` it will hide the log messages generated by Rocket. We can pass the `--nocapture` flag and then it will print all the logs as
it was discussed when we wanted to know [how to show the standard output and standard error in tests in Rust](/show-output-in-tests).

```
cargo test -- --nocapture
```

It is very colorful, but we are interested only what we printed.

The first 3 lines correspond to the first 3 calls to `log::info!`
The first part of this whole output is the cull configurations this corresponds to our first call of `log::info!`.


```
profile is debug: true
custom_a "Hello World!"
custom_b "some default for b"
custom_in_default "hi"
```

The `/defaults` route will print this to the console.



```
default: Config {
    profile: Profile(
        Uncased {
            string: "debug",
        },
    ),
    address: 127.0.0.1,
    port: 8000,
    workers: 16,
    max_blocking: 512,
    ident: Ident(
        Some(
            "Rocket",
        ),
    ),
    ip_header: Some(
        Uncased {
            string: "X-Real-IP",
        },
    ),
    limits: Limits {
        limits: [
            (
                Uncased {
                    string: "bytes",
                },
                ByteUnit(
                    8192,
                ),
            ),
            (
                Uncased {
                    string: "data-form",
                },
                ByteUnit(
                    2097152,
                ),
            ),
            (
                Uncased {
                    string: "file",
                },
                ByteUnit(
                    1048576,
                ),
            ),
            (
                Uncased {
                    string: "form",
                },
                ByteUnit(
                    32768,
                ),
            ),
            (
                Uncased {
                    string: "json",
                },
                ByteUnit(
                    1048576,
                ),
            ),
            (
                Uncased {
                    string: "msgpack",
                },
                ByteUnit(
                    1048576,
                ),
            ),
            (
                Uncased {
                    string: "string",
                },
                ByteUnit(
                    8192,
                ),
            ),
        ],
    },
    temp_dir: RelativePathBuf {
        metadata_path: None,
        path: "/tmp",
    },
    keep_alive: 5,
    shutdown: Shutdown {
        ctrlc: true,
        signals: {
            Term,
        },
        grace: 2,
        mercy: 3,
        force: true,
        __non_exhaustive: (),
    },
    log_level: Normal,
    cli_colors: true,
    __non_exhaustive: (),
}
```

## The code full code

{% include file="examples/rocket/configuration/src/main.rs" %}


