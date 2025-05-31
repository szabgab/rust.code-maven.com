# Limit the number of values for a vector argument

* num_args
* Vec
* required

* A different approach is to accept multiple values with a single mention of the argument. We can achieve this by setting the `num_args`.
* It can accept either a single number or a range such as `2..=3` (2 or 3), `..=3` (same as 0..=3), or `3..` (3 or more).
* We can also supply `required=true` to make sure the argument must be supplied.
* See [num_args](https://docs.rs/clap/latest/clap/struct.Arg.html#method.num_args)


{% embed include file="src/examples/clap/limit-number-of-args/src/main.rs" %}

```
$ cargo run -q
error: the following required arguments were not provided:
  --animal <ANIMAL> <ANIMAL>...

Usage: limit-number-of-args --animal <ANIMAL> <ANIMAL>...

For more information, try '--help'.


$ cargo run -q -- --animal Cat Dog
Cli { animal: ["Cat", "Dog"], sisters: [] }


$ cargo run -q -- --animal Cat Dog Crab
Cli { animal: ["Cat", "Dog", "Crab"], sisters: [] }


$ cargo run -q -- --animal Cat Dog Crab Snake
error: unexpected argument 'Snake' found

Usage: limit-number-of-args [OPTIONS] --animal <ANIMAL> <ANIMAL>...

For more information, try '--help'.
```


