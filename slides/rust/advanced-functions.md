# Advanced Functions
{id: advanced-functions}


## Pass function as argument - hello world
{id: pass-function-as-argument-hello-world}

* [Advanced Functions and Closures](https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html)

pass function as a parameter to another function

![](examples/advanced-functions/pass-function-as-argument-simple/src/main.rs)
![](examples/advanced-functions/pass-function-as-argument-simple/out.out)


## Pass function with parameter as an argument
{id: pass-function-with-parameter-as-an-argument}

![](examples/advanced-functions/pass-function-with-parameter-as-argument/src/main.rs)
![](examples/advanced-functions/pass-function-with-parameter-as-argument/out.out)

## Dispatch table - Calculator
{id: dispatch-table}

![](examples/advanced-functions/calculator/src/main.rs)
![](examples/advanced-functions/calculator/out.out)

## Dispatch table - Calculator
{id: dispatch-table-take-2}

![](examples/advanced-functions/calculator-dispatch-table-take2/src/main.rs)
![](examples/advanced-functions/calculator-dispatch-table-take2/out.out)

See [https://users.rust-lang.org/t/mismatched-types-when-creating-dispatch-table/114527](https://users.rust-lang.org/t/mismatched-types-when-creating-dispatch-table/114527).

## Generic functions to add numbers
{id: generic-functions}

* Generics

![](examples/advanced-functions/generic-functions-add/src/main.rs)

## Generic functions to add numbers using where clause
{id: generic-functions-where-clause}
{i: where}

![](examples/advanced-functions/generic-functions-add-where/src/main.rs)

## Exercise: generic function
{id: exercise-generic-function}

* Implement a function that receives two `u8` values and returns the bigger.
* Implement a function that receives two `f32` values and returns the bigger.
* Implement a generic function that receives two values (of the same type) and returns the bigger.

## Exercise: call the add function for two points
{id: exercise-call-the-add-function-for-two-points}

* Create a struct representing a point: two attibutes x and y holding `u32` both.
* Call the add function passing two points to it.
* The result needs to be a new Point that has x1 + x2 and y1 + y2 as coordinates.

## Exercise: Implement function to repeate a string
{id: exercise-repeate-string}

* Implement a function that receives a reference to a string and an integer number and returns the string repeated N times:

```
repeat("abc_", 3)  returns abc_abc_abc_
```

Make sure we can accept any integer.


## Solution: call the add function for two points
{id: solution-call-the-add-function-for-two-points}

![](examples/advanced-functions/add-points/src/main.rs)
![](examples/advanced-functions/add-points/out.out)

