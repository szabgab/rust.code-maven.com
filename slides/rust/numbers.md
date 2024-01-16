# Numbers
{id: numbers}


## Numerical operations (integers)
{id: numerical-operations-integers}

* The division keeps the type so dividing one integer by another integer will always return an integer.

![](examples/numbers/calc/src/main.rs)
![](examples/numbers/calc/out.out)

## Increment integers
{id: increment-integers}

![](examples/numbers/increment/src/main.rs)
![](examples/numbers/increment/out.out)

## unfit in i8 - compile time
{id: unfit-in-i8-compile-time}

![](examples/numbers/small-integers-unfit-in-i8/src/main.rs)
![](examples/numbers/small-integers-unfit-in-i8/out.out)

## unfit in i8 - run time
{id: unfit-in-i8-run-time}

![](examples/numbers/increment-small-integers/src/main.rs)
![](examples/numbers/increment-small-integers/out.out)

## rounding float
{id: rounding-float}

![](examples/numbers/rounding-float/src/main.rs)
![](examples/numbers/rounding-float/out.out)

## Floating point imprecision
{id: floating-point-imprecision}

![](examples/numbers/floating-point-imprecision/src/main.rs)
![](examples/numbers/floating-point-imprecision/out.out)


## Exponent - power
{id: exponent-power}
{i: pow}

* We can use the [pow](https://doc.rust-lang.org/std/primitive.i32.html#method.pow) method to get the exponent of a number, but Rust needs to know the exact type of that number.
* It can be set explicitly or implicitly as in the case of the function returning an `i16` number.

* We have to be careful as `pow` can overflow.

![](examples/numbers/exponent/src/main.rs)

## Exponent protecting  agains overflow - checked_pow, saturating_pow
{id: exponent-with-overflow-protection}
{i: checked_pow}
{i: saturating_pow}

* As many other mathematical operations, calling `pow` can also create a number that does not fit in the expected type and then the code would `panic!`.
* We can use the [checked_pow](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_pow) that returns an [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
* It contains the computed value, if successful or `None` if there was an overflow.

![](examples/numbers/checked-pow/src/main.rs)

* An alternative way is to use [saturating_pow](https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_pow).

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


## Compare floating point numbers
{id: compare-floating-pount-numbers}

![](examples/numbers/compare-floats/src/main.rs)

![](examples/numbers/compare-floats/out.out)

## Compare floating point numbers by rounding
{id: compare-floating-point-numbers-by-rounding}
{i: round}

![](examples/numbers/compare-floats-by-rounding/src/main.rs)
![](examples/numbers/compare-floats-by-rounding/out.out)

## Approximately compare floating point numbers
{id: approximately-compare-floating-point-numbers}
{i: TBD}
{i: approx_eq}

* [ordered-float](https://crates.io/crates/ordered-float)
* [float-cmp](https://crates.io/crates/float-cmp)

![](examples/numbers/compare-floats-approximately/src/main.rs)
![](examples/numbers/compare-floats-approximately/out.out)

## Complex numbers
{id: complex-numbers}
{i: TBD}

* The [num-complex](https://crates.io/crates/num-complex) seems to be the most popular one.

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

![](examples/numbers/infinite-floating-point-number/src/main.rs)
![](examples/numbers/infinite-floating-point-number/out.out)


