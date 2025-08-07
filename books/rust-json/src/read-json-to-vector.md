# Read JSON to Vector

* Sometimes the root of the JSON struct is a list that contains structures of key-value pairs. We can read that into a vector of structs.
* We already saw this, but then the values were plain strings. Now they are parsed into a struct.

{% embed include file="src/examples/json/read-json-to-vector/data.json" %}

## Code

{% embed include file="src/examples/json/read-json-to-vector/src/main.rs" %}

## Output

{% embed include file="src/examples/json/read-json-to-vector/out.out" %}

## Cargo.toml

{% embed include file="src/examples/json/read-json-to-vector/Cargo.toml" %}
