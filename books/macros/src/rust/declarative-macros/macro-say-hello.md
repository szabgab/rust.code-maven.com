# Macro with parameter to say hello

* macro_rules!
* expr

* The macro is called `say_hello`.
* parameters of the macro
* `$name: expr`    means that the macro is expecting an `expr` (expression) and it will be assigned to a variable called `$name`.


// $t:ty          means we have a paramerer called $t    and it has a type "ty" (type, such as i32 or f64)
// With this macro we can replace a short syntax with a longer syntax in at compile time.


{% embed include file="src/examples/macros/say-hello/src/main.rs" %}


