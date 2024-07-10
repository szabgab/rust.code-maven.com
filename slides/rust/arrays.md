# Arrays in Rust
{id: arrays}

## Arrays in Rust
{id: arrays-in-rust}

* [Array](https://doc.rust-lang.org/std/primitive.array.html)
* [The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) in the book.
* An Array in Rust has a fixed number of elements.
* All the elements are of the same type.
* In mutable arrays we can change the content, but we still cannot add or remove elements.

## Rust array of numbers, length of array
{id: rust-array}
{i: len}

* Create array
* Get the number of elements, the length of the array using `len`.
* Access elements in the array using square bracket indexing.

![](examples/arrays/numbers/src/main.rs)

![](examples/arrays/numbers/out.out)

## Array of strings - access element by index
{id: array-of-strings}
{i: mut}

* To allow us to change the values in the array we have to make it mutable using `mut`.
* In most cases there is no need to decalre the type of the values and the length.  Rust can infer.

![](examples/arrays/strings/src/main.rs)

![](examples/arrays/strings/out.out)

## Array of structs
{id: array-of-structs}
{i: Debug}
{i: dead_code}

* We can also create an array from structs.

![](examples/arrays/structs/src/main.rs)

![](examples/arrays/structs/out.out)


## Array iterate on elements
{id: array-iterate}

![](examples/arrays/numbers-iterate/src/main.rs)

![](examples/arrays/numbers-iterate/out.out)

## Rust array iterate over idices
{id: rust-array-iterate-indices}

* [needless_range_loop](https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop)

![](examples/arrays/iterate-on-index/src/main.rs)

![](examples/arrays/iterate-on-index/out.out)


## Rust array iterate indices and elements with enumerate
{id: rust-array-indices}
{i: iter}
{i: enumerate}

![](examples/arrays/numbers-index/src/main.rs)
![](examples/arrays/numbers-index/out.out)

## Rust arrays are not mutable
{id: rust-array-not-mutable}

* By default arrays are not mutable and thus we cannot change a value.

![](examples/arrays/numbers-change/src/main.rs)

## Make the Rust array mutable
{id: rust-array-mutable}
{i: mut}

![](examples/arrays/numbers-mutable/src/main.rs)

## Creating an array with the same value
{id: creating-array-with-the-same-value}

![](examples/arrays/array-of-the-same/src/main.rs)

![](examples/arrays/array-of-the-same/out.out)


## Exercise: Count digits
{id: exercise-count-digits}

* Given a string of numbers, count how many times each digit appears.

```
let text = "1213 456 78843978523 3224 2421";
```

![](examples/arrays/count-digits/out.out)

## Solution: Count digits
{id: solution-count-digits}


![](examples/arrays/count-digits/src/main.rs)
![](examples/arrays/count-digits/out.out)


