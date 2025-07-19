# Cargo build

Executing `cargo run` will first compile your code including the build of an executable and then it will run it.
In some cases you might only want do the compilation (and executable building) phase. You can do that by typing in:

```
cargo build
```

This will check if you already have an compiled binary in the `target` folder newer than your source code.
If the binary does not exist or the source code is newer then it will build the binary. By default it will build in `debug` mode
and will place the executable in the `target/debug` folder.

## Release mode

Alternatively you could do the compilation and build the executable in `release` mode by providing the `--release` flag.

```
cargo build --release
```

This will probably take longer as it involves optimizations. You will probably do this rarely.
This will create an executable in the `target/release` folder.

In both cases the filename of the executable is the value of the `package.name` field in the `Cargo.toml` file.


* build
* target
* --release


