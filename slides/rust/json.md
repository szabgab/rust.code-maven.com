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

* TODO: show in both cases how to handle cases when a field is missing or the value is incorrect type (eg. we have  "x": "qqrq")
* TODO: show in both cases what happens if there are fields in the JSON file that we don't know about in the struct.
* TODO: show lists in the JSON file
* TODO: show deeper structure


## Read JSON file using from_reader
{id: read-json-file-using-from-reader}
{i: from_reader}

![](examples/json/json-read-from-reader/data.json)

![](examples/json/json-read-from-reader/Cargo.toml)

![](examples/json/json-read-from-reader/src/main.rs)

![](examples/json/json-read-from-reader/out.out)
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


