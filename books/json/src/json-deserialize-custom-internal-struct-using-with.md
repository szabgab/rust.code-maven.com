# JSON deserialize custom internal struct using "with"


* What if the JSON contains phone numbers with area code as a single string, but we would like to represent the phone number as a struct with two fields "area" and "number"?
* We can tell serde to deserialize this field with a custom function using the `with` attribute.


## Cargo.toml

```toml
{{#include examples/json/deserialize-to-internal-struct/Cargo.toml }}
```

## Data

```json
{{#include examples/json/deserialize-to-internal-struct/data.json }}
```

## Code

```rust
{{#include examples/json/deserialize-to-internal-struct/src/main.rs }}
```


