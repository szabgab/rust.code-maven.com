---
title: Building a command line calculator in Rust
timestamp: 2023-11-08T14:30:01
author: szabgab
published: false
description: A short tutorial on writing a command line calculator in Rust.
tags:
    - Rust
---

In this article we'll go through the creation of a calculator in Rust that can be used on the command line.

## A folder to hold all the projects

I personally have a folder called `work` in my home directory (on Linux it is `~/work`) and every project has a folder inside.
If you don't have one yet I'd recommend you to create such a folder.


## Creating the crate

Open your terminal in the folder that holds all of your projects and create the crate by the following command:


```
cargo new calc
```

This will create a folder called `calc` with the following content:

```
$ tree calc/
calc/
├── Cargo.toml
└── src
    └── main.rs
```

The `tree` command available in Linux and probably also on macOS and any Bash shell you might have on Windows will show the names of the folders
and files, but it will skip the hidden folder and files. This does not show, but cargo also initiated a `git repository in the folder creating the
`.git` folder and also added the `.gitignore` file that will help us keep the git repository clean.

These are the files that were created:

![](examples/calc1/.gitignore)
![](examples/calc1/Cargo.toml)
![](examples/calc1/src/main.rs)

* The `src/main.rs` is a skeleton program.
* The `Cargo.toml` holds the meta-data of the program.
    * Its `name` (`calc`).
    * Its `version` ("0.1.0") that we can update as we release newer versions.
    * The Rust `edition` that we rely on. (It the "compability level" of Rust.)
    * It can also hold the list of dependencies, but we won't have any in this project.

```
cd calc
```

We can now add this to the git repository.

```
git add .gitignore Cargo.toml src/main.rs
git commit -m init
```

## Start Visual Studio code

If you use VS Code then you can start it from the command line by the following command:

```
code .
```

## Run the skeleton program

On the terminal we can now run:

```
cargo run
```

This will compile the program creating a folder called `target` where all the compilation takes place. In that folder it will create a subfolder called `debug` and in that folder
you'll fine a file called `calc`. (and several other files that, at this point, are not interesting to us). So you have the code compiled as `target/debug/calc`.

The command also created a file called `Cargo.lock` that holds the exact version of all the dependencies we use. As we don't have any external dependencies,
this is just repeating what we have in `Cargo.toml`.

The `cargo run` also ran the program printing **Hello, world!** on the screen.

At this point we could also run the program as

```
./target/debug/calc
```

if we wanted, but we don't need to do that as `cargo run` already did that for us.


## Create a README file

If is not really critical, but I'd recommend you create a `README.md` file in the root of the project that will describe the project.
I've added this:

![](examples/calc1/README.md)

We can now commit the new files.

There is a debate if you should add `Cargo.lock` to the git repository or not. The consensus is more or less that for libraries, especially open source libraries,
that other people will use you should exclude the lock file (add its name to the `.gitignore` file to avoid adding it to git) and for applications you should commit
it. As this is going to be an application we add it to git.

```
git add README.md Cargo.lock
git commit -m "add README"
```

## Add hard-coded calculation


![](images/hard-coded-calc.png)

