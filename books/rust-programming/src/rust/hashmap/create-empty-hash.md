# Create empty HashMap, insert key-value pairs

* HashMap
* new
* insert
* len
* mut

* [std::collections](https://doc.rust-lang.org/std/collections/index.html)
* [HashMap](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html)

* When we create a HashMap we don't necessarily have to define the types of the keys and the values as those can be deducted from the later assignments.
* If we'd like to add new key-value pairs to the hash, we need to declare it as **mutable**.
* The `insert` method allows us to add a new key-value pair.
* `len` will tell us the number of keys.

{% embed include file="src/examples/hashes/create-empty-hash/src/main.rs" %}
{% embed include file="src/examples/hashes/create-empty-hash/out.out" %}


