# Read JSON: Warn on extra (unknown) fields with serde_ignored

* If the `deny_unknown_fields` seems to be too strict, you can use the `serde_ignored` crate to collect the extra fields and do something with that knowledge. For example, to warn about them.

* Here are two JSON files, a good one with 2 fields and a bad one with an extra field.

```json
{{#include examples/json/warn-on-extra-fields/good.json }}
```

```json
{{#include examples/json/warn-on-extra-fields/bad.json }}
```

We defined the struct to be Deserialize-d just as we did earlier, but then we set up a deserializer
and use that to deserialized the JSON string. We now have the list of all the extra fields.

```rust
{{#include examples/json/warn-on-extra-fields/src/main.rs }}
```

```toml
{{#include examples/json/warn-on-extra-fields/Cargo.toml }}
```

```
$ cargo run good.json
Person { name: "Foo Bar", email: "foo@bar.com" }
-------
Unused fields: {}
Person { name: "Foo Bar", email: "foo@bar.com" }
```

```
$ cargo run bad.json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/warn-on-extra-fields bad.json`
Person { name: "Foo Bar", email: "foo@bar.com" }
-------
Unused fields: {"age"}
Person { name: "Foo Bar", email: "foo@bar.com" }
```

