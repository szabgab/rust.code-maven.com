---
title: Command line multi-counter with storage in JSON file
timestamp: 2023-12-29T14:45:01
author: szabgab
published: true
description: A command-line program that can maintain several counters in a JSON  file.
tags:
    - Path
    - BaseDirs
    - read_to_string
    - serde_json
    - from_str
    - to_string
    - writeln!
    - JSON
---


As part of the [counter example](https://code-maven.com/counter) series this command line program can maintain several counters using a JSON file as a persistent storage.

You might have already seen the [single command line counter](/cli-counter-with-plain-text-file) and we'll [serialize and deserialize a Hash](/serialize-hash-to-json).

Here is what our program should do:

```
$ cargo run foo
1
$ cargo run foo
2
$ cargo run bar
1
$ cargo run foo
3
$ cargo run
foo 3
bar 1
```


## Dependencies

For the JSON serialization and deserialization we'll use [serde_json](https://crates.io/crates/serde_json).

![](examples/multi-counter-in-json-file/Cargo.toml)


## The full code

![](examples/multi-counter-in-json-file/src/main.rs)

## Explanation


```rust
let path = std::path::Path::new("counter.json");
```

The name of the JSON file where we'll store the data. This is relative to the current working directory which might not be the best solution.
An alternative would be the using of the [directories](https://crates.io/crates/directories) crate and putting the file in the
[home directory](/home-dir) of the current user.


```rust
let bd = directories::BaseDirs::new().unwrap();
let path = bd.home_dir().join("counter.json");
```

If the file already exists we read the content using `read_to_string` and then convert the content that is supposed to be a JSON string
to a HashMap. If the file does not exist we create an empty HashMap.

We made the variable mutable as we'll want to update the appropriate counter.

```rust
let mut counters: HashMap<String, u32> = if path.exists() {
    let content = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
} else {
    HashMap::new()
};
```

Get the arguments from the command line using [Turbofish](/turbofish).

```rust
let args = std::env::args().collect::<Vec<String>>();
```

The first item in the list returned by `args` is the name of the program, so if there is exactly one item in the list that mean the user
did not provide any parameter. We list all the counters.

```rust
if args.len() == 1 {
    for (key, value) in counters.iter() {
        println!("{key} {value}");
    }
    return;
}
```

If there are two items in the list then the 2nd item (index 1) is the value provided by the user.
This is the name of the counter we save in the variable `field`.

Then we update the appropriate entry of the `counters` HashMap adding 1 to if it already exists.
Otherwise we insert 0 and increase that by 1.

Then we print the value.

Finally we serialize the HashMap to a JSON string and save it in the file.

```rust
if args.len() == 2 {
    let field = &args[1];
    *counters.entry(field.to_string()).or_insert(0) += 1;
    println!("{}", counters.get(field).unwrap());

    let json_string = serde_json::to_string(&counters).unwrap();
    let mut file = File::create(path).unwrap();
    writeln!(&mut file, "{}", json_string).unwrap();
    return;
}
```

For any other number of parameters we print a usage text to the Standard Error channel.

```rust
eprintln!("Usage: {}", &args[0]);
eprintln!("Usage: {} field", &args[0]);
```


