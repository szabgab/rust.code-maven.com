# JSON
{id: json}

## Serde JSON
{id: serde-json}

* [serde](https://serde.rs/)

* [serde on Crates.io](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)


Adding dependencies:

```
cargo add serde_json
cargo add serde -F derive
```

## Read Simple JSON file manually
{id: read-simple-json-manually}
{i: serde_json}
{i: from_str}
{i: Value}

* We would like to read the following simple JSON file:

![](examples/json/read-simple-json-manually/data.json)

* We need [serde](https://serde.rs/) and [serde_json](https://docs.rs/serde_json/latest/serde_json/)

![](examples/json/read-simple-json-manually/Cargo.toml)

* We first open the file and read the content of the file.
* Then we parse the string as some generic JSON data into a generic `serde::Value` structure.
* In this case we need to extract and convert the values.

![](examples/json/read-simple-json-manually/src/main.rs)

![](examples/json/read-simple-json-manually/out.out)

## Read Simple JSON file into a struct
{id: read-simple-json-to-struct}

![](examples/json/read-simple-json-to-struct/data.json)

![](examples/json/read-simple-json-to-struct/src/main.rs)

![](examples/json/read-simple-json-to-struct/out.out)

## Read JSON file using from_reader manually
{id: read-json-file-using-from-reader-manually}
{i: from_reader}

```
cargo run ../person.json
```

![](examples/json/person.json)

![](examples/json/read-json-from-reader-manually/Cargo.toml)

![](examples/json/read-json-from-reader-manually/src/main.rs)

![](examples/json/read-json-from-reader-manually/out.out)


## Read JSON file using from_reader to a struct
{id: read-json-file-using-from-reader-to-a-struct}
{i: from_reader}

```
cargo run ../person.json
```

![](examples/json/read-json-from-reader-to-struct/src/main.rs)

![](examples/json/read-json-from-reader-to-struct/out.out)


## Read JSON avoid extra fields - deny_unknown_fields
{id: read-json-avoid-extra-fields}
{i: deny_unknown_fields}

![](examples/json/avoid-extra-fields/src/main.rs)

![](examples/json/avoid-extra-fields/out.out)

## Read JSON handle missing values - set defaults
{id: read-json-set-default-values}
{i: default}

![](examples/json/set-default-values/src/main.rs)

![](examples/json/set-default-values/out.out)


## Read complex JSON
{id: read-complex-json}

![](examples/json/read-person/src/main.rs)

![](examples/json/read-person/out.out)

* TODO: show lists in the JSON file

## Serialize and deserialize HashMap to JSON in Rust
{id: serialize-and-deserialize-hashmap-to-json}
{i: json}
{i: HashMap}

![](examples/json/serialize-hashmap/Cargo.toml)

![](examples/json/serialize-hashmap/src/main.rs)

![](examples/json/serialize-hashmap/out.out)


## JSON serialize examples
{id: json-serialize-examples}
{i: JSON}
{i: json!}
{i: serde_json}
{i: chrono}

* [serde_json](https://crates.io/crates/serde_json)
* [chrono](https://crates.io/crates/chrono)

![](examples/json/json-serialize/Cargo.toml)

![](examples/json/json-serialize/src/main.rs)

![](examples/json/json-serialize/out.out)

## JSON serialize struct
{id: json-serialize-struct}
{i: serde}
{i: serde_json}

* [serde_](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)

![](examples/json/json-serialize-struct/Cargo.toml)
![](examples/json/json-serialize-struct/src/main.rs)

## serde
{id: serde}
{i: Serialize}
{i: Deserialize}
{i: to_string}
{i: from_string}

![](examples/json/serde-demo/Cargo.toml)
![](examples/json/serde-demo/src/main.rs)

## serde manipulate json (change, add)
{id: serde-manipulate-json}
{i: json}
{i: get_mut}

![](examples/json/serde-manipulate-json/Cargo.toml)
![](examples/json/serde-manipulate-json/src/main.rs)


## JSON serialize struct with date
{id: json-serialize-struct-with-date}

![](examples/json/json-serialize-struct-with-date/Cargo.toml)
![](examples/json/json-serialize-struct-with-date/src/main.rs)

* Deserialize into struct
* Read multi-json files (the result of a json-based logger)


