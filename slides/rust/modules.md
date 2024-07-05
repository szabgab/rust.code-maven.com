# Modules
{id: modules}

## Function in the main.rs file
{id: modules-function-in-the-main-rs-file}

* We already know that we can defined functions in the `main.rs` file and calle them.

![](examples/modules/function-in-main/src/main.rs)

## Module defined in the main.rs file
{id: modules-module-defined-in-main-rs-file}

* We can defined a module using the `mod` keyword in the `main.rs` file and inside the module we can defined functions, structs, enums etc.
* However, in order to be able to call the function from the code outside of the module, we need to make the function public using the `pub` keyword.

* We can access the public function in two different ways.

![](examples/modules/inline-module/src/main.rs)


## Two leveles of modules
{id: modules-two-levels-of-modules}

![](examples/modules/two-levels-of-modules/src/main.rs)


## Try packages
{id: try-packages}

![](examples/modules/try-packages/src/main.rs)

