# Clap: arbitraty function to validate argument

* value_parser


{% embed include file="src/examples/clap/even-number/src/main.rs" %}

```
$ cargo run -q -- --number 22
Cli { number: 22 }

$ cargo run -q -- --number 23
error: invalid value '23' for '--number <NUMBER>': An even number is expected

For more information, try '--help'.



$ cargo run -q -- --number ad
error: invalid value 'ad' for '--number <NUMBER>': invalid digit found in string

For more information, try '--help'.
```


