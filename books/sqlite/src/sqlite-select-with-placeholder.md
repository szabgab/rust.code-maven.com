# SQLite SELECT with placeholder

* prepare
* bind_iter
* next

* We could also bind multiple variables in one statement. We can use either the index of the placeholder or its name. The latter makes more readable code.

```rust
{{#include examples/sqlite/in-memory-select-placeholders/src/main.rs }}
```

```
{{#include examples/sqlite/in-memory-select-placeholders/out.out }}
```

