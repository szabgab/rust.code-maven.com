# HashMap in Rust
{id: hashmap}

## What is a HashMap?
{id: what-is-a-hashmap}

* A data structure holding key-value pairs with an O(1) access time.

* Hash (Perl)
* Map (Java)
* Dictionary (Python)
* Associative Array (PHP)

## Create empty HashMap, insert key-value pairs
{id: create-empty-hash}
{i: HashMap}
{i: new}
{i: insert}
{i: len}
{i: mut}

* [std::collections](https://doc.rust-lang.org/std/collections/index.html)
* [HashMap](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html)

* When we create a HashMap we don't necessarily have to define the types of the keys and the values as those can be deducted from the later assignments.
* If we'd like to add new key-value pairs to the hash, we need to declare it as **mutable**.
* The `insert` method allows us to add a new key-value pair.
* `len` will tell us the number of keys.

![](examples/hashes/create-empty-hash/src/main.rs)
![](examples/hashes/create-empty-hash/out.out)

## Create immutable hash with data
{id: create-hash-with-data}
{i: HashMap}
{i: from}
{i: keys}

* We can also create a HashMap from existing data. In this case the hash does not have to be mutable.

![](examples/hashes/create-hash-with-data/src/main.rs)
![](examples/hashes/create-hash-with-data/out.out)


## Check if hash contains key (key exists)
{id: hash-containes-key}
{i: from}
{i: contains_key}
{i: contains}
{i: exists}

![](examples/hashes/contains-key/src/main.rs)
![](examples/hashes/contains-key/out.out)

## Get value from hash
{id: get-value-from-hash}
{i: get}
{i: from}

* `get` returns an `Option` containing the value corresponding to the key, or `None`, if the key does not exist.

![](examples/hashes/get-value-from-hash/src/main.rs)
![](examples/hashes/get-value-from-hash/out.out)

## Iterate over keys of a hash
{id: iterate-over-keys-of-hash}
{i: keys}
{i: from}

![](examples/hashes/iterate-over-keys/src/main.rs)
![](examples/hashes/iterate-over-keys/out.out)

## Iterate over key-value pairs in a Hash
{id: iterate-over-key-value-pairs-of-hash}
{i: keys}
{i: iter}

* Use the `iter` method to get the iterator
* Though you can iterate over the hash directly. It does the same.

![](examples/hashes/iterate-over-pairs/src/main.rs)
![](examples/hashes/iterate-over-pairs/out.out)


## Rust hash update value
{id: rust-hashmap-update-value}
{i: entry}
{i: new}
{i: or_insert}

* Using the `or_insert` method we can update a value and insert a default value in case the key did not exist.

![](examples/hashes/update-hash-value/src/main.rs)
![](examples/hashes/update-hash-value/out.out)

![](examples/hashes/updating-values/src/main.rs)

## Rust update values in a hash - count words
{id: rust-update-values-in-a-hash}
{i: entry}
{i: or_insert}

![](examples/hashes/count-words/src/main.rs)

## Remove element from hash
{id: remove-element-from-hash}
{i: remove}
{i: keys}

![](examples/hashes/remove-from-hash/src/main.rs)
![](examples/hashes/remove-from-hash/out.out)

## Accessing values
{id: accessing-values}
{i: unsert}
{i: unwrap_or}
{i: get}
{i: copied}

* unwrap_or(0)  sets a default value to 0

![](examples/hashes/accessing-values/src/main.rs)
![](examples/hashes/accessing-values/out.out)


## Split string create hash
{id: split-string-create-hash}
{i: split_whitespace}
{i: collect}
{i: insert}

![](examples/hashes/split-string-create-hash/src/main.rs)
![](examples/hashes/split-string-create-hash/out.out)

## Create hash from key-value pairs after split
{id: create-hash-from-key-value-pairs}
{i: to_string}
{i: collect}
{i: insert}

![](examples/hashes/hash-from-key-value-pairs/src/main.rs)
![](examples/hashes/hash-from-key-value-pairs/out.out)

## Read key-value pairs from file
{id: read-key-value-pairs-from-file}

![](examples/hashes/read-key-value-pairs/key_value_pairs.txt)

![](examples/hashes/read-key-value-pairs/src/main.rs)
![](examples/hashes/read-key-value-pairs/out.out)

## Sort vector of hashes
{id: sort-vector-of-hashes}
{i: sort_by}
{i: push}

![](examples/hashes/sort-vector-of-hashes/src/main.rs)
![](examples/hashes/sort-vector-of-hashes/out.out)


## Mapping structs based on more than one key
{id: mapping-structs-based-on-more-than-one-key}
{i: Debug}
{i: dead_code}
{i: struct}
{i: HashMap}

![](examples/hashes/multiple-mapping/src/main.rs)

## Mapping structs based on more than one key from a vector
{id: mapping-structs-based-on-more-than-one-key-from-a-vector}


![](examples/hashes/multiple-mapping-from-vector/src/main.rs)


## Create hash from vector of tuple pairs
{id: create-hash-from-vector-of-pairs}
{i: vec!}
{i: Vec}
{i: HashMap}
{i: from_iter}


![](examples/hashes/hash-from-vector/src/main.rs)
![](examples/hashes/hash-from-vector/out.out)


## Hash of vectors in Rust
{id: hash-of-vectors}
{i: and_modify}
{i: split}
{i: contains_key}
{i: insert}


![](examples/hashes/hash-of-vectors/src/main.rs)

![](examples/hashes/hash-of-vectors/out.out)

## Integers as keys of a HashMap
{id: integers-as-keys-of-a-hashmap}

![](examples/hashes/integers-as-keys/src/main.rs)

![](examples/hashes/integers-as-keys/out.out)

## Tuples as keys of a HashMap
{id: tuples-as-keys-of-a-hashmap}

![](examples/hashes/tuples-as-keys/src/main.rs)

![](examples/hashes/tuples-as-keys/out.out)

## Structs as keys of a HashMap
{id: structs-as-keys-of-a-hashmap}
{i: struct}
{i: derive}
{i: Debug}
{i: Hash}
{i: Eq}
{i: PartialEq}

* We can also use structs as a keys in a HashMap, but for that we need to add a few traits.

![](examples/hashes/structs-as-keys/src/main.rs)

![](examples/hashes/structs-as-keys/out.out)

