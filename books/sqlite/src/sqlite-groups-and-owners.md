# SQLite - Groups and Owners (with FOREIGN KEY)

In order to make SQLite enforce FOREIGN KEY we need to turn on the [foreign_keys PRAGMA](https://sqlite.org/pragma.html#pragma_foreign_keys) on the connection.

* There is also a compile-time option called [default_foreign_keys](https://sqlite.org/compile.html#default_foreign_keys) but I think we don't want to compile our own sqlite.


```rust
{{#include examples/sqlite/groups-and-owner/src/main.rs }}
```


