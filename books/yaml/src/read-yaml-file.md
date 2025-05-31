# Read YAML file

- serde
- serde_yml
- from_reader
- as_i64
- as_str
- struct

* TODO: if the number of dashes at the top is not correct (e.g. 4, the parser will panic, how to handle this properly?)

{% embed include file="src/examples/yaml/read-yaml-file/out.out" %}

{% embed include file="src/examples/yaml/read-yaml-file/data.yaml" %}

{% embed include file="src/examples/yaml/read-yaml-file/Cargo.toml" %}


{% embed include file="src/examples/yaml/read-yaml-file/src/main.rs" %}

{% embed include file="src/examples/yaml/read-yaml-file/out.out" %}

