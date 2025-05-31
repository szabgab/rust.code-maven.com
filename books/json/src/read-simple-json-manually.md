# Read Simple JSON file manually

- serde_json
- from_str
- Value
- as_object
- as_str
- as_i64

* We would like to read the following simple JSON file:

{% embed include file="src/examples/json/read-simple-json-manually/data.json" %}

* We need [serde](https://serde.rs/) and [serde_json](https://docs.rs/serde_json/latest/serde_json/)

```
cargo add serde_json
cargo add serde -F derive
```

{% embed include file="src/examples/json/read-simple-json-manually/Cargo.toml" %}


* We first open the file and read the content of the file.
* Then we parse the string as some generic JSON data into a generic `serde::Value` structure. [serde::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html) is an `enum` that can hold any value.
* In this case we need to extract and convert the values.


## Code

{% embed include file="src/examples/json/read-simple-json-manually/src/main.rs" %}

## Output

{% embed include file="src/examples/json/read-simple-json-manually/out.out" %}


