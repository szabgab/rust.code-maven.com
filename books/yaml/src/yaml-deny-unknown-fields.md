# Deserializing YAML - deny unknown fields

If we have default values or fields are optional then we might never catch if there is a typo in one of the fields.

- deny_unknown_fields
---

Defining [default values for fields in YAML](./default-values-deserializing-yaml.md) or making fields optional are very useful features,
but if there is a typo in the YAML file we might never notice it. This is certainly a source for a lot of frustration.
Luckily there is a solution. We can tell serde to `deny_unknown_fields`. That way if there is a typo in the names of one of the fields,
the parser will return an error.

This is basically what we need to do:

```rust
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Person {
    name: String,

    #[serde(default = "get_default_married")]
    married: bool,
}

fn get_default_married() -> bool {
    false
}
```

In this `struct` we expect two fields, `name` is required, but if there is no `married` field then we set it do `false`.

This works well when the YAML file has all the fields:

```yaml
{{#include examples/yaml-deny-unknown-fields/all.yaml }}
```

```
name: Foo Bar
married: true
```

or when the `married` field is missing:

```yaml
{{#include examples/yaml-deny-unknown-fields/missing.yaml }}
```

```
name: Foo Bar
married: false
```

However if there is a typo and we have `maried` instead of `married`:

```yaml
{{#include examples/yaml-deny-unknown-fields/typo.yaml }}
```

Then without the `deny_unknown_fields` we get:

```
name: Foo Bar
married: false
```

Adding the `deny_unknown_fields` attribute would yield the following error:

```
Could not parse YAML file: unknown field `maried`, expected `name` or `married` at line 2 column 1
```

## Full example

```rust
{{#include examples/yaml-deny-unknown-fields/src/main.rs }}
```

## Dependencies in Cargo.toml

```toml
{{#include examples/yaml-deny-unknown-fields/Cargo.toml }}
```

## A potential problem

What if we get the files from some external source and the provider decides to add a new field? Our code will stop functioning.
On one hand it is good that we immediately notice the extra field, on the other hand we would not want our service to stop working
at 2am just because the data supplier decided to roll out their changes at that time.

I am not sure what should be the right solution. How do we balance the two needs: avoiding using default values when there was a typo
and allowing the seamless addition of new fields.


