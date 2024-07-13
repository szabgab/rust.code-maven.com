# Development tools
{id: tools}

## Code formatting
{id: code-formatting}

* [rustfmt - cargo fmt](https://github.com/rust-lang/rustfmt)

```
cargo fmt
```

## Clippy
{id: clippy}

* [rust-clippy](https://github.com/rust-lang/rust-clippy)

```
cargo install clippy
```

```
cargo clippy
```

## Extreme Clippy
{id: extreme-clippy}


* [Extreme Clippy for new crate](https://rust.code-maven.com/extreme-clippy)
* [Extreme Clippy for existing crate](https://rust.code-maven.com/extreme-clippy-for-existing-crate)

## Cargo audit
{id: cargo-audit}

* [Rust Security Advisory database](https://rustsec.org/)

```
cargo install cargo-audit
```

```
cargo audit
```

## Cargo watch
{id: cargo-watch}

```
cargo install cargo-watch

cargo watch -x check

cargo watch -x check -x test -x run
```

## Pre-commit hook
{id: pre-commit}

Install [pre-commit](https://pre-commit.com/) and then run `pre-commit install` to configure it on the current.


Examples
* [rust-digger](https://github.com/szabgab/rust-digger/)
* [Code-Maven SSG](https://github.com/szabgab/code-maven.rs/)

## Continuous Integration
{id: continuous-integration}
{i: CI}
{i: GitHub}

* GitHub workflow (GitHub Actions)



