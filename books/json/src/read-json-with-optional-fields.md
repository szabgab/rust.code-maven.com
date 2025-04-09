# Read JSON with optional fields: Option or default value?

Option

* In this example we expect the JSON to have 3 fields: `name`, `language`, and `married`.
* `name` is required field.
* `language` is optional, if not provided we set a default value.
* `married` is optional, if not provied we set `None`.

The type of the values is not releavant for the example.

```rust
{{#include examples/json/read-with-optional-field/src/main.rs }}
```

```json
{{#include examples/json/read-with-optional-field/just_name.json }}
```

```
$ cargo run -q just_name.json
Person {
    name: "Foo",
    language: "Rust",
    married: None,
}
We don't know if Foo is married or not
```

```json
{{#include examples/json/read-with-optional-field/no_name.json }}
```

```
$ cargo run -q no_name.json
thread 'main' panicked at src/main.rs:23:57:
JSON parsing error: Error("missing field `name`", line: 4, column: 1)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```json
{{#include examples/json/read-with-optional-field/married_no_language.json }}
```


```
$ cargo run -q married_no_language.json
Person {
    name: "Foo",
    language: "Rust",
    married: Some(
        true,
    ),
}
Marrige status: true
```

```json
{{#include examples/json/read-with-optional-field/married_with_python.json }}
```


```
$ cargo run -q married_with_python.json
Person {
    name: "Foo",
    language: "Python",
    married: Some(
        true,
    ),
}
Marrige status: true
```


```
{{#include examples/json/read-with-optional-field/single_with_python.json }}
```


```
$ cargo run -q single_with_python.json
Person {
    name: "Foo",
    language: "Python",
    married: Some(
        false,
    ),
}
Marrige status: false
```


