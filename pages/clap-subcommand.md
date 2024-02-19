---
title: Clap - subcommands for command line applications in Rust
timestamp: 2024-01-11T11:11:01
author: szabgab
published: true
description: Clap allows the creation of subcommands where each subcommand can have its own separate set of parameters.
tags:
    - Clap
    - CLI
    - Subcommand
todo:
    - TODO
---

[Clap](/clap) allows the definition of subcommand. Each subcommand can have its own separate set of parameters.

Many common tools work like this. For example `git` or [gh](https://cli.github.com/) or for that matter our beloved `cargo`.

## Dependencies

Nothing fancy here. We use the derive API as in all the other examples, so we need that feature.

{% include file="examples/clap/subcommand/Cargo.toml" %}


## Define the common rules for the Parser

The application will have one shared parameter called `--root` that defaults to the current directory.

It also has field called `command` which is a `subcommand`. The name `Commands` is a user-defined name of an `enum` we'll see in a bit.
You could use `Blabla` instead if, you wanted to confuse the readers of your code.

```rust
#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}
```

## Define the subcommands

This is where we define the `enum` called `Commands` and we derive it from [Subcommand](https://docs.rs/clap/latest/clap/trait.Subcommand.html).

For each Subcommand we can defined the parameters in the same manner as we did it in the [other examples about Clap](/clap).

In our case the `Web` subcommand will have a required parameter called `--outdir` and the `Email` subcommand will have a required parameter called `--to`.


```rust
#[derive(Subcommand)]
enum Commands {
    Web {
        #[arg(long)]
        outdir: PathBuf,
    },
    Email {
        #[arg(long)]
        to: String,
    },
}
```

## Parsing the arguments

We call `Cli::parse` to parse the arguments and then we can print the values extracted from the command line.

First we print the value of the shared `root` parameter.

Then we need a pattern matching using `match`. We need one arm for each one of the subcommands. Because we made the subcommand itself
optional, we'll need a default arm to handle the `None` case.


```rust
fn main() {
    let args = Cli::parse();

    println!("root: {:?}", args.root);


    match &args.command {
        Some(Commands::Web { outdir }) => {
            println!("outdir: {:?}", outdir);
        },
        Some(Commands::Email { to }) => {
            println!("to: {}", to);
        }
        None => {
            println!("There was no subcommand given");
        }
    }
}
```

## How does this look like to the users?

If we are running the code with `cargo run` we have to put an extra `--` between the command and the parameters, but that's not how the end-user will see it.
So let me first build a released version of the code and show the usage that way:

### Build the released version

```
cargo build --release
```

Because the name of the crate I used for demonstration is `subcommand`, this will create a file called `./target/release/subcommand`.

### Running without parameters

This will run, because the the subcommand were made optional: `Option<Commands>`

```
./target/release/subcommand
root: "."
There was no subcommand given
```

### Getting help

We can provide either `help` or `--help` or `-h` and we'll get the same output:

```
$ ./target/release/subcommand help

Usage: subcommand [OPTIONS] [COMMAND]

Commands:
  web
  email
  help   Print this message or the help of the given subcommand(s)

Options:
      --root <ROOT>  [default: .]
  -h, --help         Print help
```

### Getting subcommand specific help


```
./target/release/subcommand web -h
Usage: subcommand web --outdir <OUTDIR>

Options:
      --outdir <OUTDIR>
  -h, --help             Print help
```


```
$ ./target/release/subcommand email -h
Usage: subcommand email --to <TO>

Options:
      --to <TO>
  -h, --help     Print help
```

### Calling with parameters

Note, the common parameter that we have defined in the top-most `struct` has to be passed **before** the subcommand.

The `email` subcommand:

```
./target/release/subcommand --root ~/home email --to foo@bar.com
root: "/home/gabor/home"
to: foo@bar.com
```

And the `web` subcommand

```
./target/release/subcommand --root ~/home web --outdir ~/target
root: "/home/gabor/home"
outdir: "/home/gabor/target"
```

## The Full Code

{% include file="examples/clap/subcommand/src/main.rs" %}

## Conclusion

Armed with this example I can now improve the code that generates the Rust Maven web site.

