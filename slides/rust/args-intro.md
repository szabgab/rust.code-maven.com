# Command line arguments - ARGS demo
{id: args-demo}


## Command line arguments - args
{id: command-line-arguments}
{i: std::env}
{i: args}
{i: Vec}

At first the code to accept command line parameters might look scary and thus at this point we won't go into the details.
However it is nice to be able to accept parameters from the command line to add some interaction to the code.
So for now let's just accept the code.

![](examples/args/args-demo/src/main.rs)

## Exercise: Rectangle ARGS
{id: exercise-rectangle-args}

* Write a Rust application that accepts two values on the command line, the width and the heigh of a rectangle.
* Prints out the area and the circumference of the rectangle.

## Exercise: Circle ARGS
{id: exercise-circle-args}

* Write a Rust application that accetps a number - the radius of a circle.
* Prints out the area `r*r*PI` and the circumference `2*r*PI` of the circle.

## Exercise: basic math operations
{id: exercise-basic-math-operations-args}

* Write a Rust program that accepts two parameters on the command line and prints out the results of the 4 basic mathc operations `(+, -, /, *)`

```
cargo run 10 2
10 + 2 = 12
10 * 2 = 20
10 - 2 = 8
10 / 2 = 5
```


## Solution: Rectangle ARGS
{id: solution-rectangle-args}

![](examples/args/rectangle-args/src/main.rs)

```
cargo run

Usage target/debug/rectangle-args length width
```

```
cargo run  3 4


area: 12
circumference: 14
```

## Solution: Circle ARGS
{id: solution-circle-args}

![](examples/args/circle-args/src/main.rs)

## Solution: basic math operations
{id: solution-basic-math-operations-args}

![](examples/args/basic-math-operations/src/main.rs)

