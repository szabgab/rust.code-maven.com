---
title: Set default values while deserializing YAML in Rust
timestamp: 2023-12-18T15:15:01
author: szabgab
published: true
description: Some YAML files might be missing some value. In some cases we might want to set default values in the deserialized struct.
tags:
    - YAML
    - default
    - serde
---

We saw how to [read a deserialize a simple YAML into a struct](/read-simple-yaml). What happens if some of the fields we are expecting in the YAML file are missing?

For the following example we created a `struct` like this

```rust
#[derive(Deserialize)]
struct Person {
    name: String,
    email: String,
    year: u32,
    married: bool,
}
```

If se supply the following YAML file, each field is filled.

{% include file="examples/yaml-default-values/all.yaml" %}

However if we provide the following file:

{% include file="examples/yaml-default-values/nameless.yaml" %}

We will get an error message in the `err` variable:

```
missing field `name`
```

We get a similar error if more than one field is missing:

{% include file="examples/yaml-default-values/data.yaml" %}

## Set the default values

One of the solutions is to set **default** values for some or all of the fields.
We can do that by using the default attribute and passing in the name of a function
that is going to return the default value.

```rust
#[derive(Deserialize)]
struct Person {
    name: String,

    #[serde(default = "get_default_email")]
    email: String,

    #[serde(default = "get_default_year")]
    year: u32,

    #[serde(default = "get_default_married")]
    married: bool,
}

fn get_default_email() -> String {
    String::from("default@address")
}

fn get_default_year() -> u32 {
    2000
}

fn get_default_married() -> bool {
    false
}
```


## The full example

{% include file="examples/yaml-default-values/src/main.rs" %}

In this example we have a function called `get_filename` that gets the name of the file from the command line.


## A problem - what if we have a typo?

What if this is the YAML file

{% include file="examples/yaml-default-values/typo.yaml" %}

Have you noticed the typo I made in one of the fields? I typed in "maried" instead of "married", but I could have mixed up the field called "color" and typed in "colour",
if there indeed was such a field.

The current code will happily disregard the field with the typo and use the default value for the "married" field.

That's not ideal.


## Dependencies

See the `Cargo`.toml` we had:

{% include file="examples/yaml-default-values/Cargo.toml" %}

