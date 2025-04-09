# Read and deserialize key-value pair JSON into HashMap

* If we have a JSON file with arbitrary key-value pairs were all the keys are the same type and all the values are the same type then we can read them into a HashMap.

* In this case all the keys are strings and all the values are integers. (positive integers that can fit in `u16`).
* [Centepide](https://en.wikipedia.org/wiki/Centipede) has between 15-191 pairs of leggs and the number of pairs is always odd. So there are no Centipedes with 100 leggs.

```json
{{#include examples/json/deserialize-to-hashmap/data.json }}
```

## Output

```
{{#include examples/json/deserialize-to-hashmap/out.out }}
```

## Code

```rust
{{#include examples/json/deserialize-to-hashmap/src/main.rs }}
```


