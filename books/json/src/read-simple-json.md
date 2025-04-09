# Read simple JSON and deserialize into a struct

Deserializing a JSON string is not too hard especially if the JSON is simple and if we don't need all the fields.

- JSON
- Deserialize
- serde
- serde_json
- Debug
- dbg!
- assert!
- assert_eq!

In another post we saw how to [process arbitrary JSON string](./read-arbitrary-json.md) without defining anything about it and without creating a struct.
It can be a good idea when we start experimenting with a new JSON structure, but eventually we will want to create a struct and deserialize the
JSON string into this struct.

In this example we start on that journey for a simple JSON File that looks like this:

```json
{{#include examples/read-simple-json/data.json }}
```


## We need both serde and serde_json

For this we'll need both [serde_json](https://crates.io/crates/serde_json)
and [serde](https://crates.io/crates/serde) with the derive feature.

```toml
{{#include examples/read-simple-json/Cargo.toml }}
```


## The code

```rust
{{#include examples/read-simple-json/src/main.rs }}
```

We need to create a struct to represent the data where we define the expected fields and their type.

As our real data does not have a lot of fields we could have created a struct defining all the fields,
but I wanted to show the process you might take if you have a bigger JSON file and don't want to do all the
work up-front, or if you don't actually need all the fields from the JSON string.


```rust
struct Person {
    fname: String,
    married: bool,
}
```

We need to add the [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait to it.
I also included the [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) trait to allow us to
use the [dbg!](https://doc.rust-lang.org/std/macro.dbg.html) macro to display the content of the struct.

```rust
#[derive(Deserialize, Debug)]
```

The we open the file and use the [serde_json::from_reader](https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html)
function to read in the file.

```rust
serde_json::from_reader(&file).expect("JSON parsing error")
```

The important part that differs from the [generic JSON parsing example](/read-arbitrary-json) is
that we read the data into a variable that was annotated with the `Person` struct:


```rust
let data: Person =
```

The we can use the `dbg!` macro to show the content of the struct. We also use the
[assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html) to verify the value of a
string and the [assert!](https://doc.rust-lang.org/std/macro.assert.html) macro to verify the
value of a `bool` (boolean) field.


## Conclusion

It is quite easy to get started deserializing a simple JSON file, especially if we don't need
to get all the fields right at the beginning.

There are, however many more aspect of JSON we need to deal with.

* How to handle more complex JSON structures?
* How to handle a JSON that has a list (array) at the root?
* How can we make sure we mapped all the fields?
* What to do if a field we added to the struct is missing from the JSON?
* What to do if there is a typo in the fields of the JSON?


