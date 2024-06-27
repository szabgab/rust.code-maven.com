# Command line Interface with clap
{id: clap}

## Clap - Command Line Argument Parser
{id: clap-command-line-argument-parser}

* Start by using `std::env::args` [Command line arguments - ARGS](/args)

* [clap](https://crates.io/crates/clap)

## Clap Single positional argument
{id: clap-single-positional-argument}

```
cargo add clap --features derive
```

![](examples/clap/single-positional-argument/Cargo.toml)
![](examples/clap/single-positional-argument/src/main.rs)


## Clap Single long argument
{id: clap-single-long-argument}

![](examples/clap/single-long-argument/Cargo.toml)
![](examples/clap/single-long-argument/src/main.rs)

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


