---
title: Read arbitrary YAML files in Rust
timestamp: 2023-11-28T14:30:01
description: Read a YAML files without knowing up-front its structure?
tags:
    - YAML
    - serde
    - serde_yaml
    - as_str
    - as_u64
    - as_f64
    - as_sequence
    - get
todo:
    - show defining a 
---

When we need to read a YAML file in Rust ideally we would define a struct that maps the fields of the YAML file.
That would help with data validation and later with the handling of the data. However we can't always do that
and often we might just want to read in the YAML file and think about the specific definition later.

In this examples we'll see how to do this.

We are using [serde_yaml](https://crates.io/crates/serde_yaml) for this:

![](examples/read-arbitrary-yaml/Cargo.toml)

We have two YAML files.

## Invalid YAML format

One of them has invalid YAML format.

![](examples/read-arbitrary-yaml/broken.yaml)

If we run our code on this file we get an error message.

```
cargo run broken.yaml

There was an error parsing the YAML file did not find expected key at line 2 column 1, while parsing a block mapping
```


## Valid YAML file

We also have a good YAMl file:

![](examples/read-arbitrary-yaml/data.yaml)


## The code

This is the code:

![](examples/read-arbitrary-yaml/src/main.rs)

In the first few lines we are just [accepting a filename on the command line](/expect-one-command-line-parameter).

Then we open the file using [std::File::open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) and use
[serde_yaml::from_reader](https://docs.rs/serde_yaml/latest/serde_yaml/fn.from_reader.html) to read the YAML file and convert to an internal data structure.

This data structure, assigned to the `data` variable name, is of type [serde_yaml::Value](https://docs.rs/serde_yaml/latest/serde_yaml/enum.Value.html).
We printed it out using the first `println!` statement.

```
Mapping {
    "fname": String("Foo"),
    "lname": String("Bar"),
    "year": Number(2023),
    "height": Number(6.1),
    "numbers": Sequence [
        Number(23),
        Number(19),
        Number(42),
    ],
    "children": Sequence [
        Mapping {
            "name": String("Alpha"),
            "birthdate": Number(2020),
        },
        Mapping {
            "name": String("Beta"),
            "birthdate": Number(2022),
        },
    ],
}
```

From this point we can access the elements of this Mapping using either the `data[FIELD]` format or `data.get(FIELD)`.
The former would `panic!` if we supplied a FIELD that does not exists. The latter, the `get` function returns
an [Option](https://doc.rust-lang.org/std/option/enum.Option.html) and thus we can either use `unwrap` on it, actually
disregarding the possibility that the `get` call can return `None` or we can use `match` to handle the `None` as well.
In this code we used both strategies in different cases. Primarily to show how they can be done.

In some cases you will see `println!` statements to show the values, in some cases there are also `assert_eq!` statement
to show you what are the expected values.

Besides knowing the name of the field you'd like to fetch you also need to know the type of the field and then you need
to use one of the conversion methods of the [Value](https://docs.rs/serde_yaml/latest/serde_yaml/enum.Value.html) enum.

In the code you can see examples for [as_str](https://docs.rs/serde_yaml/latest/serde_yaml/enum.Value.html#method.as_str)
[as_u64](https://docs.rs/serde_yaml/latest/serde_yaml/enum.Value.html#method.as_u64), and
[as_f64](https://docs.rs/serde_yaml/latest/serde_yaml/enum.Value.html#method.as_f64)
to fetch the primitive values.

You can also see the [as_sequence](https://docs.rs/serde_yaml/latest/serde_yaml/enum.Value.html#method.as_sequence) to access
a sequence of values.


## The output

```
field: fname
fname=Foo
fname=Foo
field: lname
lname=Bar
lname=Bar
field: address
field: year
year=2023
year=2023
field: height
height=6.1
height=6.1

year=2023

height=6.1

Sequence [Number(23), Number(19), Number(42)]
[Number(23), Number(19), Number(42)]
23
19
42

child: Mapping {"name": String("Alpha"), "birthdate": Number(2020)}
name: Alpha
birthdate: 2020
child: Mapping {"name": String("Beta"), "birthdate": Number(2022)}
name: Beta
birthdate: 2022
```

## Conclusion

It might be a bit cumbersome to access the values this way, but it needs less start-up work of
creating the struct mapping the YAML file.

If you have a YAML file with a data structure that differs from this and you'd like me to add such
and example, let me know by opening an issue via the link below.


