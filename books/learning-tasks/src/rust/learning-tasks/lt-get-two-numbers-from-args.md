# get two numbers from args and add them together

```rust
let args = std::env::args().collect::<Vec<String>>()
let x = &args[1].parse::<u32>().unwrap();
let y = &args[2].parse::<u32>().except("Oh oh");
```


