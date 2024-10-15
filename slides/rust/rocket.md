# Rocket
{id: rocket}

## Rocket - A web framework for Rust
{id: rocket-web-framework}

* The [Rocket](https://rocket.rs/) web framework for Rust.
* [Rocket starter](https://crates.io/crates/rocket-starter) is a small tool to create a project skeleton.
* [Articles about Rocket](https://rust.code-maven.com/rocket) with examples.
* [Discussion](https://github.com/rwf2/Rocket/discussions) about Rocket where you can ask questions.

* TODO: return JSON (in an API)
* TODO: log into a logfile
* TODO: Blog engine, map the path to an entry in the database, what if that entry does not exist in the database? How do we return 404 not found. (either return Template or a 404 not found page)

## Rocket - Hello World
{id: rocket-hello-world}

```
cargo add rocket
```

![](examples/rocket/hello-world/Cargo.toml)

![](examples/rocket/hello-world/src/main.rs)

```
cargo test
```

```
cargo run
```

```
$ curl -i http://127.0.0.1:8000/
```

* Content-type is `text/plain`


## Rocket - Hello World with separate test file
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

## Rocket - Hello World returning static RawHtml
{id: rocket-hello-world-static-raw-html}
{i: content}
{i: RawHtml}


![](examples/rocket/hello-world-html/Cargo.toml)
![](examples/rocket/hello-world-html/src/main.rs)
![](examples/rocket/hello-world-html/src/tests.rs)

```
curl -i http://localhost:8000
```

* content-type: `text/html`

## Rocket - Generated RawHtml page
{id: rocket-generated-rawhtml-page}
{i: RawHtml}
{i: epoch}
{i: UNIX_EPOCH}

![](examples/rocket/generated-rawhtml/src/main.rs)

## Rocket - Hello World with Tera template
{id: rocket-hello-world-with-tera-template}
{i: Tera}

![](examples/rocket/hello-world-tera-template/Cargo.toml)
![](examples/rocket/hello-world-tera-template/src/main.rs)
![](examples/rocket/hello-world-tera-template/src/tests.rs)
![](examples/rocket/hello-world-tera-template/templates/index.html.tera)


## Rocket - Echo using GET
{id: rocket-echo-using-get}
{i: GET}
{i: Tera}

![](examples/rocket/echo-using-get/Cargo.toml)
![](examples/rocket/echo-using-get/src/main.rs)
![](examples/rocket/echo-using-get/src/tests.rs)
![](examples/rocket/echo-using-get/templates/echo.html.tera)
![](examples/rocket/echo-using-get/templates/index.html.tera)

## Rocket - Echo using POST
{id: rocket-echo-using-post}
{i: POST}
{i: Form}
{i: context}
{i: Template}
{i: FromForm}
{i: Tera}

![](examples/rocket/echo-using-post/Cargo.toml)
![](examples/rocket/echo-using-post/src/main.rs)
![](examples/rocket/echo-using-post/src/tests.rs)
![](examples/rocket/echo-using-post/templates/echo.html.tera)
![](examples/rocket/echo-using-post/templates/index.html.tera)

## Rocket - Path parameters
{id: rocket-path-parameters}

Instead of passing parameters in the Query string in a GET request we can also use the path to pass parameters.
This is especially interesgint if we would like to make the pages indexable by search engines.

* e.g. in a blog engine the path can be mapped to a blog entry
* In a social site we might want to have a separate page for each users.

![](examples/rocket/path-parameters/src/main.rs)
![](examples/rocket/path-parameters/src/tests.rs)

## Rocket - Single hit-counter using a text file
{id: rocket-single-hit-counter-using-a-text-file}

![](examples/rocket/single-counter-in-text-file/Cargo.toml)
![](examples/rocket/single-counter-in-text-file/src/main.rs)
![](examples/rocket/single-counter-in-text-file/src/tests.rs)

* Error handling - `unwrap`.
* File operations are not atomic.
* We don't handle variable overflow properly.


## Rocket - Logging to the console
{id: rocket-logging-to-the-console}
{i: trace!}
{i: debug!}
{i: info!}
{i: warn!}
{i: error!}

![](examples/rocket/logging/src/main.rs)

![](examples/rocket/logging/Rocket.toml)

## Rocket - Logging to a file using log4rs
{id: rocket-logging-to-a-file-using-log4rs}

* Add [log4rs](https://crates.io/crates/log4rs) to the dependencies.

![](examples/rocket/logging-with-log4rs-to-file/Cargo.toml)

* Create a configuration file:

![](examples/rocket/logging-with-log4rs-to-file/log4rs.yaml)

* Initiate the logging

![](examples/rocket/logging-with-log4rs-to-file/src/main.rs)


## Rocket - Calculator with GET (passing multiple parameters)
{id: rocket-calculator-with-get}
{i: RawHtml}

![](examples/rocket/calculator-with-get/src/main.rs)

## Rocket - In-memory counter using State
{id: rocket-in-memory-counter}

* A single counter in-memory counter (multiple browsers share the counter)

* [rocket](https://crates.io/crates/rocket)
* [rocket_dyn_templates](https://crates.io/crates/rocket_dyn_templates)
* [tera](https://crates.io/crates/tera)
* [serde](https://crates.io/crates/serde)

![](examples/rocket/in-memory-counter/Cargo.toml)

![](examples/rocket/in-memory-counter/src/main.rs)
![](examples/rocket/in-memory-counter/src/tests.rs)

![](examples/rocket/in-memory-counter/Rocket.toml)
![](examples/rocket/in-memory-counter/templates/404.html.tera)
![](examples/rocket/in-memory-counter/templates/incl/footer.html.tera)
![](examples/rocket/in-memory-counter/templates/incl/header.html.tera)
![](examples/rocket/in-memory-counter/templates/index.html.tera)

## Rocket - get, set (add), delete cookies - pending cookies
{id: rocket-get-set-delete-cookies}

![](examples/rocket/set-cookie/src/main.rs)

## Rocket - Multi-counter using cookies (in the client)
{id: rocket-multi-counter-using-cookies}

![](examples/rocket/multi-counter-using-cookies/src/main.rs)
![](examples/rocket/multi-counter-using-cookies/src/tests.rs)

## Rocket - Multi-counter using secure cookies (in the client)
{id: rocket-multi-counter-using-secure-cookies}

![](examples/rocket/multi-counter-using-encrypted-cookies/Cargo.toml)
![](examples/rocket/multi-counter-using-encrypted-cookies/Rocket.toml)
![](examples/rocket/multi-counter-using-encrypted-cookies/src/main.rs)
![](examples/rocket/multi-counter-using-encrypted-cookies/src/tests.rs)

## Rocket - Automatic reload of the application (watch)
{id: rocket-automatic-reload}
{i: watch}

* During development it can be usefule to automatically reload the application as we are making changes to the code.

* [cargo-watch](https://github.com/watchexec/cargo-watch)

```
cargo watch -x run
```

## Rocket - Two applications in separate files
{id: rocket-separate-files}
{i: TBD}

* We created a separate file with its own routes
* We then mounted it under a path called /blog
* We provide a function called `routes` listing all the routes in this applcation and use that in the `mount`.

Limitation of this solution:

* in the `blog_test` we need to use `super::super::rocket()` instead of `super::rocket()`.
* in the `blog_test` we need to access `/blog` that mean we need to know where it will be mounted.

![](examples/rocket/separate-files/src/main.rs)
![](examples/rocket/separate-files/src/tests.rs)

![](examples/rocket/separate-files/src/blog.rs)
![](examples/rocket/separate-files/src/blog/blog_tests.rs)

## Rocket - Redirect to another page
{id: rocket-redirect-to-another-page}
{i: Redirect}
{i: uri!}
{i: Status}
{i: SeeOther}
{i: TBD}

* TODO: Implement the same in the separate files case.
* TODO: Redirect using other Status code
* TODO: Redirect to an external URL
* TODO: Optional redirction? (e.g. after successful login we redirect, but if it fails we would like to show the login page)
* TODO: Dynamic redirect. (e.g. after successful login we go to the page where the user really wanted to go)

![](examples/rocket/redirect-to-fixed-url/src/main.rs)
![](examples/rocket/redirect-to-fixed-url/src/tests.rs)


## Rocket - Redirect with parameters
{id: rocket-redirect-with-parameters}

![](examples/rocket/redirect-with-parameters/src/main.rs)
![](examples/rocket/redirect-with-parameters/src/tests.rs)

## Rocket - Serving static files
{id: rocket-serving-static-files}
{i: relative}
{i: FileServer}

* We can use the FileServer to return static files such as images, css files, javascript files etc.
* We need to mount it to "/".
* It sets the content type properly for each file.

![](examples/rocket/static-files/src/main.rs)

![](examples/rocket/static-files/src/tests.rs)

![](examples/rocket/static-files/static/css/style.css)
![](examples/rocket/static-files/static/js/demo.js)

## Rocket - 404 page with static content
{id: rocket-404-page-with-static-content}

![](examples/rocket/http-404-page-with-static-content/src/main.rs)
![](examples/rocket/http-404-page-with-static-content/src/templates/404.html)
![](examples/rocket/http-404-page-with-static-content/src/tests.rs)

## Rocket - Access custom configuration in the routes
{id: rocket-access-custom-configuration-in-the-routes}

![](examples/rocket/configuration/Rocket.toml)

![](examples/rocket/configuration/Cargo.toml)

![](examples/rocket/configuration/src/main.rs)


## Rocket - Custom configuration and injecting (overriding) values in tests
{id: custom-configuration-and-injecting-values-in-tests}

* We would like to have some custom configuration values for the application. (e.g. database address, API key for email sending service, folder to save uploaded imagesetc.)
* During testing we would like to set these values separately. e.g. for each test we create a temporary folder and then set the custom variable of the `upload_folder` to that value.
* This will keep our environment clean and the test will be independent.

* We can add those custom configuration values to `Rocket.toml`:

![](examples/rocket/config-with-tests/Rocket.toml)

* We can then create a struct describing these parameters (MyConfig in our example) and we can use `.attach(AdHoc::config::<MyConfig>())` to make Rocket read these values.
* In each route we can use `config: &State<MyConfig>` to get the configuration values.

* In the tests we can override specific configuration values before we create the client.

```
let provider = Config::figment().merge(("custom", "In Test 1"));
let app = super::rocket().configure(provider);

let client = Client::tracked(app).unwrap();
```

![](examples/rocket/config-with-tests/src/main.rs)

* This will ensure that each test will have its own value for this `custom` field.


## Rocket - Request guard - FromRequest
{id: rocket-request-guard}
{i: FromRequest}
{i: Request}
{i: Outcome}
{i: async_trait}

* These guards don't check anything yet, they just either accept or reject the reuqest, but this can be a good skeleton.

![](examples/rocket/request-guard/src/main.rs)

## Rocket - Blog using request guard - FromRequest
{id: rocket-blog-using-request-guard}
{i: FromRequest}
{i: Status}

![](examples/rocket/blog-with-guard/src/main.rs)
![](examples/rocket/blog-with-guard/src/tests.rs)

![](examples/rocket/blog-with-guard/pages/about.md)
![](examples/rocket/blog-with-guard/pages/main.md)

## Rocket - Blog with FromParam - selectively accept pathes
{id: rocket-blog-with-fromparam}

![](examples/rocket/blog-with-fromparam/src/main.rs)
![](examples/rocket/blog-with-fromparam/src/tests.rs)

![](examples/rocket/blog-with-fromparam/pages/about.md)
![](examples/rocket/blog-with-fromparam/pages/main.md)

## Rocket - Return Status (arbitrary HTTP code)
{id: rocket-return-status}

* Returning a Status allows us to either return some content or return an arbitrary HTTP status code
* Then we can - if we want - setup a catcher for that error code to show content we would like to show.

![](examples/rocket/return-result/src/main.rs)
![](examples/rocket/return-result/src/tests.rs)

## Rocket - catch panic in the route handle
{id: rocket-catch-panic-in-the-route-handle}
{i: catch}
{i: catchers!}
{i: 500}

![](examples/rocket/catch-division-by-zero/src/main.rs)

## Simple TODO list with Rocket and SurrealDB
{id: simple-todo-list-with-rocket-and-surrealdb}

* TODO: why is the item.id.id shown as [object] in the web page while printing to the log shows the ID only.


Setup server:


```
docker volume create my-surreal-db
docker run --detach --restart always --name surrealdb -p 127.0.0.1:8000:8000 --user root -v my-surreal-db:/database surrealdb/surrealdb:v2.0.1 start --user root --pass root --log trace file://database
```

Dependencies

![](examples/rocket/simple-todo-with-surrealdb/Cargo.toml)

![](examples/rocket/simple-todo-with-surrealdb/Rocket.toml)

![](examples/rocket/simple-todo-with-surrealdb/src/main.rs)
![](examples/rocket/simple-todo-with-surrealdb/src/db.rs)

![](examples/rocket/simple-todo-with-surrealdb/templates/index.html.tera)

## Use Tera filters (length)
{id: use-tera-filters}
{i: Tera}
{i: length}

* The Tera template system comes with a number of [filters](https://keats.github.io/tera/docs/#filters)
* List of [built-in filters](https://keats.github.io/tera/docs/#built-in-filters)

this is a simple example using the `length` filter:

![](examples/rocket/use-tera-filter/templates/index.html.tera)

![](examples/rocket/use-tera-filter/src/main.rs)
![](examples/rocket/use-tera-filter/src/tests.rs)

## Create Tera filters
{id: create-tera-filters}

![](examples/rocket/create-tera-filter/Cargo.toml)
![](examples/rocket/create-tera-filter/Rocket.toml)

![](examples/rocket/create-tera-filter/src/main.rs)
![](examples/rocket/create-tera-filter/src/mytera.rs)
![](examples/rocket/create-tera-filter/src/tests.rs)

![](examples/rocket/create-tera-filter/templates/index.html.tera)

## Skip route by returning None
{id: skip-route-by-returning-none}
{i: Option}
{i: None}

* Unfortunately this does not work as I expected. see [my question](https://github.com/rwf2/Rocket/discussions/2876)

![](examples/rocket/skip-route/src/main.rs)

## Skip route using a guard
{id: skip-route-by-using-a-guard}
{i: FromRequest}
{i: Outcome::Success}
{i: Outcome::Forward}
{i: Outcome::Error}


![](examples/rocket/skip-route-using-guard/src/main.rs)

## Rocket Guards - experiment
{id: rocket-guards-experiment}
{i: TBD}

![](examples/rocket/guards/Cargo.toml)
![](examples/rocket/guards/Rocket.toml)
![](examples/rocket/guards/src/main.rs)

## Rocket return user-id
{id: rocket-return-user-id}
{i: TBD}

![](examples/rocket/return-result-user-id/Cargo.toml)
![](examples/rocket/return-result-user-id/src/main.rs)
![](examples/rocket/return-result-user-id/src/tests.rs)

