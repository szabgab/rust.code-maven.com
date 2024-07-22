# Threads in Rust
{id: threads}

## Threads in Rust
{id: threads-in-rust}

* [Fearless concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) (The Rust book)
* [std::thread](https://doc.rust-lang.org/std/thread/)

## Available cores
{id: available-parallelism}

![](examples/threads/available/src/main.rs)

## Simple thread (with fake work)
{id: thread-simple-with-fake-work}

* We can easily start a new thread using the [spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html) function of the [thread](https://doc.rust-lang.org/std/thread/) crate.
* We even have two loops one in the main thread and one in the created thread to "do some work". It seems to work.
* There is a slight problem though. Our main program might end before the thread can do the actual work and this example we don't even see that.

![](examples/threads/simple-while-main-is-working/src/main.rs)
![](examples/threads/simple-while-main-is-working/out.out)

## Simple thread
{id: thread-simple}
{i: thread}
{i: spawn}

* In this case when the main thread does not do "extra job" it is obvious that the other thread did not even have a chance to start working.

![](examples/threads/simple/src/main.rs)
![](examples/threads/simple/out.out)

## Simple thread with join
{id: thread-simple-with-join}
{i: thread}
{i: spawn}
{i: join}

* The solution is to save the `handle` of the thread and the use `join` to wait for its termination.

![](examples/threads/simple-with-join/src/main.rs)
![](examples/threads/simple-with-join/out.out)


## Show that threads work in parallel
{id: threads-work-in-parallel}
{i: thread}
{i: spawn}
{i: sleep}
{i: join}
{i: current}

* `spawn` will create a new thread. We can use `thread::current().id()` to get the id of the current thread.
* `join` in the main thread will block till the other thread stops.
* We can see "Loop in main thread ended" is already printed before the "Spawned thread ended", but then the main thread waits.

![](examples/threads/try-threads/src/main.rs)
![](examples/threads/try-threads/out.out)


## Handle panic! in threads
{id: handle-panic-in-threads}
{i: spawn}
{i: join}
{i: match}

![](examples/threads/thread-panic/src/main.rs)


```
cargo run
PANIC=23 cargo run
```

## Threads polling the substhreads
{id: polling-the-subthread}
{i: is_finished}
{i: iter}
{i: all}

![](examples/threads/polling/src/main.rs)
![](examples/threads/polling/out.out)


## Threads with messages
{id: threads-with-messages}
{i: channel}
{i: send}
{i: recv}
{i: move}

* We can facilitate communication between the main thread and the spawned thread.
* In this example the spawned thread is sending a message to the main thread.
* The `move` keyword tells Rust that the variables declared before spawning that are also used in the spawned code need to be moved. (`tx` in this case)
* We can use `recv`, which is blocking the main thread, to wait for a message from the spawned thread.
* In that case we will have to know how many messages to expect and if we are still waiting for a message while the spawned thread exits then we either get stuck or get panic!.
* Using the second loop is a better solution.


![](examples/threads/threads-messages/src/main.rs)
![](examples/threads/threads-messages/out.out)

## Two threads sending messages
{id: threads-sending-two-messages}
{i: clone}

* Sending messages from more than one spawned threads to the main thread.

![](examples/threads/threads-messages-multiple-sources/src/main.rs)
![](examples/threads/threads-messages-multiple-sources/out.out)


## Testing speed improvements with threads
{id: threads-speed-improvements}


* A CPU intensive task - computing the Fibonacci numbers up to N 10 times.
* Once in a single threaded process and once in a multi-threaded process with 10 threads.

Results

```
N    single     multi
     thread     thread
40:  7.1 sec vs 1.3 sec
41: 11.4 sec vs 2.1 sec
42: 18.1 sec vs 3.3 sec
```

![](examples/threads/speed-test/src/main.rs)
![](examples/threads/speed-test/out.out)


## Save many files (both CPU and IO intensive)
{id: save-many-files}

* In this example we demonstrate the speed improvement of threading.
* The program will count the number of prime numbers up to a given number. This part is CPU intensive.
* Then it will create many small files. - This part is IO intensive.

![](examples/threads/save-many-files/src/main.rs)

* Compute primes up to 1 (that is, do almost nothing).  Create 100,000 files. This is mostly IO intensive.
* We can see a 35-40% speed improvement going from no threads to 2 threads, but there is no more speed improvement.

![](examples/threads/save-many-files/100000_1.out)


* Compute primes up to 500 (CPU intensive).  Create 100,000 files. This has both CPU and IO part.
* We can see speed increase by each additional thread, but the improvement diminishes as we add more threads.

![](examples/threads/save-many-files/100000_500.out)


## Rust threads read-only access to shared variables
{id: read-only-access-to-shared-variables}

See several examples:

* Shared read-only variable with a number in it.
* Shared read-only variable with a string in it.
* Shared read-only variable with a string in it using Arc.

## Shared read-only variable with numeric value
{id: shared-read-only-variable-with-numeric-value}

* The integer is copied

![](examples/threads/shared-read-only-variable-number/src/main.rs)
![](examples/threads/shared-read-only-variable-number/out.out)

## Shared read-only variable with string value
{id: shared-read-only-variable-with-string-value}

* The string must be cloned for this to work.

![](examples/threads/shared-read-only-variable-string/src/main.rs)
![](examples/threads/shared-read-only-variable-string/out.out)


## Shared read-only variable with string value with Arc
{id: shared-read-only-variable-with-string-value-with-arc}

* We can use [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) to create Reference Counting around the data.
* The `clone` on the Arc will only increment the reference counter, but does not copy the data.

![](examples/threads/shared-read-only-variable-string-with-arc/src/main.rs)
![](examples/threads/shared-read-only-variable-string-with-arc/out.out)


## Pass reference of read-only vector to thread
{id: pass-reference-of-read-only-vector-to-thread}
{i: Arc}
{i: clone}

* [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) allows us to have reference counting.
* Here the `clone` only copies the reference and not the whole data structure.

![](examples/threads/pass-reference-to-vector/src/main.rs)
![](examples/threads/pass-reference-to-vector/out.out)

## Pass reference of read-only vector to thread improved
{id: pass-reference-of-read-only-vector-to-thread-improved}
{i: Arc}
{i: clone}

* In this solution the call to `clone` the Arc was moved inside the spawn.

![](examples/threads/pass-vector/src/main.rs)
![](examples/threads/pass-vector/out.out)

## Process read-only string slices in parallel
{id: process-string-slices-in-parallel}

* Reference counting with [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html).

![](examples/threads/process-string-slices/src/main.rs)
![](examples/threads/process-string-slices/out.out)

## Filling the memory showing that Arc works
{id: thread-without-copy}

![](examples/threads/thread-without-copy/src/main.rs)

## Pass and return ownership
{id: pass-and-return-reference}

* An alternate way to handle this situation is to return the vector.
* This way we pass the ownership back to the caller.

* This would only work properly if the threads do not need the same variable at the same time.
* So either they run sequentially in which case we don't gain CPU or each thread needs a different variable.

![](examples/threads/pass-and-return-reference/src/main.rs)
![](examples/threads/pass-and-return-reference/out.out)

## Thread scope
{id: thread-scrope}

* using [thread::scope](https://doc.rust-lang.org/stable/std/thread/fn.scope.html) there is an even simpler solution.

![](examples/threads/thread-scope/src/main.rs)
![](examples/threads/thread-scope/out.out)

## chdir in threads
{id: chdir-in-threads}

* The current working directory is a per process so separate threads cannot have different current working directories

## Environment variables in threads
{id: environment-variables-in-threads}

* The environment variables are per process so separate threads cannot have different values.

## Counter in a loop in the same process and thread
{id: counter-in-a-loop}

* This simple examples shows how we can change a variable from multiple threads
* This is the example without any threads:

![](examples/threads/counter-loop/src/main.rs)
![](examples/threads/counter-loop/out.out)

## Mutex - without threads
{id: mutex-without-threads}
{i: Mutex}

* Mutex = mutual exclusion.

A few examples to get used to the syntax of [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) without even using a thread.

![](examples/threads/mutex-without-threads/src/main.rs)
![](examples/threads/mutex-without-threads/out.out)

## Lock with Mutex
{id: lock-with-mutex}
{i: Mutex}

![](examples/threads/lock-with-mutex/src/main.rs)
![](examples/threads/lock-with-mutex/out.out)

## Counter with threads (shared variable) using Mutex
{id: counter-with-threads}
{i: Mutex}
{i: lock}

* Solution is using [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

* This solution is actually slower than the single-threaded solution because the the threads are waiting for each other to free the guards.

![](examples/threads/counter-with-mutex/src/main.rs)
![](examples/threads/counter-with-mutex/out.out)


## Counter with threads (local counting)
{id: counter-with-threads-local-counting}
{i: Mutex}
{i: lock}

* This solution does the work locally and updates the shared variable only at the end

![](examples/threads/counter-with-mutex-local/src/main.rs)

## Counter with message passing
{id: counter-with-message-passing}
{i: mpsc}
{i: channel}
{i: send}
{i: drop}
{i: TBD}

* Solution using messages

![](examples/threads/counter-with-messages/src/main.rs)

## Use threadpool with messages
{id: use-threadpool-with-messages}
{i: threadpool}

* [threadpool](https://crates.io/crates/threadpool)
* Using channel messages to let the main thread know the processing is done.

![](examples/threads/use-threadpool-with-messages/src/main.rs)


## A threaded version of the map function
{id: a-threaded-version-of-the-map-function}

* Using the [threaded-map](https://crates.io/crates/threaded-map) crate we can use the threadpool  as the `map` function.

![](examples/threads/map-with-thread/src/main.rs)

## Exercise: character counting
{id: exercise-character-counting-in-threads}

* Given a string count how many time each character appears in the string.

```
Input: "abcax"
Output:
a: 2
b: 1
c: 1
x: 1
```

* First implement a function that does this in a single thread.
* Then create a threaded solution with a shared HashMap where each thread updates the shared HashMap.
* Then create a threaded solution with local HashMaps and then updating the central HashMap at the end of the thread.


## Exercise: word count
{id: exercise-word-count-in-threads}

Implement the default behavior of the `wc` command of Linux/Unix. For each file showing
* number of lines
* number of words
* number of bytes


```
$ wc intro.md files.md strings.md
  182   519  5658 intro.md
  162   273  3133 files.md
  345   943  9708 strings.md
  689  1735 18499 total
```

## Exercise: count characters, words
{id: exercise-count-characters}

*  Given many files (e.g. clone the [Rust-maven](https://github.com/szabgab/rust.code-maven.com/) repository or the [slides](https://github.com/szabgab/slides) repository or the [ladino-dictionary-data](https://github.com/kantoniko/ladino-diksionaryo-data) repo)
* Count how many times each character appears.
* Count how many times each word appear.

## Exercise: run several functions on the same text
{id: exercise-run-several-functions-on-the-same-text}
{i: TBD}


In earlier parts of the course I have implemented several functions that were searching a text string to find certain characters.
Now run those functions on a large text file.

Allow the user to set the number of threads we would like to use.

## Exercise: Download many files in threads
{id: exercise-download-many-files-in-threads}

In the chapter about http we had an example for a blocking http client and we had an example downloading all the pages of the Rust Maven site using Tokio and async calls.

Implement the "download many files" application using threads and blocking the http client. Make the main thread collect the sizes of the downloaded pages.

## Solution: count characters, words
{id: solution-count-characters}

![](examples/threads/count-characters/src/main.rs)

## Solution - run several functions on the same text
{id: solution-run-several-functions-on-the-same-text}

![](examples/threads/separate-functions/src/main.rs)

TODO: what if there are variables in the main function? Can we read them from the threads? Can we write them?
TODO: How to share workload? e.g. We would like to create 10,000 files with the sequence number of the file being both the content and the filename.
TODO: What if we have a vector of 10,000 values and we would like to save each one of them in a separate file?


