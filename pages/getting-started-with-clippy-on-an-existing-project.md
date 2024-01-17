---
title: Getting started with Clippy on an existing project
timestamp: 2024-01-17T17:30:01
published: true
description: Clippy can provide a lot of valuable improvements for your code, but it is hard to add it to an existing project.
tags:
    - Clippy
    - lint
---


[Clippy](https://doc.rust-lang.org/stable/clippy/index.html) can provide a lot of valuable improvements for your code, but it is hard to add it to an existing project.

In any Crate where you have the standard Rust toolchain you can run `cargo clippy` and get numerous complaints.

Once you fix all the simple lints, you can go to the [documentation of Clippy](https://doc.rust-lang.org/stable/clippy/index.html) and find more powerful spells, err, lints.
To make life easier there are also some lints grouped together.

For example, I decided to be pedantic and enabled the `pedantic` lint group by adding the following to `Cargo.toml`:

```toml
[lints.clippy]
pedantic = "deny"
```

running `cargo clippy` after this gave me some 60 failures. Very annoying. A better way would have been to start the project with these lints already enabled, but oh well.
That time is gone.

So what I did is disabled the individual lints one-by-one and had the following in `Cargo.toml`:


```toml
[lints.clippy]
pedantic = "deny"
must_use_candidate = "allow"
manual_string_new = "allow"
needless_pass_by_value = "allow"
no_effect_underscore_binding = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
module_name_repetitions = "allow"
uninlined_format_args = "allow"

#unwrap_used = "deny"
#unwrap_or_default = "deny"
```

This **almost** worked. This disabled all the lint errors **I** received in my code-base, except the ones that violated the `uninlined_format_args` lint.
I was baffled and opened and [issue](https://github.com/rust-lang/rust-clippy/issues/12161). I got a quick response that the proper way to do this is to
set the priority of the `pedantic` lint manually. So I ended up with this configuration: (see the change in the 2nd row).


```toml
[lints.clippy]
pedantic = { priority = -1, level = "deny" }
must_use_candidate = "allow"
manual_string_new = "allow"
needless_pass_by_value = "allow"
no_effect_underscore_binding = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
module_name_repetitions = "allow"
uninlined_format_args = "allow"

#unwrap_used = "deny"
#unwrap_or_default = "deny"
```

As you can see I also have two lints at the end commented out. These and probably some other lints are not part of the `pedantic` group, but I think I will want to enable them later.
Right now they have too many issues to report.


## Fixing the lints one-by-one

My approach at this point is to go over the individual lints that I've disabled. Pick one that looks the most important (or the easiest to fix), enable it by removing
the allow-line from `Cargo.toml` and fix all the code where I have this issue.

I personally find it easier to fix similar issues throughout the code.

Of course I can do that relatively safely as I have extensive tests for the code, so I know that chance of introducing a bug without a test failing is rather slim.


## Pre-commit

In order to enforce the rules I even added running clippy to the [Pre-commit](https://pre-commit.com/) configuration file and to the GitHub Actions CI configuration.

`.pre-commit-config.yaml` like this:

```
  - repo: local
    hooks:
    - id: cargo-clippy
      name: cargo clippy
      language: system
      entry: cargo clippy -- --deny warnings
      always_run: true
      pass_filenames: false
      files: \.rs$
```

## GitHub Actions CI


```
jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Clippy stop at any warning
      run: cargo clippy -- --deny warnings
```

## Conclusion

Adding Clippy or extending the Clippy lints in an existing project is not easy and might not feel as productive work, but once can try to do that step-by-step during
a longer period of time and that will help smooth the introduction of the lints and help improve the code-base.







