# Introduction to Rust
{id: rust-intro}

## Why Rust?
{id: why-rust}

* The main web site of the [Rust the language](https://www.rust-lang.org/) points out 3:

* Performance
* Reliability
* Productivity

## Why Rust - Performance
{id: rust-performance}

* Roughly at the speed of C and C++ programs
* [ruff](https://github.com/astral-sh/ruff) - a linter for Python.
* [uv](https://github.com/astral-sh/uv) - a Python package installer (replacement for [pip](https://pip.pypa.io/), [poetry](https://python-poetry.org/), [pdm](https://github.com/pdm-project/pdm)).
* [polars](https://pola.rs/) - DataFrames (replacement for [pandas](https://pandas.pydata.org/)).
* [Pydentic](https://docs.pydantic.dev/), the data validation library for Python.
* [Apache DataFusion](https://datafusion.apache.org/) - (replacement for Spark and PySpark?)
* [speed comparision](https://github.com/niklas-heer/speed-comparison)
* [Ranking programming languages by energy efficiency](https://thenewstack.io/which-programming-languages-use-the-least-electricity/)

## What is written in Rust?
{id: what-is-written-in-rust}

* [SurrealDB](https://surrealdb.com/), a multi-model database.
* ...

## Why Rust - Reliability
{id: rust-reliability}

* type system
* memory safety
* thread-safety



* Dangling pointers
* Data races
* Buffer overflow
* Integer overflow
* Iterator invalidation
* Accessing invalid addresses in memory
* Sead-locks
* memory leaks
* double-free



## Why Rust - Productivity
{id: rust-productivity}

* Tooling (IDEs, compilation, package management, dependency management, linter).
* [Documentation](https://doc.rust-lang.org/).
* 3rd party open source libraries in the [Crates registry](https://crates.io/).


## Major features of Rust
{id: major-features-of-rust}

* Shift left programming (eliminate bugs earlier in the development process)

* Speed and Open Source community

* [Corporate support](https://foundation.rust-lang.org/members/), Linux kernel

* Error handling: exceptions VS return values - returning Result
* Eliminate `null` issue.

## Rust Jobs
{id: rust-jobs}


* [Rust Jobs Report: June 2024](https://filtra.io/rust-jun-24)
* [RustJobs.dev](https://rustjobs.dev/)
* [RustJobs.com](https://www.rustjobs.com/)


## Memory allocation
{id: memory-allocation}

* Stack
* Heap
* In the binary

## Rust Books
{id: rust-books}

* [Learn Rust](https://www.rust-lang.org/learn)
* [The Rust book](https://doc.rust-lang.org/book/)
* [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
* [Rust for Rustaceans](https://rust-for-rustaceans.com/)
* [Zero To Production In Rust](https://www.zero2prod.com/)
* [Rust Atomics and Locks](https://marabos.nl/atomics/) by Mara Bos


Publishers
* [Manning](https://www.manning.com/)
* [Pragmatic Bookshelf](https://pragprog.com/)
* [O'Reilly](https://www.oreilly.com/)
* [no starch press](https://nostarch.com/)

* [Free ebooks](https://rust-ebooks.code-maven.com/)

## Crates (3rd party libraries)
{id: crates-io}

* [Crates.io](https://crates.io/) - the registry of all the crates.
* [Docs.rs](https://docs.rs/) - the documentation of all the crates.
* [Lib.rs](https://lib.rs/) - Lightweight, opinionated, curated, unofficial alternative to crates.io.
* [Rust Digger](https://rust-digger.code-maven.com/) - analyzing crates and recommending improvements.

## Rust exercises with feedback
{id: rust-exercises-with-feedback}

* [rustlings](https://rustlings.cool/)
* [Exercism Rust track](https://exercism.org/tracks/rust)
* [Rust users forum](https://users.rust-lang.org/)

## Podcast, newsleter
{id: rust-podcast}

* [This week in Rust](https://this-week-in-rust.org/) - a weekly newsletter
* [Rustacean](https://www.rustaceans.org/)
* [Rustacean station](https://rustacean-station.org/) - a podcast


## Other Rust learning resources
{id: other-rust-learning-resources}

* [Let's Get Rusty](https://letsgetrusty.com/)
* [noboilerplate](https://github.com/0atman/noboilerplate)
* [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
* [Rust Atomics and Locks](https://marabos.nl/atomics/) Low-Level Concurrency in Practice by Mara Bos.
* [Learning Rust](https://www.youtube.com/channel/UC7OY971Es9P_AcXGYKms87A) by Ian Walker.
* [teach-rs](https://github.com/tweedegolf/teach-rs) Free material for Rust courses.
* [Easy Rust](https://dhghomon.github.io/easy_rust/)



## Rust in other languages
{id: rust-in-other-languages}

* [From Perl to Rust](https://oylenshpeegul.gitlab.io/from-perl-to-rust/introduction.html)

* [The Rust book in Hebrew](https://github.com/IttayWeiss/rustbook-heb)
* [All the languages](https://doc.rust-lang.org/stable/book/appendix-06-translation.html)

## Articles about Rust
{id: articles-about-rust}

* [Why do Programmers Love Rust?](https://www.youtube.com/watch?v=vBsEF-anSLY) presentation by Dave Rolsky [slides](https://presentations.houseabsolute.com/why-do-programmers-love-rust/) - [source](https://github.com/autarch/presentations/tree/master/why-do-programmers-love-rust)
* [Why the developers who use Rust love it so much](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/)

## Rust community
{id: rust-community}

* [Rust User groups](https://rust.code-maven.com/user-groups)
* [Rust social status](https://rust.code-maven.com/rust-update-2024-06-17)

## Demo None handling
{id: demo-none-handling}

![](examples/intro/no-null/src/main.rs)

## Demo None handling with Option
{id: demo-none-handling-with-option}
{i: Option}
{i: match}
{i: Some}
{i: None}

![](examples/intro/demo-none-handling/src/main.rs)

See original idea on [What is Rust and why is it so popular?](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)

## Demo error handling with Result
{id: demo-error-handling-with-result}
{i: Result}
{i: match}
{i: Ok}
{i: Err}

![](examples/intro/demo-result/src/main.rs)

![](examples/intro/demo-result/out.out)


## Demo error handling with Result and question mark
{id: demo-error-handling-with-pass-through}
{i: Result}
{i: dyn}
{i: Box}

![](examples/intro/demo-error-handling/src/main.rs)



