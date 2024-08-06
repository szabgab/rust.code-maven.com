# Rocket
{id: rocket}

## Rocket web framework
{id: rocket-web-framework}

* The [Rocket](https://rocket.rs/) web framework for Rust.
* [Rocket starter](https://crates.io/crates/rocket-starter) is a small tool to create a project skeleton.
* [Articles about Rocket](https://rust.code-maven.com/rocket) with examples.
* [Discussion](https://github.com/rwf2/Rocket/discussions) about Rocket where you can ask questions.


## Rocket Hello World with separate test file
{id: rocket-hello-world-with-separate-test-file}

* Create a new crate, add rocket as a dependency `cargo add rocket`

![](examples/rocket/hello-world-external-test-file/Cargo.toml)


![](examples/rocket/hello-world-external-test-file/src/main.rs)

```
cargo run
```

The tests

![](examples/rocket/hello-world-external-test-file/src/tests.rs)

```
cargo test
```

```
curl -i http://localhost:8000/
```

## Rocket Hello World returning static RawHtml
{id: rocket-hello-world-static-raw-html}

![](examples/rocket/hello-world-html/Cargo.toml)
![](examples/rocket/hello-world-html/src/main.rs)
![](examples/rocket/hello-world-html/src/tests.rs)

## Rocket: generated RawHtml page
{id: rocket-generated-rawhtml-page}
{i: RawHtml}
{i: epoch}
{i: UNIX_EPOCH}

![](examples/rocket/generated-rawhtml/src/main.rs)

## Rocket Hello World with Tera template
{id: rocket-hello-world-with-tera-template}

![](examples/rocket/hello-world-tera-template/Cargo.toml)
![](examples/rocket/hello-world-tera-template/src/main.rs)
![](examples/rocket/hello-world-tera-template/src/tests.rs)
![](examples/rocket/hello-world-tera-template/templates/index.html.tera)


## Echo using GET
{id: echo-using-get}
{i: GET}

![](examples/rocket/echo-using-get/Cargo.toml)
![](examples/rocket/echo-using-get/src/main.rs)
![](examples/rocket/echo-using-get/src/tests.rs)
![](examples/rocket/echo-using-get/templates/echo.html.tera)
![](examples/rocket/echo-using-get/templates/index.html.tera)

## Echo using POST
{id: echo-using-post}
{i: POST}
{i: Form}
{i: context}
{i: Template}
{i: FromForm}

![](examples/rocket/echo-using-post/Cargo.toml)
![](examples/rocket/echo-using-post/src/main.rs)
![](examples/rocket/echo-using-post/src/tests.rs)
![](examples/rocket/echo-using-post/templates/echo.html.tera)
![](examples/rocket/echo-using-post/templates/index.html.tera)


## Rocket: logging to the console
{id: rocket-logging-to-the-console}
{i: trace!}
{i: debug!}
{i: info!}
{i: warn!}
{i: error!}

![](examples/rocket/logging/src/main.rs)

![](examples/rocket/logging/Rocket.toml)


## Rocket: Calculator with GET (passing multiple parameters)
{id: rocket-calculator-with-get}
{i: RawHtml}

![](examples/rocket/calculator-with-get/src/main.rs)


## Rocket in-memory counter - sessions
{id: rocket-in-memory-counter-sessions}

* [rocket](https://crates.io/crates/rocket)
* [rocket_dyn_templates](https://crates.io/crates/rocket_dyn_templates)
* [tera](https://crates.io/crates/tera)
* [serde](https://crates.io/crates/serde)

![](examples/rocket/in-memory-counter-with-session/Cargo.toml)

![](examples/rocket/in-memory-counter-with-session/src/main.rs)
![](examples/rocket/in-memory-counter-with-session/src/tests.rs)

![](examples/rocket/in-memory-counter-with-session/Rocket.toml)
![](examples/rocket/in-memory-counter-with-session/templates/404.html.tera)
![](examples/rocket/in-memory-counter-with-session/templates/incl/footer.html.tera)
![](examples/rocket/in-memory-counter-with-session/templates/incl/header.html.tera)
![](examples/rocket/in-memory-counter-with-session/templates/index.html.tera)

## Rocket: Automatic reload of the application
{id: rocket-automatic-reload}
{i: watch}

* During development it can be usefule to automatically reload the application as we are making changes to the code.

* [cargo-watch](https://github.com/watchexec/cargo-watch)

```
cargo watch -x run
```


