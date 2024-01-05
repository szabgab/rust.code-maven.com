---
title: Rocket - Single counter in a plain text file
timestamp: 2024-01-05T13:40:01
published: true
description: An example to count the number of visits. Part of the counter example series.
tags:
    - Rocket
    - web
---

This [counter example](https://code-maven.com/counter) is part of the [Rocket](/rocket) series.


## Dependencies

Besides Rocker we also use the [tempdir](https://crates.io/crates/tempdir) crate to create a [temporary directory](/temporary-directory) for our data file.

![](examples/rocket/single-counter-in-text-file/Cargo.toml)


## The main code

The code from the example on the web all set the return value of the route function to be `&'static str`. In order to make this example work I had to change that to `String`.

We use a file called "counter.txt" in the current working directory, which, if we run the application using `cargo run` then is the same as the folder of the crate.
We allow the user (who runs the application)  to override this by setting an environment variable called `COUNTER_PATH`. This will be used in the tests to set the location
of the "counter.txt" in some temporary folder.

![](examples/rocket/single-counter-in-text-file/src/main.rs)


## The test

In the test we create a temporary folder and we place the "counter.txt" file there. This will ensure that running the test does not interfere with the application.
We set the environment variable `COUNTER_PATH` to this path that is consulted in the application itself.

We send two `get` requests to see the counter increasing.

![](examples/rocket/single-counter-in-text-file/src/tests.rs)

