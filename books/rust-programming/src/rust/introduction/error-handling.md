# Error handling

How can a function indicated that it encountered a problem?

## Exception

In some languages the code raises an exception (or throws an exception) that some code higher up the call stack is expected to catch using some `try`-code.
This is said to be rather expensive and of course if at some point we forget to handle the exception then the whole program might crash.

## Return status code

In other languages the function returns a value indicating if there was an error. The programmer needs to make sure the caller can easily separate error codes from real return values.
There are various solutions for this, for example passing references and returning the data by changing the reference or if the languages allows for it returning multiple values.
(e.g. the first value is the status indicating success or the type of error and the second value being the real generated value.)

While this is doable in most languages this is convention not enforced by the language and by the compiler.

This also means that every return value to every function call must be inspected.


## Return a Result

The way this is solved in Rust is that functions that might fail can be defined to return a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) enum.
This can have either a real value wrapped in `Ok()` if the call was successful or an error message wrapped in `Err()` if there was an error.

The caller then is forced to check if the returned value indicates success or error using the `match` keyword.

This is built-in the language and thus make it easier to handle it in the same way in every code-base.

In order to avoid the need to inspect the returned value of every function call, Rust also allows us to make the call automatically short-circuit the current function
when an a call returns an error. This makes it easy to "catch" the errors only higher up in the call-stack without paying the price incurred by exception handling.



