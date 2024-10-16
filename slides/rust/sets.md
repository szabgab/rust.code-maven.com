# Sets
{id: sets}

## HashSet
{id: hashset}

* [HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html)
* sets are implemented as Hashes with the value being ().

## Basic Set operations in Rust
{id: basic-set-operations}
{i: set}
{i: HashSet}
{i: insert}
{i: remove}
{i: contains}

* A [HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html) can be used for the mathematical SET operations.
* We can `insert` values. The HashSet will only contain one copy of each value regardless the number of times we `insert` it.
* We can `remove` values.
* We can get the number of elements in the set using `len`.
* We can check if a set `contains` a certain value.
* There is no order among the elements so when we print them they might be in any order.

![](examples/sets/basic-set-operations/src/main.rs)
![](examples/sets/basic-set-operations/out.out)

## Union and bitor
{id: union}
{i: union}
{i: bitor}

![](examples/sets/union/src/main.rs)
![](examples/sets/union/out.out)

## Difference
{id: difference}
{i: difference}

![](examples/sets/difference/src/main.rs)
![](examples/sets/difference/out.out)

