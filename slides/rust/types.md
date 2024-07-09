# Rust types
{id: types}


## Rust Types
{id: rust-types}

* Integers: `i8`, `i16`, `i32`, `i64`, `i128`.
* Unsigned integers `u8`, `u16`, `u32`, `u64`, `u128`.
* Floating point: `f32`, `f64`.
* [minimum and maximum values of numeric types](https://rust.code-maven.com/minimum-and-maximum-values-of-numeric-types)
* [size of integers in Python](https://python.code-maven.com/size-of-integer-in-python)

* Boolan: `bool`: `true`, `false`.
* Char: 



## Showing type
{id: showing-type}

* Using the rust-analyzer will make your IDE show the type. Eg. in Visual Studio Code.
* A trick that you can also use is to specify some type at the type of assignment, e.g. `()`, the [unit](https://doc.rust-lang.org/std/primitive.unit.html).
* Then the compiler will complain during compilation.

```
let x: () = ...
let readdir: () = std::path::Path::new(".").read_dir().unwrap();
```


## Rust numerical operations
{id: rust-numerical-operations}

![](examples/types/numerical-operations/src/main.rs)

## Rust numerical types
{id: rust-numerical-types}
{i: i32}
{i: i64}

* By default numbers are stored in i32 whose range is -2147483648..=2147483647
* We can explicitely put numbers in different types

![](examples/types/number-types/src/main.rs)

![](examples/types/numbers/src/main.rs)

* List of Rusts [primitive types](https://doc.rust-lang.org/core/primitive/index.html)

## Rust type mismatch in numerical operation
{id: rust-type-mismatch-in-numerical-operation}

![](examples/types/type-mismatch/src/main.rs)

* TODO: if we remove the i32 then this works even though, I think, the default is i32


## Rust Overflow
{id: rust-overflow}

![](examples/types/overflow/src/main.rs)
![](examples/types/overflow/out.out)

## Print type of variable
{id: print-type-of-variable}
{i: type_name}
{i: type_name_of_val}

* Sometimes during development, during the exploration of Rust it is useful to print the type of a varible.
* This is one way to do it.
* It will be added to the stanard library as [type_name_of_val](https://doc.rust-lang.org/stable/core/any/fn.type_name_of_val.html).

![](examples/types/print-type/src/main.rs)

![](examples/types/print-type/out.out)


