# Declarative macros
{id: declarative-macros}

## Examples of standard declarative macros
{id: examples-of-declarative-macros-in-std}


* [print!](https://doc.rust-lang.org/std/macro.print.html)
* [println!](https://doc.rust-lang.org/std/macro.println.html)
* [eprint!](https://doc.rust-lang.org/std/macro.eprint.html)
* [eprintln!](https://doc.rust-lang.org/std/macro.eprintln.html)
* [format!](https://doc.rust-lang.org/std/macro.format.html)

* [vec!](https://doc.rust-lang.org/std/macro.vec.html) - create a vector.

* [todo!](https://doc.rust-lang.org/std/macro.todo.html)
* [unimplemented!](https://doc.rust-lang.org/std/macro.unimplemented.html)

* [include_str!](https://doc.rust-lang.org/std/macro.include_str.html)
* [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html)

* [Full list of standard macros](https://doc.rust-lang.org/std/#macros)
* [Crates tagged as macro](https://crates.io/keywords/macro)

* [Declarative macros](https://veykril.github.io/tlborm/decl-macros.html) in the Little book of macros

## todo! for and if-condition
{id: macro-todo}
{i: todo!}

* [todo!](https://doc.rust-lang.org/std/macro.todo.html) when we put in the layout of the code but cannot work on all the codepathes at the same time and we would like to say it explicitely.

![](examples/macros/todo/src/main.rs)

```
cargo run foo
cargo run bar
```

## todo! for a match
{id: tod-for-a-match}
{i: todo!}

![](examples/macros/todo-match/src/main.rs)

```
$ cargo run -q foo
We are handling foo

$ cargo run -q bar
We are handling bar


$ cargo run -q qqrq
thread 'main' panicked at src/main.rs:15:17:
not yet implemented: We still need to implement for qqrq
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


## unimplemented
{id: unimplemented}
{i: unimplemented!}

* Very similar to `todo!` without the vague promise that it will be implemented.
* [unimplemented!](https://doc.rust-lang.org/std/macro.unimplemented.html)


## About Declarative macros
{id: about-declarative-macros}

* They are also called [Macros-By-Example](https://doc.rust-lang.org/reference/macros-by-example.html) in the reference book.
* We use `macro_rules!` to defined them.
* When used, declarative macros look like functions with an exclamation mark `!` at the end.
* `match`-like arms to match the input of the macro.

* Macro matchers and transcribers (aka. expansions).


## Hello World macro
{id: macro-hello-world}
{i: macro_rules!}

* The name of the macro is `hello_world`
* The `()` matches empty input.

![](examples/macros/hello-world/src/main.rs)

## Macro with literal values
{id: macro-with-literal-values}

![](examples/macros/macro-literal-values/src/main.rs)


## Macro with parameter to say hello
{id: macro-say-hello}
{i: macro_rules!}
{i: expr}

* The macro is called `say_hello`.
* parameters of the macro
* `$name: expr`    means that the macro is expecting an `expr` (expression) and it will be assigned to a variable called `$name`.


// $t:ty          means we have a paramerer called $t    and it has a type "ty" (type, such as i32 or f64)
// With this macro we can replace a short syntax with a longer syntax in at compile time.


![](examples/macros/say-hello/src/main.rs)


## Fragment specifiers
{id: macro-fragment-specifiers}

* block
* expr
* ident
* ...

[fragment-specifiers](https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html)


## Macro with literal
{id: macro-with-literal}
{i: literal}

![](examples/macros/macro-with-literal/src/main.rs)

## Macro with ident - create function
{id: macro-with-ident}

* `ident` matches an [identifier](https://doc.rust-lang.org/reference/identifiers.html) (e.g variable names, function names)

![](examples/macros/macro-create-function/src/main.rs)


## Macro with optional parameter to say hello
{id: macro-with-optional-parameter}

* In this macro we have two "arms". The first will handle the case when no paramater is passed.
* We can have separate implementations or we can recursively use the macro.

![](examples/macros/say-hello-optional/src/main.rs)


## Macro with many parameters to say hello
{id: macro-say-hello-many-times}
{i: macro_rules!}
{i: expr}
{i: "*"}
{i: "+"}

* This macro can accept 0 or more parameters and then it will repeate the code as many times as parameters we got.
* Instead of `*` we could use `+` in the declaration and that would mean the macro accepts 1 or more parameters.

![](examples/macros/say-hello-many-times/src/main.rs)
![](examples/macros/say-hello-many-times/out.out)

## Macro to create a HashMap to be a counter
{id: macro-to-crate-a-hashmap}
{i: HashMap}

![](examples/macros/create-counter-hash/src/main.rs)
![](examples/macros/create-counter-hash/out.out)

## Macro prt! to explore memory allocation
{id: macro-prt-to-explore-memory-allocation}

![](examples/macros/macro-prt/src/main.rs)
![](examples/macros/macro-prt/out.out)

## Macro String::from
{id: macro-string-from}

![](examples/macros/macro-string-from/src/main.rs)

## Macro ok! to replace unwrap
{id: macro-ok}

Seen in the tests of the [sqlite crate](https://crates.io/crates/sqlite).

![](examples/macros/macro-ok/src/main.rs)

## Macro pair! to create tuple with Some
{id: macro-pair}

Seen in the [sqlite crate](https://crates.io/crates/sqlite).

![](examples/macros/macro-sqlite-pair/src/main.rs)

## Optional function parameters using macros
{id: optional-function-parameters-using-macro}

* Rust does not allow the declaration of the same function-name with different signatures as many other languages.
* Rust does not allow the definition of a function with optional parameters and/or with default values.

* We can use declarative macros to overcome this limitation.

* We could define a macro "only for the case with the optional value" or, as we do in this example, we can create a macro for both cases.

![](examples/macros/optional-parameter/src/main.rs)
![](examples/macros/optional-parameter/out.out)


## Debugging macros using trace_macros
{id: debuging-macros-using-trace-macros}
{i: trace_macros}

* Install Nightly chain

```
rustup toolchain update nightly
```

```
cargo +nightly run
```

## Define a function using a macro
{id: define-a-function-using-a-macro}

* We can use both `tt` and `ident` in the macro parameter definition.
* We can use the macro both inside and outside functions.
* The former will create functions scoped to the block where they were created. The latter will be global.

![](examples/macros/define-function/src/main.rs)

## Error logging
{id: macro-error-logging}

![](examples/macros/error-logging/src/main.rs)

## Some crates that provide macros
{id: crates-that-provide-macros}

* [Crates tagged as macro](https://crates.io/keywords/macro)

* HTML

## HTML to string macro
{id: html-to-string-macro}

* [html-to-string-macro](https://crates.io/crates/html-to-string-macro)

![](examples/macros/embed-html/Cargo.toml)
![](examples/macros/embed-html/src/main.rs)


