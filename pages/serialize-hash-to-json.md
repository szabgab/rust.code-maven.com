---
title: Serialize and deserialize HashMap to JSON in Rust
timestamp: 2023-12-29T09:30:01
author: szabgab
published: true
description: Serialize a HashMap to JSON and deserialize JSON to a HashMap in Rust using serde_json.
tags:
    - HashMap
    - serialize
    - deserialize
    - serde_json
    - serde
    - to_string
    - from_str
    - JSON
    - assert_eq!
todo:
    - TODO
---

In most of the other articles about [JSON and Rust](/json) we deal with data that can be represented by a `struct` where the keys are fixed and known up-front.

There are, however, cases when the keys can be arbitrary values of some type. E.g. arbitrary strings. For example if we want to count how many times a word
appears in a text, the best representation might be a HashMap where we won't know up-front which words are in a text and thus won't know up-front what will be the keys.

So in this example we took a HashMap that with key-value pairs in it mapping Strings to numbers and we serialized the HashMap using `serde_json::to_string` to a JSON string:

```rust
let json_string = serde_json::to_string(&data_before).unwrap();
```


Then deserialized it using `serde_json::from_str` back to a HashMap with the appropriate type definition.

```rust
let data_after: HashMap<String, u32> = serde_json::from_str(&json_string).unwrap();
```

Actually I included two ways to deserialize, once by declaring the type of the new variable and once by using the [Turbofish 🐠](/turbofish) operator.

```rust
let data_turbofish = serde_json::from_str::<HashMap<String, u32>>(&json_string).unwrap();
```

Use whichever makes more sense to you.

After both deserialization we used the **assert_eq!** macro to compare the resulting data structure to the original one.

## The dependencies in Cargo.toml

We need [serde_json](https://crates.io/crates/serde_json) for this.

![](examples/hash-to-json/Cargo.toml)

## The code

![](examples/hash-to-json/src/main.rs)


## Running the example

```
cargo run
```

Will print:

```
{"bar": 42, "foo": 23}
{"bar":42,"foo":23}
{"bar": 42, "foo": 23}
{"foo": 23, "bar": 42}
```

