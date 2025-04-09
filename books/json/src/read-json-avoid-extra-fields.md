# Read JSON avoid extra fields - deny_unknown_fields

* What should happen if a new field is added to the JSON, but our code is not updated yet?
* Should we let it slide, or should we report an error?

```rust
{{#include examples/json/avoid-extra-fields/src/main.rs }}
```

```
{{#include examples/json/avoid-extra-fields/out.out }}
```


