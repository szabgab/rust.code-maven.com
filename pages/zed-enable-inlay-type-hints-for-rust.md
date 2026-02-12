---
title: Zed - enable inlay type hints for Rust projects
timestamp: 2026-02-12T11:30:01
author: szabgab
published:  true
show_related: true
description:
tags:
    - Zed
    - IDE
    - type hints
    - derive types
---

Inlay hints or inline type hints allow you to see the derived types of a variable.

For example when you assign the return value of a function to a variable but you don't explicitely write the type of the variable,
Rust will derive (deduct) the type based on the function signature.

If you declare a variable `let x = 2;`, Rust will derived that it is an `i32` and the inlay type hints will show `let x: i32 = 2;`.

In another case if add together two variables holding numbers (that default to `i32`) and declare the resulting variable to be `i8`
then Rust will derived that the original two variables also must be `i8`.

So

```rust
    let x = 2;
    let y = 3;
```

is understood as

```rust
    let x: i32 = 2;
    let y: i32 = 3;
```

but


```rust
    let x = 2;
    let y = 3;
    let z: i8 = x +y;
```

is understood as


```rust
    let x: i8 = 2;
    let y: i8 = 3;
    let z: i8 = x +y;
```

The inlay type hints will help you understand this.


## Inline (inlay) type hints:

I configured the following to make it work. I am not sure everything is really needed.

* Installed rust-analyzers

```
rustup component add rust-analyzer
```


* settings.json

In the `settings.json` file I added the following:


```json
  "inlay_hints": {
    "enabled": true,
  },
  "enable_language_server": true,
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path": "/home/gabor/.cargo/bin/rust-analyzer"
      },
      "initialization_options": {
        "inlayHints": {
          "maxLength": null,
          "lifetimeElisionHints": {
            "enable": "always",
            "useParameterNames": true,
          },
          "closureReturnTypeHints": {
            "enable": "always",
          },
        },
      },
    },
  },
```

* Trusted code

Opened the crate I was working on as a "project" and then in the top-left corner of the IDE I clicked on "Trusted project".


Then I could also use `Crtl-:` to toggle the inline hint.


