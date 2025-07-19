# Rocket - Custom configuration and injecting (overriding) values in tests

* We would like to have some custom configuration values for the application. (e.g. database address, API key for email sending service, folder to save uploaded imagesetc.)
* During testing we would like to set these values separately. e.g. for each test we create a temporary folder and then set the custom variable of the `upload_folder` to that value.
* This will keep our environment clean and the test will be independent.

* We can add those custom configuration values to `Rocket.toml`:

{% embed include file="src/examples/rocket/config-with-tests/Rocket.toml" %}

* We can then create a struct describing these parameters (MyConfig in our example) and we can use `.attach(AdHoc::config::<MyConfig>())` to make Rocket read these values.
* In each route we can use `config: &State<MyConfig>` to get the configuration values.

* In the tests we can override specific configuration values before we create the client.

```
let provider = Config::figment().merge(("custom", "In Test 1"));
let app = super::rocket().configure(provider);

let client = Client::tracked(app).unwrap();
```

{% embed include file="src/examples/rocket/config-with-tests/src/main.rs" %}

* This will ensure that each test will have its own value for this `custom` field.


