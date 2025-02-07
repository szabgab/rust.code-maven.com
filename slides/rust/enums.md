# Enum
{id: enum}

## Why enums
{id: why-enums}

* [Defining an enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

* An enum type has `variants`.
* `match` pattern matching operations must be exhaustive, this helps us ensuring that we handled every variant.

## Enum to represent exit status
{id: enum-to-represent-exit-status}

* We define a **type** called `ExitCode` that has two **variants**: `Success` and `Failure`.
* We can then, based on the results of our porgram set the value of exit.

* However, this is a bit limited as we can only indicate failure without the details we are used to - which is a number.

* By default Rust does not implement the `Debug` trait for an arbitrary `enum` so we `derive` from the `Debug` trait to be able to print the values using the `:?` placeholder.
* We can also observe that `if`-statements can have a return value.

![](examples/enums/exit-status/src/main.rs)

## Enum to represent exit code
{id: enum-to-represent-exit-code}

* We can also define `Failure` variant of the `ExitCode` type to have an number - a small number holding a value between 0-255.
* We can use a `match` statement to extract the actual number from the `Failure`.


![](examples/enums/exit-code/src/main.rs)

* Apparently the standard library of Rust uses a struct to represent an [ExitCode](https://doc.rust-lang.org/std/process/struct.ExitCode.html).

## Enumeration of the 7 days of the week
{id: enumeration-of-days-of-week}

* We can then assign one of the days to a variable and then we can use a `match` to know which day it is.

![](examples/enums/weekdays-simple/src/main.rs)

![](examples/enums/weekdays-simple/out.out)


## Enumeration with non-exhaustive patterns
{id: enumeration-non-exhaustive-match}
{i: enum}
{i: dead_code}

* In this example in the `match` we don't hanle every variant of the enum and thus we have to handle the "deafult" case using and `_` underscore.
* Try running this code after commenting out the row handline `_`.

![](examples/enums/weekdays/src/main.rs)
![](examples/enums/weekdays/out.out)

## Enumeration and manual comparision
{id: enumeration-and-manual-comparision}
{i: PartialEq}

We can also compare variables holding enum variants, but for that to work we also need to derivede from the [ParialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) trait.
Basically we need to implement the operation that allows use to compare two values of this type.

![](examples/enums/weekdays-manual-comparision/src/main.rs)
![](examples/enums/weekdays-manual-comparision/out.out)

## Enumeration and order
{id: enumeration-and-order}
{i: PartialEq}
{i: PartialOrd}

![](examples/enums/weekdays-order/src/main.rs)

![](examples/enums/weekdays-order/out.out)


## Enumeration colors - functions and String
{id: enumeration-colors-with-functions-and-string}
{i: enum}
{i: dead_code}
{i: PartialEq}

* Similar example with colors with functions to convert to RGB and from RGB values.

![](examples/enums/colors-with-functions-using-strings/src/main.rs)
![](examples/enums/colors-with-functions-using-strings/out.out)


## Enumeration colors - functions and str
{id: enumeration-colors-with-functions-and-str}

* This example is similar, but instead of storing owned string we are using str in the enum.

![](examples/enums/colors-with-functions-using-str/src/main.rs)
![](examples/enums/colors-with-functions-using-str/out.out)


## Enumeration colors - with method
{id: enumeration-colors-with-method}
{i: enum}
{i: Debug}
{i: impl}
{i: self}

![](examples/enums/colors-with-method/src/main.rs)
![](examples/enums/colors-with-method/out.out)

## Enumeration colors - with constructor
{id: enumeration-colors-with-constructor}
{i: impl}
{i: Self}

![](examples/enums/colors-with-constructor/src/main.rs)
![](examples/enums/colors-with-constructor/out.out)

## Enumerate without PartialEq using match
{id: enumerat-with-match}
{i: enum}
{i: match}
{i: dead_code}

* If we don't have PartialEq on an enum and we don't want to add it or cannot add it (e.g. because it was supplied by an external crate) we can use `match`:

![](examples/enums/colors-match/src/main.rs)
![](examples/enums/colors-match/out.out)

## Struct using enum
{id: struct-using-enum}
{i: struct}
{i: enum}

![](examples/enums/colors-struct/src/main.rs)
![](examples/enums/colors-struct/out.out)

## Exercise: enum for programming languages
{id: exercise-enum-for-programming-languages}

* Create an enum representing "all" the programming languages. (py for Python, rs for Rust)
* Add a constructor to return the enum based on the file extension.
* Remember, Perl uses both `pl` and `pm` as extensions.
* Write tests that will check some of the cases.

## Solution: enum for programming languages
{id: solution-enum-for-programming-languages}

![](examples/enums/file-extensions/src/main.rs)


## The Result enum
{id: the-result-enum}

The [Result enum](https://doc.rust-lang.org/std/result/enum.Result.html)

```
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

TODO: show the constructor example without lifetimes
TODO: Show ordering of simple enums and enums that can have different variants
TODO: show enum where one of the variants have more than one values or a value which is complex (e.g. tuple or struct)

## Iterate over the variants of an enum
{id: iterate-over-variants-of-enum}

Sometimes we need to go over all the variants of an enum and do something for each one of them.

![](list-variants/Cargo.toml)

![](list-variants/src/main.rs)

![](list-variants/out.out)


