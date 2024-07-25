# Declarative macros
{id: declarative-macros}

## Examples of standard declarative macros
{id: examples-of-declarative-macros-in-std}


* [print!](https://doc.rust-lang.org/std/macro.print.html)
* [println!](https://doc.rust-lang.org/std/macro.println.html)
* [vec!](https://doc.rust-lang.org/std/macro.vec.html)
* [todo!](https://doc.rust-lang.org/std/macro.todo.html)
* [include_str!](https://doc.rust-lang.org/std/macro.include_str.html)
* [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html)
* [unimplemented!](https://doc.rust-lang.org/std/macro.unimplemented.html)
* [matches!](https://doc.rust-lang.org/std/macro.matches.html)

* [Full list of standard macros](https://doc.rust-lang.org/std/#macros)
* [Crates tagged as macro](https://crates.io/keywords/macro)

* [Declarative macros](https://veykril.github.io/tlborm/decl-macros.html) in the Little book of macros

## todo!
{id: macro-todo}
{i: todo!}
{i: unimplemented!}

* [todo!](https://doc.rust-lang.org/std/macro.todo.html) when we put in the layout of the code but cannot work on all the codepathes at the same time and we would like to say it explicitely.
* [unimplemented!](https://doc.rust-lang.org/std/macro.unimplemented.html)

![](examples/macros/todo/src/main.rs)

```
cargo run foo
cargo run bar
```

## About Declarative macros
{id: about-declarative-macros}

* Use `macro_rules!`
* Macros look like functions with an exclamation mark `!` at the end.
* `match`-like arms to match the input of the macro.


* Macro matchers and transcribers.

## Hello World macro
{id: macro-hello-world}
{i: macro_rules!}

* The name of the macro is `hello_world`
* The `()` matches empty input.

![](examples/macros/hello-world/src/main.rs)

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


## HTML to string macro
{id: html-to-string-macro}

* [html-to-string-macro](https://crates.io/crates/html-to-string-macro)

![](examples/macros/embed-html/Cargo.toml)
![](examples/macros/embed-html/src/main.rs)


## Macro ok! to replace unwrap
{id: macro-ok}

Seen in the tests of the sqlite crate.

```
macro_rules! ok(($result:expr) => ($result.unwrap()));
```

## Optional function parameters using macros
{id: optional-function-parameters-using-macro}

![](examples/macros/optional-parameter/src/main.rs)
![](examples/macros/optional-parameter/out.out)


## Define a function using a macro
{id: define-a-function-using-a-macro}

* We can use both `tt` and `ident` in the macro parameter definition.
* We can use the macro both inside and outside functions.
* The former will create functions scoped to the block where they were created. The latter will be global.

![](examples/macros/define-function/src/main.rs)

