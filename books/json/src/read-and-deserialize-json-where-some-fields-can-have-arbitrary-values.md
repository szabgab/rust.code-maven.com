# Read and deserialize JSON file where some fields can have arbitrary values

* Defined fields: title, jobs, runs-on
* Values selected from a well defined list: ubuntu, windows
* User supplied values: test, build, "Sample file"


```json
{{#include examples/json/deserialize-json-where-some-keys-are-arbitrary/data.json }}
```

## Output

```
{{#include examples/json/deserialize-json-where-some-keys-are-arbitrary/out.out }}
```


## The code

```rust
{{#include examples/json/deserialize-json-where-some-keys-are-arbitrary/src/main.rs }}
```



