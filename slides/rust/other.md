# Other
{id: other}

## Variable shadowing
{id: variable-shadowing}
{i: let}

* We can declare the same variable multiple time using the `let` keyword.
* The 2nd and subsequent declarations hide (shadow) the previous ones.
* If this happened inside a block then when the shadowing version of the variable goes out of scope the previous value becomes visible again. (Observe, the last line being 6.)

![](examples/other/shadowing/src/main.rs)
![](examples/other/shadowing/out.out)

## String formatting
{id: string-formatting}
{i: format}
{i: sprintf}

![](examples/other/string-formatting/src/main.rs)

![](examples/other/reverse/src/main.rs)
![](examples/other/reverse/out.out)

![](examples/other/collect/src/main.rs)

## Factorial functions returning Result
{id: factorial-function-returning-result}
{i: Restult}
{i: Ok}
{i: Err}

![](examples/other/factorial-returning-result/src/main.rs)
![](examples/other/factorial-returning-result/out.out)

## Split function into files
{id: split-functions-into-file}

![](examples/other/project/src/main.rs)
![](examples/other/project/src/helper.rs)
![](examples/other/project/Cargo.toml)

## Variable Scope in Rust
{id: rust-variable-scope}

* Every block creates a scope

![](examples/other/scope/src/main.rs)
![](examples/other/scope/out.out)

## Declare empty variable and assign to it later
{id: assign-to-variable-later}

![](examples/other/assign-later/src/main.rs)


## Declare empty variable - requires type
{id: declare-empty-variable}

![](examples/other/empty-string/src/main.rs)
![](examples/other/empty-string/out.out)


## SystemTime now
{id: systemtime-now}

![](examples/other/now/src/main.rs)

## Exit
{id: exit}
{i: exit}

