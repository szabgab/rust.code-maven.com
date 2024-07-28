# Procedural macros
{id: procedural-macros}


## Procedural macros
{id: procedural-macros-intro}
{i: proc-macro}
{i: TokenStream}

* [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)
* Apparently the Macro needs to be in its own Crate separate from where it is being used.

* Add `proc-macro` to the `Cargo.toml`

* Here we show that the macro is executed during the compilation.
* It needs to return a TokenStream.

![](examples/macros/hello-world-macro/Cargo.toml)
![](examples/macros/hello-world-macro/src/lib.rs)

![](examples/macros/hello-world-use/Cargo.toml)
![](examples/macros/hello-world-use/src/main.rs)


## Macro accepting a single string
{id: macro-accepting-a-single-string}

![](examples/macros/say-hello-macro/src/lib.rs)
![](examples/macros/say-hello-use/out.out)
![](examples/macros/say-hello-use/src/main.rs)


## Accept list of values as a plain string in a macro
{id: accept-list-of-values-as-a-plain-string}

![](examples/macros/say-hello-many-times-macro/Cargo.toml)
![](examples/macros/say-hello-many-times-macro/src/lib.rs)

![](examples/macros/say-hello-many-times-use/Cargo.toml)
![](examples/macros/say-hello-many-times-use/src/main.rs)
![](examples/macros/say-hello-many-times-use/out.out)


## Compile random number in the code using macro
{id: compile-random-number-on-the-code}

![](examples/macros/random-constant-macro/Cargo.toml)
![](examples/macros/random-constant-macro/src/lib.rs)

![](examples/macros/random-constant/Cargo.toml)
![](examples/macros/random-constant/src/main.rs)

TODO: #[macro_export]

## SQLx - compile-time SQL queries
{id: sqlx-compile-time-sql-queries}

TODO
* [SQLx](https://crates.io/crates/sqlx)



