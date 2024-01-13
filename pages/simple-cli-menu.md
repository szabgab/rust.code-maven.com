---
title: Simple CLI menu in the terminal (color selector)
timestamp: 2024-01-13T20:30:01
published: true
description: Several ways to show a menu on the command line and accept input from the user.
tags:
    - stdin
    - match
    - Option
    - Some
    - None
    - Result
    - Ok
    - Err
---

A simple assignment: Given a list of colors, (e.g.  blue green yellow white) present the user a menu on the command line:

1. blue
2. green
3. yellow
4. white

Allow the user to type in one of the numbers and show the name of the corresponding color. e.g. if the users types in 3, print out yellow.
Make it flexible so changing the list will change the menu without the need for any further changes in the code.

Then move the colors out to a text file called `colors.txt` where each row is the name of a color and display the menu that way.

## Solutions

There are a number of solutions, but they also have some common parts.

### Showing the menu

We iterate over the values and we use `enumerate` to get a tuple of the index and the the value.
However the index will start at 0 and we would like to show the menu starting from 1 so we display `ix + 1`

```rust
for (ix, value) in values.iter().enumerate() {
    println!("{}) {}", ix + 1, value);
}
```

### Reading the input from STDIN

In this version we call `expect` on the `read_line` method. This will `panic!` in case we can't read from Standard Input.

```rust
let mut response = String::new();
std::io::stdin()
    .read_line(&mut response)
    .expect("Failed to get input");
```

I am not sure how could that happen in such an interactive application, but let's see an alternative
in case you'd like to handle gracefully the case when reading from STDIN fails and you'd like to repeat the action.

```rust
match std::io::stdin().read_line(&mut response) {
    Ok(_) => {},
    Err(err) => {
        eprintln!("Error: {}", err);
        continue;
    }
}
```


## Hard-coded colors - keep asking till we get a valid answer

![](examples/infinite-cli-menu-with-hard-coded-values/src/main.rs)


## Read colors from file - keep asking till we get a valid answer

In order to make this work we change the vector we pass to the `infinite_menu` function to be a vector of `String` elements
and we also set the returned value to be a `String`.

We also had to `clone` the string that we return to disconnect it from the vector that was passed to us.


![](examples/infinite-cli-menu/src/main.rs)

The list of colors:

![](examples/infinite-cli-menu/colors.txt)


## Hard-coded colors - return an `Option`

In this example, instead of having a loop in which we keep asking the user till we get an acceptable answer, we only ask once
and if it was not a whole number or was not in the expected range then we return `None`.

In this version we already pass a vector of `String` value, but it is built up from a vector of hard-coded `&str` values.
This is "being prepared for the case when we are going to read from a file, but not actually doing it.

![](examples/cli-menu-returning-option/src/main.rs)

## Conclusion

I think this is a nice demonstration of using `match` to find out if an operation succeeded. How to handle a `Result` that can be either `Ok` or `Err`?

It is also a nice way to see how to return an `Option` that can be either a real value (a `String` in our case) wrapped in `Some` or `None` and then how to handle that with `match`.

