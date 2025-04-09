# YAML and Rust


YAML is a file format often used as a configuration file. Most of the programming languages have a way to deserialize YAML into some internal data structure. So does Rust via serde.


---

[YAML](https://yaml.org/) is a human-readable and human writable file format often used for configuration.
I maintain several project where people collect data into thousands of YAML files and the some program collects the data and generats a web site.

This is a collection of articles on dealing with YAML in the Rust programming language.

* [Read arbitrary YAML files in Rust](./read-arbitrary-yaml.md) without the need to know the whole structure of the file. `serde`, `serde_yaml`
* [Read a simple, flat YAML file into a struct](./read-simple-yaml.md) `struct`, `Deserialize`
* [Set default values while deserializing YAML](./default-values-deserializing-yaml.md) `default`
* [Deserializing YAML - deny unknown fields](./yaml-deny-unknown-fields.md) `deny_unknown_fields`


