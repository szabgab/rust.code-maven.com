# Read arbitrary JSON without much preparation

A simple way to get started using data that arrives in JSON format.

- serde_json
- from_reader
- Value
- JSON
- as_str
- unwrap
- as_f64
- as_u64
- as_array
- assert_eq!


In order to read a JSON file, probably the best approach is to define a struct that will hold the content of the JSON file.
Unfortunately it can be time consuming, so  to get started you might want to read in the content of the JSON file and
use the hand-picked values from the data.

In order to do this we'll use [serde_json](https://docs.rs/serde_json/latest/serde_json/) as you can see in the `Cargo.toml`
file:

## Cargo.toml

```toml
{{#include examples/read-arbitrary-json/Cargo.toml }}
```

This is the data file we use. It does not have any real meaning, it just contains all kinds of data types.

```json
{{#include examples/read-arbitrary-json/data.json }}
```

## The code:

```rust
{{#include examples/read-arbitrary-json/src/main.rs }}
```


The program expects the name of the JSON file on the command line. We have a function called `get_filename` that will
return the name of the file or will exit if the user did not provide a filename.

Then we open the JSON file and read in the content and convert it into a data-structure of type
[Value](https://docs.rs/serde_json/latest/serde_json/enum.Value.html) using the
[serde_json::from_reader](https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html) function.
We assigned it to a variable called `data`. When you work on real data, please try using a more descriptive name!

Then we see two ways to access the data one is calling the [get](https://docs.rs/serde_json/latest/serde_json/enum.Value.html#method.get) method
on the data. This will return an Option so we need to `unwrap` it to get the real value or we need to arrange some
more serious error handling if we are not sure the field "fname" exists.

```rust
data.get("fname").unwrap()
```

Alternatively we can access the data using square brackets:

```rust
data["lname"]
```

In either case we get another **Value** from which we can get the real value by using one of the `as_` functions listed
for the [Value](https://docs.rs/serde_json/latest/serde_json/enum.Value.html).


In normal code you'd probably do something with the values these functions return, but for our demonstration I used
the [assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html) macro to compare the values to expected values.


## Running the code

```
cargo run data.json
```

will produce this output:

```rust
[src/main.rs:17] &data = Object {
    "children": Array [
        Object {
            "birthdate": Number(2020),
            "name": String("Alpha"),
        },
        Object {
            "birthdate": Number(2022),
            "name": String("Beta"),
        },
    ],
    "fname": String("Foo"),
    "height": Number(178.2),
    "lname": String("Bar"),
    "married": Bool(true),
    "numbers": Array [
        Number(23),
        Number(19),
        Number(42),
    ],
    "year": Number(1992),
}
```

## Conclusion

This way of reading a JSON file can be useful to get started, but we'll need a more robust way to verify the data and to
make it easier to use the data once we read it in.




