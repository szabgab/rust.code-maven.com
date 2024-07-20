# Iterators
{id: iterators}

## Iterate over vector of numbers
{id: iterate-over-vector-of-numbers}
{i: vec!}
{i: for}

![](examples/iterators/iterator-over-vector-of-numbers/src/main.rs)
![](examples/iterators/iterator-over-vector-of-numbers/out.out)


## Alternatively, we could create an iterator using the iter method.
{id: use-the-iter-method}
{i: iter}

![](examples/iterators/iterator-over-vector-of-numbers-using-iter/src/main.rs)
![](examples/iterators/iterator-over-vector-of-numbers-using-iter/out.out)


## Create a simple iterator to count up to a number
{id: create-simple-iterator-to-count}
{i: Iterator}
{i: Item}
{i: next}


![](examples/iterators/iterator-limited-counter/src/main.rs)
![](examples/iterators/iterator-limited-counter/out.out)

## Create a simple iterator to count boundless
{id: create-simple-iterator-to-count-boundless}
{i: break}

![](examples/iterators/iterator-unlimited-counter/src/main.rs)
![](examples/iterators/iterator-unlimited-counter/out.out)


## Iterate over files in current directory
{id: iterate-over-files-in-current-directory}
{i: read_dir}

![](examples/iterators/list-dir/src/main.rs)

## Iterate over files in current directory calling next
{id: iterate-over-files-in-current-directory-calling-next}
{i: read_dir}
{i: next}

![](examples/iterators/list-dir-manually/src/main.rs)

## Iterator to walk directory tree
{id: walk-directory-tree}
{i: ReadDir}

![](examples/iterators/walk-directory-tree-return-strings/src/main.rs)

## Count number of elements of an iterator
{id: count-number-of-elements-of-an-iterator}
{i: count}

![](examples/iterators/count/src/main.rs)
![](examples/iterators/count/out.out)

## Iterator: all the elements
{id: all-the-elements}
{i: all}
{i: iter}
{i: into_iter}

* `all` - calls a closure on every element of the iterator and if the closure returns `true` for every element then the expression returns `true`.
* See the documentation of the [all](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all) method.

* `iter` iterates over borrowed references and thus we need to dereference the variables with `*`, but we can continue useing the original vector.
* `into_iter` iterates over the real values and thus we cannot use the original vector any more.

* The last example shows that the iteration will stop when it encounters the first `false`.

![](examples/iterators/all/src/main.rs)
![](examples/iterators/all/out.out)

## Exercise: Iterator for the fibonacci series
{id: exercise-fibonacci-iterator}

* Crate an iterator that on every iteration will return the next value from the Fibonacci series.

## Solution: Iterator for the fibonacci series
{id: solution-fibonacci-iterator}

![](examples/iterators/fibonacci/src/main.rs)

