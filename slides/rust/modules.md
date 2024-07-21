# Modules
{id: modules}

## Function in the main.rs file
{id: function-in-the-main-rs-file}

* We already know that we can define functions in the `main.rs` file and call them.

![](examples/modules/function-in-main/src/main.rs)
![](examples/modules/function-in-main/out.out)

## Module defined in the main.rs file
{id: module-defined-in-main-rs-file}

* In order to divide our code into logical units we can use modules.
* First step is to define a module using the `mod` keyword in the `main.rs` file and inside the module we can define functions, structs, enums etc.
* However, in order to be able to call the function from the code outside of the module, we need to make the function public using the `pub` keyword.

* We can access the public function in two different ways.

![](examples/modules/inline-module/src/main.rs)
![](examples/modules/inline-module/out.out)

## Module in other file
{id: module-in-other-file}


![](examples/modules/module-in-file/src/main.rs)
![](examples/modules/module-in-file/src/tools.rs)
![](examples/modules/module-in-file/out.out)

