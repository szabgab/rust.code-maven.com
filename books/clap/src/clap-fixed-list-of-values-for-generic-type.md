# Clap: fixed list of valid values for generic type

* value_parser

* Besides creating an enum we could also just list the possible value of a generic type using the [value_parse](https://docs.rs/clap/latest/clap/struct.Arg.html#method.value_parser) option.

{% embed include file="src/examples/clap/value-parser-fixed-list/src/main.rs" %}

```
$ cargo run -q -- -h
Usage: value-parser-fixed-list --animal <ANIMAL>

Options:
      --animal <ANIMAL>  [possible values: cat, dog, crab]
  -h, --help             Print help



$ cargo run -q
error: the following required arguments were not provided:
  --animal <ANIMAL>

Usage: value-parser-fixed-list --animal <ANIMAL>

For more information, try '--help'.



$ cargo run -q -- --animal cat
Cli { animal: "cat" }


$ cargo run -q -- --animal snake
error: invalid value 'snake' for '--animal <ANIMAL>'
  [possible values: cat, dog, crab]

For more information, try '--help'.
```