* [Function std::process::exit](https://doc.rust-lang.org/std/process/fn.exit.html)

![](examples/other/exit/src/main.rs)

```
echo $?
echo %ERROR_LEVEL%
```

## Define multiple variables
{id: define-multiple-variables}

![](examples/other/define-multiple-variables/src/main.rs)

## wc
{id: wc}

![](examples/other/rust-wc/Cargo.toml)
![](examples/other/rust-wc/src/main.rs)

## copy vs clone
{id: copy-vs-clone}

TODO
* [What’s the difference between Copy and Clone?](https://doc.rust-lang.org/std/marker/trait.Copy.html#whats-the-difference-between-copy-and-clone)

## Type alias
{id: type-alias}
{i: type}

* We can use the `type` keyword to create aliases to existing types. This can help us in reading the code, but Rust does not do any enforcement.
* As you can see in the following example we can pass arguments to a "different" type as long as it is an alias to the same type.
* [type](https://doc.rust-lang.org/std/keyword.type.html)

![](examples/other/type-alias/src/main.rs)
![](examples/other/type-alias/out.out)

## Struct and type alias - Polygon
{id: struct-and-type-alias}
{i: struct}
{i: type}

* The simplest way to represent a polygon (a series of points) is a vector of `Point` instances.
* We can even give it a name using the [type](https://doc.rust-lang.org/std/keyword.type.html) keyword.
* Despite its name it does **not** create a new type, just an alias.
* That's why we cannot use `impl` to add a method.

![](examples/struct/polygon-type/src/main.rs)
![](examples/struct/polygon-type/out.out)


## Solution: Age limit
{id: solution-age-limit}

![](examples/other/age-limit-18/src/main.rs)
![](examples/other/age-limit-18-21/src/main.rs)
![](examples/other/age-limit-anywhere/src/main.rs)

## Multi-counter in manually handled CSV file
{id: multi-counter-in-manually-handled-csv-file}


![](examples/other/multi_counter_with_manual_csv/src/main.rs)


## Get path to current executable
{id: get-path-to-current-executable}
{i: current_exe}

![](examples/other/current-executable/src/main.rs)

## cache dependencies with sccache
{id: cache-dependencies}

* [sccache](https://github.com/mozilla/sccache)

## Commafy
{id: commafy}

![](examples/other/commafy/Cargo.toml)
![](examples/other/commafy/src/lib.rs)

## Commafys
{id: commafy2}

![](examples/other/commafy2/src/main.rs)


## Use statements
{id: use-statements}

![](examples/other/use-statements/src/main.rs)

## Take version number from Cargo.toml
{id: version-number-from-cargo-toml}
{i: VERSION}
{i: CARGO_PKG_VERSION}


![](examples/other/version-number/src/main.rs)

## Ansi colors
{id: ansi-colors}

* [ansi_term](https://crates.io/crates/ansi_term)

![](examples/other/ansi-colors/Cargo.toml)
![](examples/other/ansi-colors/src/main.rs)

## What I learned from learning Rust
{id: what-i-learned-from-learning-rust}

* Gabor Szabo
* https://szabgab.com/
* https://github.com/szabgab

* https://rustatic.code-maven.com/
* https://rust-digger.code-maven.com/
* https://rust.code-maven.com/

## Temperature converter
{id: tempreature-converter}

![](examples/other/temperature-converter/Cargo.toml)
![](examples/other/temperature-converter/input.json)
![](examples/other/temperature-converter/src/main.rs)

## Check slides
{id: check-slides}

![](examples/other/check-slides/src/main.rs)


## Expressions vs statements
{id: expressions-vs-statement}

* Expressions have a return value do NOT need a trailing semi-colon
* Statements do not have values and need a semi-colon

![](examples/other/statements/src/main.rs)

## Send email via SendGrid
{id: send-email-via-sendgrid}

![](examples/other/send-mail-with-sendgrid/src/main.rs)

![](examples/other/send-mail-with-sendgrid/Cargo.toml)

With a file called config.txt in the same directy that has:

```
SENDGRID_API_KEY=SG....
```

## Equality of Some - values
{id: equality-of-some-values}
{i: Some}

![](examples/other/some-equality/src/main.rs)
![](examples/other/some-equality/out.out)

## Fork
{id: fork}

* [fork](https://docs.rs/fork/)

![](examples/other/show-forking/Cargo.toml)
![](examples/other/show-forking/src/main.rs)

TODO: wait, waitpid?

## sysinfo - Which Operating System are we running on?
{id: which-operating-system}
{i: systinfo}
{i: kernel_version}
{i: os_version}

* [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)

![](examples/other/system-info/Cargo.toml)

![](examples/other/system-info/src/main.rs)

![](examples/other/system-info/out.out)

## Operating system information with os_info
{id: operating-system}
{i: os_info}
{i: os_type}
{i: architecture}

* [os_info](https://crates.io/crates/os_info)

![](examples/other/os-information/Cargo.toml)

![](examples/other/os-information/src/main.rs)

![](examples/other/os-information/out.out)


## Parse string to Rust expression using syn
{id: parse-string-to-rust-expression}
{i: syn}
{i: parse_str}

![](examples/other/syn-parse-str/src/main.rs)
![](examples/other/syn-parse-str/out.out)

## Parse HTML
{id: parse-html}
{i: Html}
{i: parse}
{i: parse_document}
{i: CSS}
{i: Selector}
{i: attr}
{i: inner_html}

* [scraper](https://crates.io/crates/scraper)

![](examples/other/parse-html/src/main.rs)

## Fix URL parameter
{id: fix-url-parameter}
{i: trim_end_matches}
{i: url}

* The user can provide a URL, but I would like to be flexible and accept both with and without a trailing slash:
* https://rust.code-maven.com
* https://rust.code-maven.com/

At first I tried some over-engineered solutions, till I got the recommendation to use `trim_end_matches`.

![](examples/other/fix-url-string/src/main.rs)

However using the [url](https://crates.io/crates/url) crate might be the best solution in this case:

![](examples/other/fix-url/Cargo.toml)
![](examples/other/fix-url/src/main.rs)

## My little Rust runner
{id: my-little-rust-runner}

* This is especially useful for the slides so I can create individual Rust example files and run them stand alone.

![](rust.sh)


```
./rust.sh examples/intro/hello.rs
```


## Crate library and use local library
{id: use-local-library}
{i: dependencies}
{i: path}


* Create a library:

```
cargo new add-lib --lib
```

The files created:

![](examples/libraries/add-lib/Cargo.toml)
![](examples/libraries/add-lib/src/lib.rs)

* Create another crate that will use this library:

```
cargo new add-app
cd add-app
cargo add --path ../add-lib/
```

![](examples/libraries/add-app/Cargo.toml)
![](examples/libraries/add-app/src/main.rs)


## Ordered floats
{id: ordered-floats}
{i: TODO}

* [ordered-float](https://crates.io/crates/ordered-float)

## Linked list using struct and Box
{id: linked-list-using-struct}
{i: Box}

![](examples/other/linked-list-in-struct/src/main.rs)

![](examples/other/linked-list-in-struct/out.out)

## Undirected graph
{id: undirected-graph}


* [Seven Bridges of Königsberg](https://en.wikipedia.org/wiki/Seven_Bridges_of_K%C3%B6nigsberg)

![](examples/other/undirected-graph/src/main.rs)

## Memory leak
{id: memory-leak}

In this example we try to implement a memory leak as we demonstrate [in Python](https://code-maven.com/slides/python/memory-leak)
showing how Rust makes us avoid it. So far this code has compilation errors.

![](examples/other/memory-leak/src/main.rs)

## Debug assertions
{id: debug-assertions}

![](examples/other/debug-assertions/src/main.rs)

```
cargo run

cargo run --release
```

## Add method to HashMap that sums the values
{id: add-method-to-hashmap-that-sums-the-values}
{i: HashMap}
{i: TBD}

![](examples/hashes/total-values-method/src/main.rs)
![](examples/hashes/total-values-method/out.out)

## Passing string to function
{id: passing-string-to-function}
{i: TBD}

![](examples/other/passing-string-to-function/src/main.rs)

## Struct duplicate
{id: struct-duplicate}

* This seems to be an old example showing that if we don't compose one struct from another then we have to implement everything in both cases.
* In this case the `Circle` struct has its own x and y attribute and its own `mv` method.

![](examples/struct/circle-duplicate/src/main.rs)
![](examples/struct/circle-duplicate/out.out)

## Multiple referene to a struct
{id: multiple-reference-to-a-struct}

![](examples/struct/multiple-referene-to-struct/src/main.rs)

## Print struct (Point)
{id: print-struct-point}
{i: std::fmt::Display}
{i: Display}

![](examples/struct/print-point/src/main.rs)
![](examples/struct/print-point/out.out)

## Debug struct (Point)
{id: debug-struct-point}
{i: std::fmt::Debug}
{i: Debug}

![](examples/struct/debug-point/src/main.rs)
![](examples/struct/debug-point/out.out)

## Num traits
{id: num-traits}


* [num-traits](https://crates.io/crates/num-traits)
* [num-traits](https://docs.rs/num-traits)

## map with threads with Mutex
{id: map-with-thread-mutex}
{i: Mutex}
{i: TBD}

![](examples/threads/map-thread-suggested/src/main.rs)

## Accessing envrionment variables
{id: accessin-environment-variables}
{i: std::env}
{i: std::env::var}

We can access the environment variables from Rust using the [std::env::var](https://doc.rust-lang.org/std/env/fn.var.html) function.
It returns a `Result`.

![](examples/other/get-environment-variable/src/main.rs)

Run as

```
cargo run
RUST=42 cargo run
```

## List environment variables
{id: list-environment-variables}
{i: std::env}
{i: std::env::vars}

![](examples/other/environment-variables/src/main.rs)

## Closures
{id: closures}

![](examples/other/closure-demo/src/main.rs)

## Reference to a number
{id: reference-to-a-number}
{i: TBD}

![](examples/other/reference-to-number/src/main.rs)

## Out of memory
{id: out-of-memory}

This program will create an ever growing string. When it reaches the total size of the (free) memory in the computer it crashes.

![](examples/other/out-of-memory/src/main.rs)

## Two leveles of modules
{id: modules-two-levels-of-modules}

![](examples/modules/two-levels-of-modules/src/main.rs)


## Try packages
{id: try-packages}

![](examples/modules/try-packages/src/main.rs)

## release.toml
{id: release-toml}

* More than 500 crates have a file called `release.toml` in their git repository and 2 have `.release.toml`.
* These are configuration files of the [cargo-release](https://crates.io/crates/cargo-release) crate to make releases easier.

## thousands crate for struct with Display (commafy)
{id: thousands-crate-for-struct-with-display}
{i: thousands}
{i: commafy}

![](examples/other/thousands-for-struct/src/main.rs)
![](examples/other/thousands-for-struct/out.out)


