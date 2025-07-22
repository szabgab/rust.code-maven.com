# Demo Option to handle None

Let's see another, probably a bit more realistic, example of this situation.

In this example when we declare the variable `input` we also initialize it to `None`.

Then we collect the values from the command line into a variable called `args`. Let's disregard the details of this for now. I wanted to create a working example
for the None-handling and this is the simplest I managed to do.

If there are more than 1 values in that `args` variable, meaning if the user supplied at least one value after the name of the program then we assign this value
to the `input` variable. However, we don't just assign the value as it arrived from the user, we wrap it in the `Some()` expression. It might look like a function,
but it is not really one. It just wraps the value. We'll discuss the details later.

So now, after the `if`-statement we either have `None` in the `input` variable or we have `Some(value)` in there.

{% embed include file="src/examples/introduction/demo-option/src/main.rs" %}

In order to use it this variable properly Rust makes us check if the content of the variable is `None` or `Some(value)`.
Rust has many nice ways to do this, probably the clearest is using he `match` pattern-matching statement.

We know that the `input` variable has two possible states hence we need two "arms" in the `match` statement. One will be triggered if the variable contains `Some(value)`
the other when the variable contains `None`. In both cases we can provide a block of code to be executed.

In our example we provided a variable called `thing`, yes that is a variable we can use any name there. The value provided by the user that was later wrapped in the `Some()` construct
will be assigned to this variable for the duration of the block after the fat-arrow.

## Usage

We can provide the command-line parameters right after we typed in `cargo run`:

```
$ cargo run
There was no input

$ cargo run hello
The input is hello
```

## Some extra notes

* In this example we had to use the `mut` keyword to make the variable mutable.
* In this case, because each **arm** of the `match` statement has only one statement (a `println!` in both cases) we could get-by without the curly braces.
