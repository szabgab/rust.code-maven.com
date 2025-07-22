# Demo error handling with Result

Let's see how in Rust a function reports an error condition and how it can be handled by the caller.

The first line in the `read_files` function `std::fs::read_to_string` will read in the content of a file and it is expected to return it as a string.
However things can go wrong. The file might not exist or the person running the program might have not rights to read the file.
How does the function report the problem?

This function returns a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) enum that is either the content of the file wrapped in `Ok()`
or some type representing an error wrapped in `Err()`. (Although `Ok()` and `Err()` look like function calls they are called the variants of the Result enum.)

We can't directly use the returned value we must somehow check which one of the variants was returned and we need to extract the content of the variant.
We can do this with `match` or one of its many alternatives.

In this example you can see the same code twice. Once trying to read a file that does not exist and once trying to read a file that does.

{% embed include file="src/examples/introduction/demo-result/src/main.rs" %}

The output looks like this:

{% embed include file="src/examples/introduction/demo-result/out.out" %}

In the first case we printed the content of `Err()` which is the error message we got from the system-call trying to access the file.

In the second case we got the length of the string which is the length of the file.

## Always checking the returned value?

It would be rather annoying if for every function call at every level of the call-stack we would nee to check the status of the result.

However, this isn't the case. Rust allows us to automatically propagate the result up to the caller if it was an `Err()`.
This is what we can see in the next example.


---

* Result
* match
* Ok
* Err


