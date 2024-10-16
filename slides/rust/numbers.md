# Numbers
{id: numbers}

## Rust numerical types
{id: rust-numerical-types}
{i: i8}
{i: u8}
{i: i32}
{i: i64}

* By default integer numbers are stored in `i32` that has a range of -2147483648..=2147483647.
* By default floating point numbers are stored in `f64`.
* We can explicitely put numbers in different types.
* In Python a small integers takes up 28 bytes. [Size of integers in Python](https://python.code-maven.com/size-of-integer-in-python).

![](examples/numbers/number-types/src/main.rs)

## Numerical operations on integers
{id: numerical-operations-integers}
{i: +}
{i: -}
{i: *}
{i: /}
{i: %}

* The division keeps the type so dividing one integer by another integer will always return an integer.

![](examples/numbers/calc/src/main.rs)
![](examples/numbers/calc/out.out)

## Divide integers and get float
{id: divide-integers-and-get-float}
{i: as}
{i: f32}

![](examples/numbers/divide-integers/src/main.rs)

![](examples/numbers/divide-integers/out.out)

## Rust type mismatch in numerical operation
{id: rust-type-mismatch-in-numerical-operation}
{i: i32}
{i: i64}

![](examples/numbers/type-mismatch/src/main.rs)

* If we remove the `i32` then this works even though the default is `i32`.
* That's because Rust will infere the type of the first variable from the type of the second variable and the operation.


## Increment integers - augmented assignment
{id: increment-integers}
{i: +=}
{i: ++}
{i: --}

* Instead of `x = x + 1` we can use `x += 1` called [augmented assignment](https://en.wikipedia.org/wiki/Augmented_assignment).
* There are no prefix and postfix [increment and decrement operators](https://en.wikipedia.org/wiki/Increment_and_decrement_operators).

![](examples/numbers/increment/src/main.rs)
![](examples/numbers/increment/out.out)

## unfit in i8 - compile time
{id: unfit-in-i8-compile-time}

![](examples/numbers/small-integers-unfit-in-i8/src/main.rs)
![](examples/numbers/small-integers-unfit-in-i8/out.out)

## unfit in i8 - run time - overflow
{id: unfit-in-i8-run-time}

![](examples/numbers/overflow/src/main.rs)

```
cargo run
```

![](examples/numbers/overflow/out.out)

* In debug mode `panic!`


```
cargo run --release
```

![](examples/numbers/overflow/release.out)


## How to find code that might overflow / underflow?
{id: how-to-find-overflow}

* [arithmetic_side_effects](https://rust-lang.github.io/rust-clippy/master/index.html#/arithmetic_side_effects)

```
cargo clippy -- --deny clippy::arithmetic_side_effects
```

```
error: arithmetic operation that can potentially result in unexpected side-effects
```


* See also: [overflow_check_conditional](https://rust-lang.github.io/rust-clippy/master/index.html#/overflow_check_conditional)
* and [implicit_saturating_add](https://rust-lang.github.io/rust-clippy/master/index.html#/implicit_saturating_add)


## Handle overflow and underflow - saturating
{id: handle-overflow-and-underflow-saturating}
{i: saturating_add}

* [saturating_add](https://doc.rust-lang.org/std/primitive.i8.html#method.saturating_add)

![](examples/numbers/handle-overflow-saturating/src/main.rs)

![](examples/numbers/handle-overflow-saturating/out.out)

## Handle overflow and underflow - checked
{id: handle-overflow-and-underflow-checked}
{i: checked_add}

* [checked_add](https://doc.rust-lang.org/core/primitive.i64.html#method.checked_add)
* Returns and `Option` that, is either the incremented value wrapped in `Some` or `None`.

![](examples/numbers/handle-overflow-checked-add/src/main.rs)

![](examples/numbers/handle-overflow-checked-add/out.out)

## Handle overflow and underflow - overflowing
{id: handle-overflow-and-underflow-overflowing}
{i: overflowing_add}

* [overflowing_add](https://doc.rust-lang.org/core/primitive.i64.html#method.overflowing_add)
* Returns a tuple: the value (possible after overflow, and a boolean indicating if overflow happened)

![](examples/numbers/handle-overflow-overflowing-add/src/main.rs)

![](examples/numbers/handle-overflow-overflowing-add/out.out)

## Handle overflow and underflow - carrying
{id: handle-overflow-and-underflow-carrying}
{i: carrying_add}

* [carrying_add](https://doc.rust-lang.org/core/primitive.i64.html#method.carrying_add) (experimental)


## Handle overflow and underflow - strict
{id: handle-overflow-and-underflow-strict}
{i: strict_add}

* [strict_add](https://doc.rust-lang.org/core/primitive.i64.html#method.strict_add) (experimental)

## Compare integers
{id: compare-integers}
{i: cmp}
{i: Less}
{i: Greater}
{i: Equal}
{i: Ordering}

* We can use the regular `<`, `>`, `==` operators to compare any type of integers assuming the two sides are from the same type.
* The `cmp` method returns a value from the [Ordering](https://doc.rust-lang.org/std/cmp/enum.Ordering.html) enum.

![](examples/numbers/compare-integers/src/main.rs)

![](examples/numbers/compare-integers/out.out)


## Compare integers in and if-statement
{id: compare-integers-if-statement}

![](examples/numbers/compare-integers-and-action/src/main.rs)


## Exponent - power
{id: exponent-power}
{i: pow}

* We can use the [pow](https://doc.rust-lang.org/std/primitive.i32.html#method.pow) method to get the exponent of a number, but Rust needs to know the exact type of that number.
* It can be set explicitly or implicitly as in the case of the function returning an `i16` number.

* We have to be careful as `pow` can overflow.

![](examples/numbers/exponent/src/main.rs)

## Exponent protecting  against overflow - checked_pow, saturating_pow
{id: exponent-with-overflow-protection}
{i: checked_pow}
{i: saturating_pow}

* As many other mathematical operations, calling `pow` can also create a number that does not fit in the expected type and then the code would `panic!`.
* We can use the [checked_pow](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_pow) that returns an [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
* It contains the computed value, if successful or `None` if there was an overflow.
* An alternative way is to use [saturating_pow](https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_pow).

![](examples/numbers/checked-pow/src/main.rs)


## Square root (sqrt)
{id: square-root}
{i: sqrt}
{i: as}
{i: f64}

* Calling the `sqrt` method needs to know the underlying type.
* It suggests `i32`, but in fact integers don't have `sqrt` implemented, only floats.
* We can convert (cast) an integer to a floating point using the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.

![](examples/numbers/sqrt/src/main.rs)

## Square root of integer numbers
{id: square-root-of-integers}
{i: isqrt}
{i: integer_sqrt}

* There is a method called [isqrt](https://doc.rust-lang.org/std/primitive.i32.html#method.isqrt), but it is experimental.
* There is a crate called [integer-sqrt](https://crates.io/crates/integer-sqrt) that provides a trait called `IntegerSquareRoot` and a method called `integer_sqrt`.

![](examples/numbers/sqrt-of-integer/Cargo.toml)

![](examples/numbers/sqrt-of-integer/src/main.rs)


## Floating point imprecision
{id: floating-point-imprecision}

![](examples/numbers/floating-point-imprecision/src/main.rs)
![](examples/numbers/floating-point-imprecision/out.out)

## Compare floating point numbers
{id: compare-floating-pount-numbers}

* At first we compare two floating point numbers we created.
* Then we see that the floating point imprecision leads to lack of equality.

![](examples/numbers/compare-floats/src/main.rs)

![](examples/numbers/compare-floats/out.out)


## rounding float
{id: rounding-float}
{i: round}
{i: f64}

* The [round](https://doc.rust-lang.org/std/primitive.f64.html#method.round) method of floating point rounds to the nearest integer.
* So to round to a specific precision we can multiply-round-divide.


![](examples/numbers/rounding-float/src/main.rs)
![](examples/numbers/rounding-float/out.out)


## Compare floating point numbers by rounding
{id: compare-floating-point-numbers-by-rounding}
{i: round}

![](examples/numbers/compare-floats-by-rounding/src/main.rs)
![](examples/numbers/compare-floats-by-rounding/out.out)

## Approximately compare floating point numbers
{id: approximately-compare-floating-point-numbers}
{i: TODO}
{i: approx_eq}

* [float-cmp](https://crates.io/crates/float-cmp)

* Where ULP stands for "units of least precision", or "units in the last place".

![](examples/numbers/compare-floats-approximately/src/main.rs)

![](examples/numbers/compare-floats-approximately/out.out)

## NaN - Not a Number
{id: not-a-number}
{i: NaN}
{i: sqrt}

* Floating point numbers, [f32](https://doc.rust-lang.org/std/primitive.f32.html) or f64, can also represent a value called NaN or Not a Number.
* One way to get the number is to take the square root of -1.0.
* Most operations with a NaN result in NaN.
* Two NaN values are not equal.

* The `sqrt` (square root) method is not implemented for integers.

![](examples/numbers/not-a-number/src/main.rs)
![](examples/numbers/not-a-number/out.out)

## Infinite floating point numbers
{id: infinite-floating-point-numbers}
{i: inf}
{i: NaN}

* You get `inf` or `-inf` if you devide by 0.0 or -0.0 respectively.
* Adding  `inf` to `-inf` yields a `NaN`.
* Integers don't have infinite values.

![](examples/numbers/infinite-floating-point-number/src/main.rs)
![](examples/numbers/infinite-floating-point-number/out.out)


## Complex numbers
{id: complex-numbers}
{i: TODO}

* The [num-complex](https://crates.io/crates/num-complex) seems to be the most popular one.

## Functions and test adding numbers
{id: functions-and-test-adding-numbers}


![](examples/numbers/add-numbers/src/main.rs)

```
cargo run
```

```
cargo test
```


## Exercise: Rectangle ARGS with protection
{id: exercise-rectangle-args-with-protection}

* Change the previous solution to have a function that accepts 2 `u8` values and returns the area as `u8`.
* What should happen if the two numbers are both 20?
* One solution should return 255 in that case.
* You might provide other solutions as well, but for now this one is enough.


## Exercise: Rectangle add tests
{id: exercise-rectangle-add-tests}

* Take the rectangle solution.
* Move the "business logic" (the computation) to one or more functions.
* Write tests to verify the functions.

## Exercise: Circle add tests
{id: exercise-circle-add-tests}

* Take the circle solution.
* Move the "business logic" (the computation) to one or more functions.
* Write tests to verify the functions.

## Solution: Rectangle ARGS with protection
{id: solution-rectangle-args-with-protection}

![](examples/numbers/rectangle-u8/src/main.rs)

