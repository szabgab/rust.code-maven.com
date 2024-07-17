# Vectors in Rust
{id: vectors}

## Fixed vector of numbers
{id: fixed-vector-of-numbers}
{i: vec!}
{i: len}

* A [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) vector is a series of values of the same type.
* We can initialize a vector using the [vec!](https://doc.rust-lang.org/std/macro.vec.html) macro.
* We can get the length of the vector using the [len](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len) method.
* We cannot print a vector with the simle `{}` placeholder because [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) is not implemented for it.
* However we can use the `{:?}` or the `{:#?}` placeholders.
* By default vectors are immutable.

![](examples/vectors/numbers/src/main.rs)
![](examples/vectors/numbers/out.out)

## Iterate over elements of vector using for-loop
{id: iterate-over-element-of-vector}
{i: for}
{i: in}

* We can iterate over the elements of a vector using the `for .. in ..` loop construct.

![](examples/vectors/numbers-iterate/src/main.rs)
![](examples/vectors/numbers-iterate/out.out)

## Mutable vector of numbers, append (push) values
{id: mutable-numbers-vector}
{i: mut}
{i: push}
{i: append}

* We can make a vector mutable by using the `mut` keyword.
* We can **append** an element to the end of the vector using the `push` method.

![](examples/vectors/mutable-numbers-vector/src/main.rs)
![](examples/vectors/mutable-numbers-vector/out.out)

## Mutable empty vector for numbers (push)
{id: mutable-empty-vector-for-numbers}
{i: push}
{i: mut}
{i: vec!}

* We can also create a mutable empty vector without even declaring the types of the values.
* When we `push` the first value that's how Rust will know what is the type of the elements of the vector.
* Trying to push values of other types would then generate a compilation error.

![](examples/vectors/mutable-empty-vector-for-integers/src/main.rs)
![](examples/vectors/mutable-empty-vector-for-integers/out.out)

## Mutable empty vector for strings
{id: mutable-empty-vector-for-strings}

* This is the same example as before, but this time we push a string first.

![](examples/vectors/mutable-empty-vector-for-strings/src/main.rs)
![](examples/vectors/mutable-empty-vector-for-strings/out.out)

## Mutable empty vector with type definition
{id: rust-vector}
{i: vec!}
{i: push}

* We can also declare the type of the values in the vector. It might make the code more readable.
* And it might eliminate the need to explicitely tell a `parse` method the target value type.
* In this case Rust will see the `parse` method and because the result is pushed onto a vector of `i32` numbers it will know that `i32` is the type of the **number** variable.

![](examples/vectors/vector-with-type/src/main.rs)
![](examples/vectors/vector-with-type/out.out)

## Mutable vector of strings
{id: mutable-vector-of-strings}

![](examples/vectors/mutable-vector-of-strings/src/main.rs)
![](examples/vectors/mutable-vector-of-strings/out.out)

## Remove the last element using pop, reduce capacity
{id: remove-last-element-using-pop}
{i: pop}
{i: shrink_to_fit}

![](examples/vectors/vector-pop/src/main.rs)
![](examples/vectors/vector-pop/out.out)


## Stack and the capacity of a vector
{id: stack-and-the-capacity-of-a-vector}
{i: push}
{i: pop}
{i: len}
{i: capacity}

A little example showing how a vector growth its length (`len`) and `capacity` as we `push` more and more values on it.

Then how it reduces the length but keeps the allocated `capacity` as we `pop` out elements.

Finally how it can reduce the `capacity` when calling `shrink_to_fit`.

![](examples/vectors/stack/src/main.rs)
![](examples/vectors/stack/out.out)

## Extend vectors of numbers (combining two vectors)
{id: vector-extend-numbers}
{i: extend}

![](examples/vectors/extend-numbers/src/main.rs)
![](examples/vectors/extend-numbers/out.out)


## Extend vector of Strings (combining two vectors)
{id: vector-extend-strings}
{i: extend}

![](examples/vectors/extend-strings/src/main.rs)
![](examples/vectors/extend-strings/out.out)


## Append vector of Strings (moving over elements)
{id: vector-append-strings}
{i: append}

* This will empty the second vector.

![](examples/vectors/append-strings/src/main.rs)
![](examples/vectors/append-strings/out.out)

## Split string into iterator
{id: split-string-into-iterarot}
{i: split}

* [split](https://doc.rust-lang.org/std/primitive.str.html#method.split) method returning an iterator called [Split](https://doc.rust-lang.org/std/str/struct.Split.html).

* [split_whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) method returning an iterator called [SplitWhitespace](https://doc.rust-lang.org/std/str/struct.SplitWhitespace.html).

![](examples/vectors/split-string-into-iterator/src/main.rs)
![](examples/vectors/split-string-into-iterator/out.out)


## Split string into vector
{id: split-string-into-vector}
{i: split}
{i: vec}

![](examples/vectors/split-string-into-vector/src/main.rs)
![](examples/vectors/split-string-into-vector/out.out)

## Sort vector of numbers
{id: sort-vector-of-numbers}
{i: sort}

![](examples/vectors/sort-vector/src/main.rs)
![](examples/vectors/sort-vector/out.out)

## Sort vector of strings using sorting condition
{id: sort-vector-of-strings}
{i: sort}
{i: sort_by}

![](examples/vectors/sort-strings/src/main.rs)
![](examples/vectors/sort-strings/out.out)

## Exercise: Median
{id: exercise-median}

* Write a function that given a vector of integers it will return the median.

## Exercise: ROT13
{id: exercise-rot13}

* Implement a function that given a string will return it ROT13 encrypted version.
* If we call the function again with the result we should get back the original string.

## Solution: Median
{id: solution-median}

![](examples/vectors/median/src/main.rs)
![](examples/vectors/median/out.out)

## Solution: ROT13
{id: solution-rot13}


![](examples/vectors/rot13/src/main.rs)


## Convert vector of chars to String
{id: chars-to-string}
{i: iter}
{i: into_iter}

* The `into_iter` consumes the vector so we won't be able to use it again.
* The `iter` allows for the reusing the iterator.


![](examples/vectors/chars-to-string/src/main.rs)
![](examples/vectors/chars-to-string/out.out)

## Vector of tuples
{id: vector-of-tuples}

![](examples/vectors/vector-of-tuples/src/main.rs)
![](examples/vectors/vector-of-tuples/out.out)

## Vector of structs
{id: vector-of-structs}

![](examples/vectors/vector-of-structs/src/main.rs)
![](examples/vectors/vector-of-structs/out.out)

## Vector of structs - change value
{id: vector-of-structs-change-value}


![](examples/vectors/vector-of-structs-change-value/src/main.rs)
![](examples/vectors/vector-of-structs-change-value/out.out)

## Join elements of vector into a string
{id: join-elements-of-vector}
{i: join}

![](examples/vectors/join/src/main.rs)
![](examples/vectors/join/out.out)

## Join vector of integers
{id: join-vector-of-integers}
{i: join}
{i: iter}
{i: map}
{i: collect}

![](examples/vectors/join-integers/src/main.rs)
![](examples/vectors/join-integers/out.out)

## Maximum value of a vector
{id: maximum-vale-of-a-vector}
{i: max}
{i: match}
{i: iter}

![](examples/vectors/max-number/src/main.rs)
![](examples/vectors/max-number/out.out)


## Longest or shortest string in a vector
{id: longes-or-shortest-string-in-vector}
{i: max}
{i: min}
{i: max_by}
{i: min_by}
{i: cmp}

* max and min abc order
* max and min by length

![](examples/vectors/get-longest-string/src/main.rs)
![](examples/vectors/get-longest-string/out.out)


## Change vector using map
{id: change-vector-using-map}
{i: into_iter}
{i: map}
{i: collect}

![](examples/vectors/map-on-integers/src/main.rs)
![](examples/vectors/map-on-integers/out.out)

## Update values in vector of structs using map
{id: update-values-in-vector-of-structs-using-map}

![](examples/vectors/update-vector-of-structs/src/main.rs)
![](examples/vectors/update-vector-of-structs/out.out)

## map is lazy
{id: map-is-lazy}

![](examples/vectors/map-is-lazy/src/main.rs)
![](examples/vectors/map-is-lazy/out.out)


## map is lazy that can cause problems
{id: map-is-lazy-with-problems}

![](examples/vectors/map1/src/main.rs)
![](examples/vectors/map1/out.out)

![](examples/vectors/map2/src/main.rs)
![](examples/vectors/map2/out.out)

![](examples/vectors/map3/src/main.rs)
![](examples/vectors/map3/out.out)

## filter numbers
{id: filter-numbers}
{i: iter}
{i: filter}
{i: cloned}
{i: collect}

* [cloned](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)

![](examples/vectors/filter-numbers/src/main.rs)
![](examples/vectors/filter-numbers/out.out)

## filter numbers iter into
{id: filter-numbers-iter-into}
{i: iter_into}

![](examples/vectors/filter-numbers-iter-into/src/main.rs)
![](examples/vectors/filter-numbers-iter-into/out.out)


## filter numbers by named function
{id: filter-numbers-by-named-function}

![](examples/vectors/filter-numbers-by-function/src/main.rs)
![](examples/vectors/filter-numbers-by-function/out.out)

## filter string
{id: filter-string}
{i: iter}
{i: filter}
{i: cloned}
{i: collect}


![](examples/vectors/filter-strings/src/main.rs)
![](examples/vectors/filter-strings/out.out)

## Two references to the same vector
{id: two-refernces-to-the-same-vector}
{i: TBD}

![](examples/vectors/two-references-to-the-same-vector/src/main.rs)
![](examples/vectors/two-references-to-the-same-vector/out.out)

## Filter vector of structs (cloning)
{id: filter-vector-of-structs-cloning}
{i: filter}
{i: cloned}
{i: Clone}
{i: TBD}

![](examples/vectors/filter-vector-of-structs-with-clone/src/main.rs)
![](examples/vectors/filter-vector-of-structs-with-clone/out.out)

## Convert vector of structs to vector of references
{id: convert-vector-of-structs-to-vector-of-references}
{i: TBD}

![](examples/vectors/convert-vector-of-structs-to-vector-of-references/src/main.rs)
![](examples/vectors/convert-vector-of-structs-to-vector-of-references/out.out)


## Filter vector of structs without copy
{id: filter-vector-of-structs-without-copy}
{i: TBD}

![](examples/vectors/filter-vector-of-structs/src/main.rs)
![](examples/vectors/filter-vector-of-structs/out.out)

## Accessing the last element of a vector
{id: accessing-the-last-element-of-a-vector}
{i: len}
{i: last}
{i: TBD}

* Unlike Python and Perl, rust won't let us use a negative index in a vector so we won't be able to access the last element using `vector_name[-1]`
* We can either use `vector_name.len()-1` or
* We can use `vector_name.last()`, but in this case we get an `Option` that can be `None` as well.

* If we access a seemingly arbitrary element that we calculated using `vector_name.len()-1` then either we get back a value or Rust will panic if we gave an index too big.
* On the other hand using `last` we are more protected. In that case we either get a value or `None` if the vector was empty.

![](examples/vectors/last-vec-index/src/main.rs)
![](examples/vectors/last-vec-index/out.out)

## Insert element in vector
{id: instert-element-in-vector}
{i: insert}
{i: TBD}

![](examples/vectors/insert-element-in-vector/src/main.rs)


## Vector with optional values - None or out of range?
{id: none-or-out-of-range}
{i: get}
{i: is_none}
{i: is_some}
{i: TBD}

* If we have a vector that some of the elements can be `None` then the other elements must be `Some`-values and the whole thing must be defined using `Option`.
* If we try to access an element in a vector that is out of range we get a run-time panic.
* In order to avoid such panic we either need to check if our index is in range or we can use the `get` method.
* We can use the `get` method to access the element. It will return `None` if the index was out of range.
* Then the question arise, how do we know if the value was out of range or if it was in the range but the value was `None`?

![](examples/vectors/none-or-out-of-range/src/main.rs)
![](examples/vectors/none-or-out-of-range/out.out)


## Vector with optional values
{id: vector-with-optional-values}
{i: Option}
{i: None}
{i: Some}
{i: get}
{i: TBD}

![](examples/vectors/options/src/main.rs)
![](examples/vectors/options/out.out)


## Vector length and capacity
{id: vector-length-and-capacity}
{i: len}
{i: capacity}
{i: with_capacity}
{i: resize}
{i: TBD}

![](examples/vectors/with-capacity/src/main.rs)
![](examples/vectors/with-capacity/out.out)

## References to numbers
{id: references-to-numbers}
{i: TBD}

![](examples/other/references-to-numbers/src/main.rs)
![](examples/other/references-to-numbers/out.out)

## Queue
{id: queue}
{i: VecDeque}
{i: push_back}
{i: pop_front}
{i: len}
{i: capacity}

* [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) provides for a fast queue
* It probably has to be mutable to make sense though we could create one from a fixed list of values and then just access the elements.
* We can add element at the end using `push_back`.
* We can fetch elements from the front using `pop_front`.
* As we add more elements Rust will automatically grow the `capacity` of the vector by a little bit more than needed to allow for faster growth.


![](examples/vectors/deque/src/main.rs)
![](examples/vectors/deque/out.out)

## Iterate over both index and value of a vector (enumerate)
{id: iterate-over-vector-index-value}
{i: for}
{i: iter}
{i: enumerate}

* Instead of getting the index of the current element of Rust, we can either iteratore over the indices or use `enumerate`.
* First example: iterating over the values.
* Second example: iterating over the indices and getting the value. This triggers a `needless_range_loop` suggesting the third solution:
* Third example: Creating an iterator out of the vector and calling `enumerate` on it. This will allow us to iterate over the index-value pairs.

![](examples/loops/enumerate/src/main.rs)
![](examples/loops/enumerate/out.out)

## Create vector of strings from array of str using from_iter
{id: create-vector-of-strings}
{i: from_iter}

![](examples/vectors/create-from-iter/src/main.rs)
![](examples/vectors/create-from-iter/out.out)


## Exercise: Count words using two vectors
{id: exercise-count-words-using-two-vectors}

* Given a string that consists of words and white-spaces, count how many times each word appears!
* In this solution we use two vectors. A much better solution would be to use `HashMap`, but in this example I wanted to show the solution with two vectors.
* One vector will hold the list of distinct words.
* The second vector will hold the count for each word.


## Solution: Count words using two vectors
{id: solution-count-words-using-two-vectors}


![](examples/vectors/count-words/src/main.rs)
![](examples/vectors/count-words/out.out)


