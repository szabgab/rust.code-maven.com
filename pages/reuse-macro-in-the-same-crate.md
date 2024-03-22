---
title: Reuse macro in the same Rust crate across several files
timestamp: 2024-03-22T11:30:01
author: szabgab
published: true
description: Create a macro and use it in several places in the same crate.
tags:
    - macro_rules!
    -
todo:
    - move macro to separate crate in the project
    - move macro to separate crate in a different project
---

As you develop your project you probably start out with a single `main.rs` file and as the project growth you move some of the functions to other files.
How can you move macros to other files?

This is what we had in `src/main.rs` before splitting up the code.

```rust
macro_rules! prt {
    ($var: expr) => {
        println!("{:?}", $var);
    };
}

fn main() {
    let x = 23;
    prt!(x);
}
```

We have a very simple macro in this example.

## Moving the code:

We moved the `macro_rules!` definition in a file called `src/macros.rs`. The name of the file does not matter, but this seem to make sense.
The definition did not have to change.

We added

```rust
pub(crate) use prt;
```

This will make the macro public to the current crate.

Finally we put the following line at the top of the file.

```
#![allow(unused_macros, unused_imports)]
```

This is probably only important if you don't want to use all the macros. Probably this is only relevant to me while I am writing examples
to show Rust. In real-world code you probably would get rid of the macros you don't use or you'd create a separate crate that exports
macros and then you'd import only the ones you need.

The full file:

{% include file="examples/reuse-macro-in-the-same-crate/src/macros.rs" %}


In the meantime in the `src/main.rs` file we added two lines:


```rust
mod macros;
use macros::prt;
```

The first one tells Rust that we know about the module called `macros`. The second line imports the macro into our namespace.

The full file:

{% include file="examples/reuse-macro-in-the-same-crate/src/main.rs" %}


See also [this answer](https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files).
