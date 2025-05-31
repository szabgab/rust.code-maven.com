# Serialize and deserialize HashMap to JSON in Rust

* If we have a `HashMap` we can easily serialize it into a JSON string (which we can save to a file if we want to).
* And we can deserialize back to HashMap and check that we get back the same data.

{% embed include file="src/examples/json/serialize-hashmap/Cargo.toml" %}

## Code

{% embed include file="src/examples/json/serialize-hashmap/src/main.rs" %}

## Output

{% embed include file="src/examples/json/serialize-hashmap/out.out" %}



