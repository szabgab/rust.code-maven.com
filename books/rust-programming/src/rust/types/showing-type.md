# Showing type

* Using the rust-analyzer will make your IDE show the type. Eg. in Visual Studio Code.
* A trick that you can also use is to specify some type at the type of assignment, e.g. `()`, the [unit](https://doc.rust-lang.org/std/primitive.unit.html).
* Then the compiler will complain during compilation.

```
let x: () = ...
let readdir: () = std::path::Path::new(".").read_dir().unwrap();
```


