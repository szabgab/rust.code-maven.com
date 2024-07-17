# Learning tasks
{id: learning-tasks}

## Tasks
{id: tasks}

* Get the name of the user and say a greeting with the name.
* Get two numbers and do some calculations (rectangle).

## Hello World
{id: lt-hello-world}

* Install rust compiler, vs code, analyzer
* manual compilation
* cargo new
* cargo run

## Exercise hello world
{id: lt-exercise-hello-world}

* Intsall Rust
* Install VS Code or whatever IDE you prefer
* Install rust-analyzer
* Create a new crate
* run the Hello World

## Hello Foo
{id: lt-hello-foo}

let args = std::env::args().collect::<Vec<String>>()
let name = &args[1];
println!("Hello {}", name);

## Add two numbers in main
{id: lt-add-two-numbers-in-main}

let x = 23;
let y = 19;
let sum = x + y;
println!("{x} + {y} = {sum}");

## Set type of numbers
{id: lt-set-type-of-numbers}

let x: u8 = 23;
let y = 19;
let sum = x + y;
println!("{x} + {y} = {sum}");


## get two numbers from args and add them together
{id: lt-get-two-numbers-from-args}

let args = std::env::args().collect::<Vec<String>>()
let x = &args[1].parse::<u32>().unwrap();
let y = &args[2].parse::<u32>().except("Oh oh");


## add protection to args (if , len)
{id: lt-add-protection-to-getting-args}


## add protection to number parsing
{id: lt-add-protection-to-number-parsing}


## Exercise: rectangle, circle
{id: lt-exercise-rectangle-circle}


* Handle overflow


* Read, Write file with oneliners
* Counter




