---
title: Split a string into several individual variables
timestamp: 2023-12-13T11:30:01
published: true
description: We can split a string into the exact number of parts we would like to have.
tags:
    - itertools
    - splitn
    - next
    - into_iter
---

Sometimes we have string that contains several values separated by a delimiter. Similar to a line of a CSV file, but the separator isn't necessarily a comma.

For example a string that looks like this: **"Foo:Bar:42"**. How can we split it up and assign the parts to individual variables?

We have several solutions here using the [splitn](https://doc.rust-lang.org/std/primitive.str.html#method.splitn) method that allows us to tell rust exactly how many part we are interested in.

This `splitn` returns an iterator we can either use directly or collect its values to a vector and then use that.

Each solution has some advantages and some problems.

`splitn_and_next` uses the iterator directly calling the `next` method several times. The problem with this solution is that if there are not enough parts in the original string,
then there won't be enough iterations and this code will `panic!`.

`splitn_and_collect` collects the parts into a vector. This allows us to check if the number of parts is as expected.

`splitn_itertools` only needs to collect the parts into a vector in order to verify the correct number of parts.
Then we convert it back to an iterator and assign the values to the variables.


![](examples/split-string-into-many-variables/src/main.rs)


## Using itertools

In order to be able to collect the elements from an iterator in a single call we need the [itertools crate](https://crates.io/crates/itertools).

To add it to the crate we are working on type `cargo add itertools` or edit the `Cargo.toml` file and add it manually under the `dependencies` section:


![](examples/split-string-into-many-variables/Cargo.toml)

