# Read JSON to Vector

* Sometimes the root of the JSON struct is a list that contains structures of key-value pairs. We can read that into a vector of structs.
* We already saw this, but then the values were plain strings. Now they are parsed into a struct.

```json
{{#include examples/json/read-json-to-vector/data.json }}
```

## Code

```rust
{{#include examples/json/read-json-to-vector/src/main.rs }}
```

## Output

```
{{#include examples/json/read-json-to-vector/out.out }}
```

