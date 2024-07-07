# Arrays in Rust
{id: rust-arrays}

## Arrays in Rust
{id: arrays-in-rust}

* [Array](https://doc.rust-lang.org/std/primitive.array.html)
* [The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) in the book.
* An Array in rust has a fixed number of elements.
* All the elements are of the same type.
* In mutable arrays we can change the content, but we still cannot add or remove elements.

## Rust array of numbers, length of array
{id: rust-array}
{i: len}

![](examples/arrays/numbers/src/main.rs)

## Array of strings - access element by index
{id: array-of-strings}

![](examples/arrays/strings/src/main.rs)

## Array iterate on elements
{id: array-iterate}

![](examples/arrays/numbers-iterate/src/main.rs)

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

![](examples/arrays/numbers/array-of-the-same/src/main.rs)

![](examples/arrays/numbers/array-of-the-same/out.out)

## Count digits
{id: count-digits}

![](examples/arrays/count-digits/src/main.rs)
![](examples/arrays/count-digits/out.out)

## Array of structs
{id: array-of-structs}
{i: Debug}
{i: dead_code}

* We can also create a vector from structs.

![](examples/arrays/structs/src/main.rs)


