# JSON
{id: json}

## Serde for JSON
{id: serde-json}

* [serde](https://serde.rs/) is a framework for SERializing and DEserializing Rust data structures.

* [serde](https://crates.io/crates/serde)
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
{i: as_object}
{i: as_str}
{i: as_i64}

* We would like to read the following simple JSON file:

![](examples/json/read-simple-json-manually/data.json)

* We need [serde](https://serde.rs/) and [serde_json](https://docs.rs/serde_json/latest/serde_json/)

```
cargo add serde_json
cargo add serde -F derive
```

![](examples/json/read-simple-json-manually/Cargo.toml)

* We first open the file and read the content of the file.
* Then we parse the string as some generic JSON data into a generic `serde::Value` structure. [serde::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html) is an `enum` that can hold any value.
* In this case we need to extract and convert the values.

![](examples/json/read-simple-json-manually/src/main.rs)

![](examples/json/read-simple-json-manually/out.out)

## Read Simple JSON file into a struct
{id: read-simple-json-to-struct}
{i: Deserialize}
{i: read_to_string}

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

## Read complex JSON
{id: read-complex-json}

![](examples/json/read-person/src/main.rs)

![](examples/json/read-person/out.out)

## JSON files - missing fields
{id: json-missing-fields}

* How to deal with fields that are in our `struct` but are missing from the JSON?

* Return `Result<Error>` and handle it or let it `panic!`.
* Set a default value.
* Make the value optional with `Option`.

## Read JSON handle missing fields - set defaults
{id: read-json-set-default-fields}
{i: default}

![](examples/json/set-default-values/src/main.rs)

![](examples/json/set-default-values/out.out)


## Read JSON with Optional fields
{id: read-json-with-optional-fields}
{i: Option}

![](examples/json/read-with-optional-field/just_name.json)
![](examples/json/read-with-optional-field/no_name.json)
![](examples/json/read-with-optional-field/married_no_language.json)
![](examples/json/read-with-optional-field/married_with_python.json)
![](examples/json/read-with-optional-field/single_with_python.json)

![](examples/json/read-with-optional-field/src/main.rs)


## Read JSON avoid extra fields - deny_unknown_fields
{id: read-json-avoid-extra-fields}
{i: deny_unknown_fields}

* What should happen if a new field is added to the JSON, but our code is not updated yet?
* Should we let it slide, or should we report an error?

![](examples/json/avoid-extra-fields/src/main.rs)

![](examples/json/avoid-extra-fields/out.out)


## Alias some fields in JSON (handle dash in JSON kesy)
{id: alias-some-fields-in-json}
{i: alias}

![](examples/json/alias-json-fields/data.json)

![](examples/json/alias-json-fields/src/main.rs)

![](examples/json/alias-json-fields/out.out)

## Read JSON to Vector
{id: read-json-to-vector}

* Sometimes the root of the JSON struct is a list that contains structures of key-value pairs. We can read that into a vector of structs.

![](examples/json/read-json-to-vector/data.json)

![](examples/json/read-json-to-vector/src/main.rs)

![](examples/json/read-json-to-vector/out.out)

## Read lists of JSON structures
{id: read-list-of-json-structures}
{i: TBD}

![](examples/json/read-list-of-json/data.json)

![](examples/json/read-list-of-json/src/main.rs)

![](examples/json/read-list-of-json/out.out)


## Serialize and deserialize HashMap to JSON in Rust
{id: serialize-and-deserialize-hashmap-to-json}
{i: json}
{i: HashMap}

![](examples/json/serialize-hashmap/Cargo.toml)

![](examples/json/serialize-hashmap/src/main.rs)

![](examples/json/serialize-hashmap/out.out)


## JSON serialize struct
{id: json-serialize-struct}
{i: serde}
{i: serde_json}

* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)

![](examples/json/json-serialize-struct/Cargo.toml)
![](examples/json/json-serialize-struct/src/main.rs)

## Serialize struct and Deserialize JSON
{id: serialize-struct-deserialize-json}
{i: Serialize}
{i: Deserialize}
{i: to_string}
{i: from_string}

![](examples/json/serde-demo/Cargo.toml)
![](examples/json/serde-demo/src/main.rs)


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


