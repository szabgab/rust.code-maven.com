# case on type-name (match-like) (variant of enum)

What if we have an `enum` and we would like to display different things based on the variant of the `enum`? We can use the `case` and `when` keywords. They are not as powerful as the `match` operator in Rust,
but they are useful in this situation.

## Cargo.toml

We need both `liquid` and `serde` with the `derive` feature as dependencies:

{% embed include file="src/examples/liquid/liquid-case-on-type-name/Cargo.toml" %}

## Code

In this example too I used a template embedded in the Rust code. In a real appliation I'd put the templates in separate files, but in such an example it is easier to have just one file.

As you can see the `Color` `enum` has 3 variants, but the template only handle 2

{% embed include file="src/examples/liquid/liquid-case-on-type-name/src/main.rs" %}


## Output

{% embed include file="src/examples/liquid/liquid-case-on-type-name/out.out" %}

