# Regex match exact text

* Regex
* captures

* [regex::Captures](https://docs.rs/regex/latest/regex/struct.Captures.html)

In the first example we do something really simple, something for what we actually don't event need a regex,
but it can show you the basic syntax in Rust.

We have a string `The black cat climbed the green tree` that we read from somewhere so we assume it is a `String`.

We would like to see if the series of characters `cat` is in the string.

So we create a `Regex` using the `cat` string as a regex and call the `captures` method. This returns an `Option` that is either `Some` match or
`None`.


## Cargo.toml

```toml
{{#include examples/regex-simple-match/Cargo.toml }}
```

## main.rs

```rust
{{#include examples/regex-simple-match/src/main.rs }}
```

## Output

```
The black cat climbed the green tree
Full match: "cat"
No match
```



