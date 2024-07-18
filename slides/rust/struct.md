# struct
{id: struct}

## Create simple struct
{id: create-simple-struct}
{i: struct}

* A simple `struct` will have one or more fields. Each field has a name and a type.
* We can then create an instance of the struct with actual values.
* We can use the dot-notation to access the values of the fields.
* We cannot change the values unless we declare the struct as mutable using the `mut` keyword.

![](examples/struct/point/src/main.rs)
![](examples/struct/point/out.out)

## Change attributes of a mutable struct
{id: mutable-struct}
{i: mut}

* Using the `mut` keyword we can defined a `struct` to be mutable.

![](examples/struct/mutable-point/src/main.rs)
![](examples/struct/mutable-point/out.out)

## Implement a method for a struct
{id: struct-method}
{i: impl}
{i: self}
{i: &self}

* We can add methods to a `struct` using the `impl` keyword (we implement the method).
* The first parameter of every method must be the variable name `self` that will represent the current instance.
* We can have either `self` or `&self`. The latter will borrow the instance and it will allow us to call two methods (or the same method twice as in our example).
* If we used only `self` the instance would be moved on the first method call. Rarely used.

![](examples/struct/point-with-method/src/main.rs)
![](examples/struct/point-with-method/out.out)


## Struct method to modify fields
{id: struct-method-to-modify-fields}
{i: impl}
{i: mut}
{i: &mut}

* We can add methods to a `struct` using the `impl` keyword (we implement the method) that will modify the struct.
* For this we need to write `&mut self` in the method and the struct instance must be also mutable.

![](examples/struct/point-with-mutable-method/src/main.rs)
![](examples/struct/point-with-mutable-method/out.out)

## Struct inheritance
{id: struct-inheritance}

* There is no inheritance among structs similar to classes in other languages. However, there is composition.
* In other words, attributes of a struct can be both primitive types and other structs.

## Struct composition: Circle
{id: struct-composition}
{i: struct}
{i: impl}

* A `Cricle` is built from a `Point` and a radius.
* We can implement (using `impl`) methods on both the `Point` and the `Circle` struct.
* The `Circle` struct does **not** have a `mv` method.

![](examples/struct/circle-compose/src/main.rs)
![](examples/struct/circle-compose/out.out)

## Struct composition: Line
{id: struct-composition-line}

* Not much difference from the `Circle`. A `Line` is composed of two `Point` structs.

![](examples/struct/line-compose/src/main.rs)
![](examples/struct/line-compose/out.out)

## Struct with vector of structs - Polygon
{id: struct-with-vector}

* We can create a `struct` that has a single attribute which is a vector of `Point` instances.
* We can then use `impl` to implement a function.

![](examples/struct/polygon-struct/src/main.rs)
![](examples/struct/polygon-struct/out.out)

## Printing struct fails
{id: printing-struct-fails}

* We can print the values of the individual attributes of a struct, but we cannot print the whole struct.

![](examples/struct/printing-struct-fails/src/main.rs)
![](examples/struct/printing-struct-fails/out.out)

## Print struct - implement Display
{id: print-struct}
{i: impl}
{i: Display}
{i: Formatter}

![](examples/struct/printing-struct/src/main.rs)
![](examples/struct/printing-struct/out.out)

## Debug struct - implement Debug
{id: debug-struct}
{i: impl}
{i: Debug}

![](examples/struct/debugging-struct/src/main.rs)
![](examples/struct/debugging-struct/out.out)

## Derive Debug for struct
{id: derive-debug-for-struct}
{i: derive}
{i: Debug}

* We don't need to implement the `fmt` method of the `Debug` trait ourselves. We can `derive` it:

![](examples/struct/debug-struct/src/main.rs)

![](examples/struct/debug-struct/out.out)


## Struct with vector and optional value
{id: struct-with-vector-and-optional-value}

![](examples/struct/struct-with-vector-and-optional-value/src/main.rs)
![](examples/struct/struct-with-vector-and-optional-value/out.out)

## Printing and debug-printing simple struct
{id: printing-simple-struct}

![](examples/struct/struct-cannot-be-printed/src/main.rs)

![](examples/struct/struct-with-debug/src/main.rs)
![](examples/struct/struct-with-debug/out.out)


![](examples/struct/struct-with-display/src/main.rs)
![](examples/struct/struct-with-display/out.out)

## Use a tuple as a struct to represent color
{id: use-a-typle-as-struct}

![](examples/struct/tuple-as-struct/src/main.rs)
![](examples/struct/tuple-as-struct/out.out)

## Add method to tuple-based struct
{id: method-to-tuple-based-struct}

