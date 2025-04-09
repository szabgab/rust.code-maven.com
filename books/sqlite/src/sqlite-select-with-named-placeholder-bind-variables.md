# SQLite SELECT with named placeholder - bind variables

* bind
* prepare

* When the values are in variables - which is almost always - then we should use placeholders and bind the variables to them.
* We can use the `bind` method to bind them one-by-one.

```rust
{{#include examples/sqlite/in-memory-select-placeholders-bind/src/main.rs }}
```

```
{{#include examples/sqlite/in-memory-select-placeholders-bind/out.out }}
```


