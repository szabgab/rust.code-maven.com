# Hello World in Rust!

Let's start with the traditional "Hello World!" example. In this case we are going to use the Rust compiler directly just so you will see it yourself, but then we'll quickly switch to the commonly used wrapper around it called `cargo`.

* Rust files must have an extension of `.rs`. Both the main file that we run and the libraries.
* The name of the file does not matter for now, but let's call it `main` and let's put it in the `src` folder to make it easier to switch to `cargo`.
* The main Rust file must have a function called `main`.
* We declare functions using the `fn` keyword.
* Rust uses curly braces like most of the C-like languages to denote blocks of code. (Unlike Python that uses indentation or Ruby that uses `end`.)
* In order to print to the screen we use the `println!` [macro](https://doc.rust-lang.org/book/ch19-06-macros.html). It looks like function call, but we know it is a macro because of the exclamation mark `!`. It generates some Rust code during compilation.

{% embed include file="src/examples/intro/hello/src/main.rs" %}

We user the Rust compiler called `rustc` to compile the code. It will generated a file called `main` on Linux and macOS or `main.exe` on MS Windows.

```
rustc src/main.rs
```

We can take this platform and architecture dependent executable and run it on its own.

```
./main
```



* fn
* main
* println!


