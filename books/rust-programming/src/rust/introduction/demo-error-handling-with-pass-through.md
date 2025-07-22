# Demo error handling with Result and question mark

In this example we set up the `read_files` function to return a `Result` enum that can contain either nothing (`()`) or a "boxed error".
Don't worry about the details of this for now. Suffice to say that the `read_files` function itself now is expected to return a Result.

That means in the `main` function now we should check, using `match`, if it was a success (returning an `OK()` that contains nothing represented by an empty pair of parentheses,
in which case we do nothing, or if it was an `Err()` in which case we print the error message.

Inside the `read_files` function we could remove all the `match` expressions and added a question-mark `?` at the end of the `read_to_string` function calls.
This operator will make Rust check if the returned value is `Ok()`. If it is then the expression will return the actual content without the `Ok()` wrapper.
So in this case the variable `content` will already contain the content of the file.
If the returned value is `Err()` then Rust will arrange for the `read_files` function to immediately return this same error wrapped in `Err()`.

Basically if there was an error it passes up on the call-stack, if it was successful then it peals off the `OK()` wrapper and gives us the value we were looking for.

{% embed include file="src/examples/introduction/demo-error-handling/src/main.rs" %}

Running the code will result in this output:

{% embed include file="src/examples/introduction/demo-error-handling/out.out" %}

This is different from the previous one as we also changed the semantics of the code.
So I might need a better example to demonstrate this, but I think you get the idea of how Rust handles errors and how we can eliminate a lot of the boilerplate code.

---

* Result
* dyn
* Box


