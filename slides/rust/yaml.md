# YAML
{id: yaml}

## YAML in Rust
{id: yaml-in-rust}
{i: serde_yaml}
{i: serde_yml}

* [serde_yaml](https://crates.io/crates/serde_yaml) - deprecated
* [serde_yml](https://crates.io/crates/serde_yml) - a fork of serde_yaml

## Read YAML file
{id: read-yaml-file}
{i: serde}
{i: serde_yml}
{i: from_reader}
{i: as_i64}
{i: as_str}
{i: struct}
{i: TODO}

* TODO: if the number of dashes at the top is not correct (e.g. 4, the parser will panic, how to handle this properly?)

![](examples/yaml/read-yaml-file/out.out)


![](examples/yaml/read-yaml-file/data.yaml)

![](examples/yaml/read-yaml-file/Cargo.toml)


![](examples/yaml/read-yaml-file/src/main.rs)

![](examples/yaml/read-yaml-file/out.out)


## Read YAML file where some field can have arbitrary values
{id: yaml-where-some-fields-can-be-arbitrary-values}

![](examples/yaml/deserialize-yaml-where-some-keys-are-arbitrary/data.yaml)
![](examples/yaml/deserialize-yaml-where-some-keys-are-arbitrary/src/main.rs)

## Deserialize date in YAML
{id: deserialize-date-in-yaml}

![](examples/yaml/load-datetime-field/data.yaml)

![](examples/yaml/load-datetime-field/Cargo.toml)
![](examples/yaml/load-datetime-field/src/main.rs)

![](examples/yaml/load-datetime-field/out.out)

