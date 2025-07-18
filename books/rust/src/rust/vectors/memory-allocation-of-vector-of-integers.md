# Memory allocation of vector of numbers

* extend
* as_ptr

{% embed include file="src/examples/vectors/memory-integers/src/main.rs" %}
{% embed include file="src/examples/vectors/memory-integers/out.out" %}

* The Vec contains 3 values: len, capcatity and a pointer (ptr) to where the actual vector is. This is on the stack.
* The actual vector is on the heap.
* In case of integers the values of the vector are in a continuous block of memory starting from the place where the `ptr` points to.
* In this case we have `i32` values in the vector. Each one takes up 4 bytes starting from `0x5fc29ca5db80`.
* We created another variable on the heap that was placed on `0x5fc29ca5dba0`. This is 32 bytes higher.
* As we extend the vector with 4 more numbers to have 6 elements, we can see that the vector remains in the same place. `0x5fc29ca5db80`.
* When we extend it to 7 elements taking up `7*4 = 28` bytes, we can see the vector was moved to the memory starting at `0x5fc29ca5dbc0`.
* It is unclear to me why was it already moved while we still could have added one more `i32`.


