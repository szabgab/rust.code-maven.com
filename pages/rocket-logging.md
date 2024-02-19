---
title: Rocket - logging in the web application
timestamp: 2024-01-18T18:50:01
author: szabgab
published: true
description: We can use println and eprintln in the Rocket-based code, but it is much better to use the logging facilities of Rocket. I think.
tags:
    - log
    - logging
    - debug!
    - info!
    - warn!
    - error!
    - Rocket
    - web
---

We can use `println` and `eprintln` in the [Rocket](/rocket)-based code, but it is much better to use the logging facilities of Rocket. I think.


## Dependencies

Nothing fancy. We use Rocket.

{% include file="examples/rocket/logging/Cargo.toml" %}

## The code

This is just a plain [Hello World!](/rocket-hello-world) application with Rocket. I did not even include the tests.

{% include file="examples/rocket/logging/src/main.rs" %}

There are 4 macros we can use right from the `rocket` namespace:

```rust
rocket::debug!
rocket::info!
rocket::warn!
rocket::error!
```

We can run the application with `cargo run` and then when visit the web site at `http://localhost:8000/` we will see the log messages on the console:


![](images/rocket-logging.png)


But oh, we only see the info, warn, and error messages. We don't see the **debug** message.


## Configuring the log level


Rocket allows us to [configure](https://rocket.rs/v0.5/guide/configuration/) many things in the `Rocket.toml` file.

So I created it and added an entry to change the default **normal** level to **debug**.

{% include file="examples/rocket/logging/Rocket.toml" %}

Restarted the application and visited the web-site again.

This time I also saw the "debug" log message, but there was a lot of other output as Rocket printed both the whole `Request` and the whole `Response` struct.

I guess for now I stick with `info!`.

