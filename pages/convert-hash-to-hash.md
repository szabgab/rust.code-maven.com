---
title: Convert hash to hash in Rust
timestamp: 2024-03-28T13:30:01
author: szabgab
published: true
description:
tags:
    - HashMap
    - new
    - from
    - from_iter
    - insert
    - map
    - collect
---

In a recent project I had a HashMap and I needed to create another HashMap using the same keys, and calculating the value based on the values from the
original hash. More precisely I had to calculate the percentage of each value from the total and convert it into a string to be displayed later.

In the following example we'll see two approaches for this and we'll see the second approach twice to make it clearer.

In the first case I've creates an empty mutable HashMap, iterated over the keys and inserted the new key-value pairs.

```rust
    let mut perc: HashMap<&str, String> = HashMap::new();
    for field in stats.keys() {
        perc.insert(field, percentage(stats[field], total));
    }
```

In the second example I took a more "functional" approach.

```rust
    let perc: HashMap<&&str, String> = HashMap::from_iter(
        stats
            .iter()
            .map(|(k, v)| (k, percentage(*v, total)))
            .collect::<Vec<(&&str, String)>>(),
    );
```

As it felt a bit too complex I also created a two-step version of this approach.
First we create a vector of tuples that will be the future key-value pairs of the
new HashMap and then we build the hash using the `from_iter` call.

```rust
    let pairs = stats
        .iter()
        .map(|(k, v)| (k, percentage(*v, total)))
        .collect::<Vec<(&&str, String)>>();
    let perc: HashMap<&&str, String> = HashMap::from_iter(pairs);
```

## The full code

{% include file="examples/convert-hash-to-hash/src/main.rs" %}


