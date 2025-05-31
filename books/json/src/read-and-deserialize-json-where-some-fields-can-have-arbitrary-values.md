# Read and deserialize JSON file where some fields can have arbitrary values

* Defined fields: title, jobs, runs-on
* Values selected from a well defined list: ubuntu, windows
* User supplied values: test, build, "Sample file"


{% embed include file="src/examples/json/deserialize-json-where-some-keys-are-arbitrary/data.json" %}

## Cargo.toml

{% embed include file="src/examples/json/deserialize-json-where-some-keys-are-arbitrary/Cargo.toml" %}

## Output

{% embed include file="src/examples/json/deserialize-json-where-some-keys-are-arbitrary/out.out" %}


## The code

{% embed include file="src/examples/json/deserialize-json-where-some-keys-are-arbitrary/src/main.rs" %}