![](examples/struct/tuple-as-struct-method/src/main.rs)
![](examples/struct/tuple-as-struct-method/out.out)

## Struct with method
{id: struct-with-method}

![](examples/struct/struct-with-method/src/main.rs)
![](examples/struct/struct-with-method/out.out)

## Structs and circural references
{id: structs-and-circular-references}

* Rust will make sure we cannot create circular reference in this way.
* `#[allow(unused_mut)]` is needed to silence clippy, the linter

![](examples/struct/circural-references/src/main.rs)
![](examples/struct/circural-references/out.out)

* Try to enable the commented out code and see the error message.

![](examples/struct/circural-references/err.out)

## new method with default values for struct
{id: new-method-with-default-values}
{i: new}

* We use the `new` name by convention. It has no special meaning in Rust.

![](examples/struct/new-method/src/main.rs)

## The new method has no special feature
{id: new-method-no-special-feature}
{i: new}

* We could use any other name instead of `new`. For example we could use `qqrq` as well.
* The name `new` is only to give the power of familiarity.

![](examples/struct/constructor/src/main.rs)


## Default values
{id: default-values}

![](examples/struct/default/src/main.rs)

## Default for composite struct
{id: default-for-composite-struct}

![](examples/struct/default-for-composite-struct/src/main.rs)

![](examples/struct/default-for-composite-struct/out.out)


## Compare structs for Equality
{id: compare-structs-for-equality}
{i: Eq}
{i: PartialEq}

* Each data type in Rust either implements Eq or PartialEq to allow users to check if two objects of the same type are equal using either the `==` operator or the `eq` method.
* When creating a struct it does not automatically implement these traits, but we can add them.
* Primitive data types such as `integers` and `strings` implement both Eq and PartialEq.
* `float` on the other hand only implements PartialEq as a float can also be NaN that would break Eq.

* We can add the `Eq` trait to any struct and if all the elements of the struct implement `Eq` then we can add that too:
* It will automatically provide us with the possibility to use `==` or `eq` (or `!=` or `ne` for that matter) on the values of that type.
* However `Eq` is mostly just an indication to the compiler, the actual implementation is in `PartialEq` so we need to add that too.

* In order for two objects of this type to be equal, all the fields have to be equal.

![](examples/struct/compare-structs-for-equality/src/main.rs)

![](examples/struct/compare-structs-for-equality/out.out)


## Compare structs for Equality - manual implementation
{id: compare-structs-for-equality-manual-implementation}
{i: PartialEq}
{i: eq}


* The `#[allow(dead_code)]` part is only needed as in this example we never use the `name` field. In real code you will probably not need it.

![](examples/struct/compare-structs-for-equality-manually/src/main.rs)
![](examples/struct/compare-structs-for-equality-manually/out.out)


## Compare structs for partial equality - PartialEq
{id: compare-structs-for-partial-equality}
{i: struct}
{i: PartialEq}

* [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
* the trait `Eq` is not implemented for `f32`

![](examples/struct/compare-structs-for-partial-equality/src/main.rs)
![](examples/struct/compare-structs-for-partial-equality/out.out)


## Compare structs for ordering (sorting) - Ord
{id: compare-structs-for-ordering}
{i: Ord}
{i: PartialOrd}

* In order to be able to decide which object is "bigger" or "smaller" than the other one we need the `Ord` trait that requires `PartialOrd` trait and the `Eq` and `PartialEq` traits.
* This will allow use to `sort` the values.
* Comaring the fields happen in the order the appear in the definition of ths struct. In our case Rust forst compares the 'number' fields. The 'name' fields are only compared if the 'number' fields are equal.

![](examples/struct/compare-structs-for-ordering/src/main.rs)
![](examples/struct/compare-structs-for-ordering/out.out)

## Compare structs for partial ordering (sorting) - PartialOrd
{id: compare-structs-for-partial-ordering}
{i: PartialOrd}

* [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
* Neither the `Eq` nor the `Ord` traits are implemented for floats.

![](examples/struct/compare-structs-for-partial-ordering/src/main.rs)
![](examples/struct/compare-structs-for-partial-ordering/out.out)


## Manually implement ordering
{id: manually-implement-ordering}
{i: PartialOrd}
{i: PartialEq}
{i: partial_cmp}
{i: eq}

* In rare case we might want to have an ordering that is not the same as the default implementation of `PartialOrd`. In these cases we can implement it ourselves.
* For this we need to implement both `PartialEq` and `PartialOrd`.
* In our case the functions only take into account the `height` field.
* the `#[allow(dead_code)]` is only needed in this example because we never access the `id` and `name` fields.

![](examples/struct/implement-ordering/src/main.rs)

![](examples/struct/implement-ordering/out.out)


