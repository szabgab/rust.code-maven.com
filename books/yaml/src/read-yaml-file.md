# Read YAML file

- serde
- serde_yml
- from_reader
- as_i64
- as_str
- struct

* TODO: if the number of dashes at the top is not correct (e.g. 4, the parser will panic, how to handle this properly?)

```
{{#include examples/yaml/read-yaml-file/out.out }}
```

```yaml
{{#include examples/yaml/read-yaml-file/data.yaml }}
```

```toml
{{#include examples/yaml/read-yaml-file/Cargo.toml }}


```rust
{{#include examples/yaml/read-yaml-file/src/main.rs }}

```
{{#include examples/yaml/read-yaml-file/out.out }}
```

