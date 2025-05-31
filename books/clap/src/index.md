# CLI - Command line Interface with clap

In this book we are going to learn how to use Clap, the Command Line Argument Parser for Rust.

* [Crates.io](https://crates.io/crates/clap)
* [docs.rs](https://docs.rs/clap/latest/clap/)

There are two main ways to use Clap. Using the manual Builder and the Derive mode. In these example we'll learn about using the Builder

## First example: One parameter


In order to get started we need to add clap to the depenedencies. We can do so either by running `cargo add clap` or by editing the `Cargo.toml` file
manually adding the `[dependencies]` section if it isn't there and the row with `clap`.

The `cargo add clap` command will use the latest version number of clap. By the time your read this book this number will probably have a higher value
than what you see here.

This is the `Cargo.toml` in my example:

{% embed include file="src/examples/case1/Cargo.toml" %}


{% embed include file="src/examples/case1/src/main.rs" %}

```
$ cargo run -q
No name provided
```

```
$ cargo run -q Foo
name: "Foo"
```

```
$ cargo run -q "Foo Bar"
name: "Foo Bar"
```


```
cargo build --release
```

```
$ ./target/release/demo
No name provided
```

```
$ ./target/release/demo Foo
name: "Foo"
```

If the user does not understand what parameters are expected by this program she can run the command proving the `--help` flag or
the shorter `-h` flag and Clap will print some explanation on how to use this command line tool:


```
$ ./target/release/demo --help
Usage: demo [name]

Arguments:
  [name]

Options:
  -h, --help  Print help

```

We can achieve the same even without using the compiled binary using `cargo build`, but in that case we need to supply the `--help` flag
after a `--`.

```
cargo run -q -- --help
```

The above might look strange, but if forget to proveide the `--` separator and we write this:

```
cargo run -q --help
```

then we'll see the help of the cargo command itself. That was not our intention.


## Positional parameters

{% embed include file="src/examples/case2/src/main.rs" %}


## Named parameter

{% embed include file="src/examples/case3/src/main.rs" %}


