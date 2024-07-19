# Learning tasks
{id: learning-tasks}

## Task: Setup environment
{id: lt-setup-environment}

* Create GitHub repository, clone it
* Add folder crates
* Add `.gitignore`

```
*.swp
target
```

## Task: Print Hello World
{id: lt-hello-world}

* Install rust compiler, vs code, analyzer
* manual compilation
* cargo new
* cargo run

## Exercise: Hello world
{id: lt-exercise-hello-world}

* Intsall Rust
* Install VS Code or whatever IDE you prefer
* Install rust-analyzer
* Create a new crate
* run the Hello World


## Task: Get input from command line and print it
{id: lt-get-input-from-command-line-and-print-it}

## Hello Foo
{id: lt-hello-foo}

```
let args = std::env::args().collect::<Vec<String>>()
let name = &args[1];
println!("Hello {}", name);
```

## Task: Get two numbers from the command line and do some calculations on them
{id: lt-get-two-numbers-from-the-command-line}

## Add two numbers in main
{id: lt-add-two-numbers-in-main}

```
let x = 23;
let y = 19;
let sum = x + y;
println!("{x} + {y} = {sum}");
```

## Set type of numbers
{id: lt-set-type-of-numbers}

```
let x: u8 = 23;
let y = 19;
let sum = x + y;
println!("{x} + {y} = {sum}");
```

## get two numbers from args and add them together
{id: lt-get-two-numbers-from-args}

```
let args = std::env::args().collect::<Vec<String>>()
let x = &args[1].parse::<u32>().unwrap();
let y = &args[2].parse::<u32>().except("Oh oh");
```

## add protection to args (if , len)
{id: lt-add-protection-to-getting-args}

```
if args.len() {
    eprintln!("{}", args[0]);
}
```

## add protection to number parsing
{id: lt-add-protection-to-number-parsing}

## Exercise: rectangle, circle
{id: lt-exercise-rectangle-circle}

* Handle overflow

* Read, Write file with oneliners

## Task: Implement file-based counter
{id: lt-implement-file-based-counter}

* Try to read the content of the file.
* todo! if it succeeds
* If it fails set the deault to 1

* Then replace the todo by reading the file, converting the value to a number, incrementing by 1, writing to file.

## Task: Word Count (wc)
{id: lt-word-count-wc}

* Implement a basic version of the `wc` (Word Count) command.
* On the command line get the name of a file, count how many rows, words, and bytes are in it.








