# SQLite INSERT with placeholder

* prepare
* bind
* next

* We can (and should) use placeholders in INSERT statements as well

```rust
{{#include examples/sqlite/in-memory-insert-placeholders/src/main.rs }}
```

```
{{#include examples/sqlite/in-memory-insert-placeholders/out.out }}
```

