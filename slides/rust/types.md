# Rust types
{id: types}

## Rust Types
{id: rust-types}

* Integers: `i8`, `i16`, `i32`, `i64`, `i128`. `isize` (on 32 bit machines this 32 bit, on 64 bit machines this is 64 bit).
* Unsigned integers `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.
* Floating point: `f32`, `f64`.
* [minimum and maximum values of numeric types](https://rust.code-maven.com/minimum-and-maximum-values-of-numeric-types)
* [size of integers in Python](https://python.code-maven.com/size-of-integer-in-python)

* Boolean: `bool`: (`true`, `false`).
* Char: `char`
* String: `str`

* String
* Array
* Vector
* HashMap
* Struct
* Enum

* List of Rusts [primitive types](https://doc.rust-lang.org/core/primitive/index.html)

## Showing type
{id: showing-type}

* Using the rust-analyzer will make your IDE show the type. Eg. in Visual Studio Code.
* A trick that you can also use is to specify some type at the type of assignment, e.g. `()`, the [unit](https://doc.rust-lang.org/std/primitive.unit.html).
* Then the compiler will complain during compilation.

```
let x: () = ...
let readdir: () = std::path::Path::new(".").read_dir().unwrap();
```

## Print type of variable
{id: print-type-of-variable}
{i: type_name}
{i: type_name_of_val}

* Sometimes during development, during the exploration of Rust it is useful to print the type of a varible.
* This is one way to do it.
* It will be added to the stanard library as [type_name_of_val](https://doc.rust-lang.org/stable/core/any/fn.type_name_of_val.html).

![](examples/types/print-type/src/main.rs)

![](examples/types/print-type/out.out)


