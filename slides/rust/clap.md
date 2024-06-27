# Command line Interface with clap
{id: clap}

## Clap - Command Line Argument Parser
{id: clap-command-line-argument-parser}

* Start by using `std::env::args`
* See [Command line arguments - ARGS](args)

* [clap](https://crates.io/crates/clap) is the Command Line Argument Parser

* [clap documentation](https://docs.rs/clap/latest/clap/)

* Derive
* Builder

## Clap Single positional argument
{id: clap-single-positional-argument}

```
cargo add clap --features derive
```

![](examples/clap/single-positional-argument/Cargo.toml)
![](examples/clap/single-positional-argument/src/main.rs)

## Clap Single long argument
{id: clap-single-long-argument}
{i: arg}
{i: long}

![](examples/clap/single-long-argument/Cargo.toml)
![](examples/clap/single-long-argument/src/main.rs)

## Clap Short arguments
{id: clap-short-arguments}
{i: short}

* We can allow to use `-x` short names (defaults to the first letter of the attribute)
* We can set the letter ourselves.

![](examples/clap/short-arguments/src/main.rs)

## Clap accepting string, number, bool - default values
{id: clap-string-number-bool-default}
{i: default_value}
{i: default_value_t}

![](examples/clap/number-string-bool/src/main.rs)


## Clap add help to each argument
{id: clap-add-help-to-each-argument}

![](examples/clap/help-with-arguments/src/main.rs)


## Clap example for CLI
{id: clap-example}
{i: CLI}


![](examples/clap/clap-example/Cargo.toml)
![](examples/clap/clap-example/src/main.rs)

TODO

* short keys
* values witout any prefix
* one long value (e.g. a long text without the need for quotes, as it is

* mutuallay exclusive options (--delete, --add, --list)


