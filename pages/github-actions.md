---
title: GitHub Actions for Rust projects
timestamp: 2025-07-06T12:30:01
author: szabgab
published: true
show_related: true
description:
---

Using `ubuntu-latests`, `macos-latest`, or `windows-latests` will automatically bring you the most recent stable version of the Rust compiler. [see this example](/default-github-workflow-for-rust-on-linux).

This is nice as you won't need to worry about updating it, but it has the obvious disadvantage that the rust compiler automatically changes without you making any change in the configuration.


Many projects use the [actions-rs/toolchain](https://github.com/actions-rs/toolchain), but it has been deprecated on October 13, 2023.

Many use [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) to set the toolchain(s) explicitely. You can either set a specific version number or you can use the words:
`stable` for the most recent stable version, `beta` for the next version, `nightly` for wilde development efforts.

{% include file="examples/github-workflows/matrix.yml" %}


## GitHub Actions

Popular GitHub Actions used by Rust Crates.

* [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) to install the right toolchain(s),  target(s), and component(s).
* [bnjbvr/cargo-machete](https://github.com/bnjbvr/cargo-machete) to remove unused Rust dependencies
* [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) smart caching for rust/cargo projects with sensible defaults to reduce compilation time.


## Cargo commands

Various cargo commands I saw in the GitHub Workflows

```
cargo build --verbose
```

```
cargo test --verbose
```


```
cargo fmt
cargo fmt -- --check
cargo fmt -- --all --check
```


```
cargo clippy
cargo clippy -- --deny clippy::pedantic
cargo clippy --all-targets ${{ matrix.feature }} -- -D warnings
    // where the matrix.feature field can be ['', '--no-default-features', '--all-features']
```


```
cargo doc --no-deps --all-features
```

## GitHub Workflow elements

```yaml
env:
  CARGO_TERM_COLOR: always
```

## Caching with the generic cache action

Using [actions/cache](https://github.com/actions/cache) (but see the rust-cachec above)


```yaml
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-
```

## Verify git tag matches Cargo.toml version

```
      - name: Verify tag matches Cargo.toml version
        run: |
          TAG_VERSION=${GITHUB_REF#refs/tags/v}
          CARGO_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
          echo "Tag version: $TAG_VERSION"
          echo "Cargo.toml version: $CARGO_VERSION"
          if [ "$TAG_VERSION" != "$CARGO_VERSION" ]; then
            echo "Error: Tag version ($TAG_VERSION) does not match Cargo.toml version ($CARGO_VERSION)"
            exit 1
          fi
```

## Publish to crates.io

```
      - name: Publish to crates.io
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CRATES_IO_TOKEN }}
```

