---
title: Initialize immutable HashMap with data
timestamp: 2024-04-25T11:20:01
author: szabgab
published: true
description:
tags:
    - HashMap
    - from
    - tuples
    - keys
    - values
---

Earlier we saw how to [define an empty HashMap and insert values](/create-empty-hashmap-and-insert-pairs).
We also saw that there is [no need to define the types of a HashMap](/create-empty-hashmap-without-type-definition), Rust can figure that out during compilation.


Sometimes, actually probably quite rarely we would like to create a HashMap and already fill it with values. We can do that using the
[from](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.from) method providing it an array of tuples.
In each tuple the first element will become the key and the second element will become the value in the HashMap.

We don't need to explicitly tell Rust what is the type of the keys and the values of the HashMap, Rust can figure it out from the
types in the tuples. In this case both the keys and the values are of type `&str`.


## Use the keys method to get the keys

[keys](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.keys)

```rust
println!("{:#?}", animal.keys());
```

## Use the values method to get the values

[values](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.values)


```rust
println!("{:#?}", animal.values());
```


## We cannot insert in immutable HashMap

If we try:

```rust
animal.insert("cat", "cute");
```

We get a compilation error:

```
cannot borrow `animal` as mutable, as it is not declared as mutable
```

It is not that surprising, after all the variable is immutable by default.


## Full code

{% include file="examples/hashmap/initialize-hashmap-with-data/src/main.rs" %}


