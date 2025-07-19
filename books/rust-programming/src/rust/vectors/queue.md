# Queue

* VecDeque
* push_back
* pop_front
* len
* capacity

* [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) provides for a fast queue
* It probably has to be mutable to make sense though we could create one from a fixed list of values and then just access the elements.
* We can add element at the end using `push_back`.
* We can fetch elements from the front using `pop_front`.
* As we add more elements Rust will automatically grow the `capacity` of the vector by a little bit more than needed to allow for faster growth.


{% embed include file="src/examples/vectors/deque/src/main.rs" %}
{% embed include file="src/examples/vectors/deque/out.out" %}


