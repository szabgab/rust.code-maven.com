# rayon
{id: rayon}

## What is rayon
{id: what-is-rayon}

* [Rayon](https://crates.io/crates/rayon) is a data-parallelism library for Rust.

* "work stealing"
* [Cilk](https://en.wikipedia.org/wiki/Cilk)

## replace map with par_iter
{id: replace-map-with-par-iter}
{i: rayon}
{i: par_iter}
{i: map}

![](examples/rayon/par-iter/src/main.rs)
![](examples/rayon/par-iter/out.out)


## Tasks with different processing time
{id: tasks-with-different-processing-time}
{i: TBD}

Experimental example to show how Rayon distributes the load when the tasks have random processing time
(betwee 1-1000 ms) and/or when there are a few long-running tasks and many short tasks.

![](examples/rayon/tasks-with-random-time/src/main.rs)


## map with threads
{id: map-with-threads}
{i: TBD}

TODO: This is experimental code that needs to be improved

![](examples/rayon/map-thread/src/main.rs)


