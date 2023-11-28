---
title: Read a simple YAML file into a struct
timestamp: 2023-11-28T20:30:01
description: You can define a struct that represents the fields of a YAML file to get automatic data conversion.
tags:
    - YAML
    - struct
    - serde
    - Deserialize
todo:
    - TODO
---

In an earlier article we saw how to [read an arbitrary YAML file](/read-arbitrary-yaml) and then access the individual fields.

A more time consuming, but more robust way is to define a `struct` mapping all the fields of the YAML file. We'll see several such examples.

For all of them we'll need both [serde_yaml](https://crates.io/crates/serde_yaml) and [serde](https://crates.io/crates/serde) as you can see in the `Cargo.toml` file:

![](examples/read-simple-yaml/Cargo.toml)


## Data

Let's see this simple YAML file:

![](examples/read-simple-yaml/data.yaml)

## The code

![](examples/read-simple-yaml/src/main.rs)

Before getting to the `main` function we define a `struct` with the fields of the YAML file and the type of values the YAML file has.
We add the [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) **trait** to it.

The first few lines in the `main` function is to accept the name of the YAML file on the command line as described in the
article on how to [expect one command line parameter](/expect-one-command-line-parameter).

The we open the YAML file using the [std::fs::File::open](https://doc.rust-lang.org/std/fs/struct.File.html) function and then we call the
[serde_yaml::from_reader](https://docs.rs/serde_yaml/latest/serde_yaml/fn.from_reader.html) function to read the content of the file
and parse it. The most important part is to assign to a variable that was defined to be type of the `struct` we have defined earlier.
In this example I cleverly used the name `Data`. Don't do that! Find some more descriptive name in real code!

```rust
let data: Data =
```

The content of the resulting variable looks like this:

```
Data {
    fname: "Foo",
    lname: "Bar",
    year: 2023,
    height: 6.1,
    married: true,
}
```

Very nice.

We can access the individual values using the dot-notation.
We can then print the values or, as shown in this case, we can use `assert_eq!` or `assert!` to verify the values.

