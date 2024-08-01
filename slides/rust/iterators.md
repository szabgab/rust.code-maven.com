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


## Iteration moves values
{id: iteration-moves-values}

* If we remove the `&` from the first iteration the code won't compile any more as we have moved the values.

![](examples/iterators/iteration-moves-values/src/main.rs)
![](examples/iterators/iteration-moves-values/out.out)


## Iterator that restarts
{id: iterator-that-restarts}
{i: next}

* After consuming all the elements of the iterator it return `None`, but then it restarts and we can ask for the `next` element.

![](examples/iterators/restarting/src/main.rs)
![](examples/iterators/restarting/out.out)

## Circular Iterator that restarts
{id: circular-iterator-that-restarts}
{i: next}

![](examples/iterators/circular/src/main.rs)
![](examples/iterators/circular/out.out)




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

## Mutable iterator
{id: mutable-iterator}
{i: iter_mut}

![](examples/iterators/change/src/main.rs)
![](examples/iterators/change/out.out)

## Take the first N elements of an iterator
{id: take-the-first-n-iterations}
{i: take}

* `take` creates a new iterator from the first `n` element of any iterator.
* It can be used on an infinite iterator as well.
* The example we have here does not add value over just using a range, but for every other iterator it will make sense.

![](examples/iterators/iterator-take/src/main.rs)

![](examples/iterators/iterator-take/out.out)

## Skip the first N elements of an iterator
{id: skip-the-first-n-iterations}

![](examples/iterators/iterator-skip/src/main.rs)

![](examples/iterators/iterator-skip/out.out)


## Exercise: Iterator for the Fibonacci series
{id: exercise-fibonacci-iterator}

* Crate an iterator that on every iteration will return the next value from the Fibonacci series.

## Solution: Iterator for the Fibonacci series
{id: solution-fibonacci-iterator}

![](examples/iterators/fibonacci/src/main.rs)

## Iter strings
{id: iter-strings}
{i: TBD}

![](examples/iterators/iter-strings/src/main.rs)

