# Tuples in Rust
{id: tuple}

## Rust tuple - fixed-sizes, mixed, ordered collection
{id: rust-tuple}
{i: tuple}
{i: parenthesis}
{i: ()}

* Each value can be any type. (heterogeneous)
* The types and the number of elements are fixed at creation.
* In this example the types are deducted from the values.
* It is ordered.
* We can access the individual elements using their index and the dot-notation.

* The content can be changed only if the definition uses `mut`.
* [Tuple](https://doc.rust-lang.org/std/primitive.tuple.html)
* [Tuple Types](https://doc.rust-lang.org/reference/types/tuple.html)
* [Examples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)

![](examples/tuples/create-tuple/src/main.rs)
![](examples/tuples/create-tuple/out.out)

## Define the types in the tuple
{id: define-the-types-in-the-tuple}

* Optionally we can define the types of elements of a tuple.
* This can be useful if we don't want the default types. (e.g the default integer type is `i32`, but here we use `i8`.)

![](examples/tuples/define-types/src/main.rs)
![](examples/tuples/define-types/out.out)

## Change tuple (mutable)
{id: change-tuple}
{i: mut}

* Use the `mut` keyword to make the values of the tuple mutable.
* We still cannot add elements to the tuple. It's shape and the types of the fields are fixed.
* Assign a new value to one of the elements using the dot-notation.

![](examples/tuples/change-tuple/src/main.rs)
![](examples/tuples/change-tuple/out.out)

## Create tuple with types, but without values
{id: create-tuple-with-types-but-without-values}
{i: mut}

* We can create a tuple without initializing. In this case it seems even more useful to declare the types. (Though not required.)
* We must have the `mut` keyword to make it mutable.
* Then later we can assign all the values at once.
* Before we initialize all the values we cannot assign them one-by-one.

![](examples/tuples/create-tuple-without-values/src/main.rs)
![](examples/tuples/create-tuple-without-values/out.out)


## Destructuring tuple
{id: destructuring-tuple}
{i: _}
{i: underscore}

* It is not **destructing**! Just looks similar.
* It means taking the values of a tuple apart into individual variables.
* We have to assign all the values.
* We can use the underscore `_` (multiple times) as a placeholder for the value we don't want to assign.

* Alternatively, we could assign them one-by-one.

![](examples/tuples/destructuring-tuple/src/main.rs)
![](examples/tuples/destructuring-tuple/out.out)

## The empty tuple is called the unit
{id: empty-tuple-unit}
{i: unit}
{i: ()}

* An empty pair of parentheses `()` creates an empty tuple, also called a [unit](https://doc.rust-lang.org/std/primitive.unit.html).
* Functions that don't return anything return the same `unit` value.

![](examples/tuples/empty/src/main.rs)
![](examples/tuples/empty/out.out)

## One element tuple
{id: one-element-tuple}

* We can create a one-element tuple by putting a comma after the element, but probably there is not much value in it.
* It is better to just create a variable holding that single value.

![](examples/tuples/one-element/src/main.rs)
![](examples/tuples/one-element/out.out)

## Enumerate over vector uses tuples
{id: enumerate-over-vector}

![](examples/tuples/enumerate-vector/src/main.rs)

![](examples/tuples/enumerate-vector/out.out)


## Return multiple values from a function
{id: return-multiple-values-from-a-functions}

![](examples/tuples/return-multiple-values/src/main.rs)

![](examples/tuples/return-multiple-values/out.out)

