# CLI - Command Line Interface

People who use a terminal are used to run tons of different command line tools and utilities. They are used to certain ways to provide input on the command line.

In this book we'll see how you can write Rust programs that will accept parameters on the command line.

Let's start with a few examples from the point of view of the user to see what kind of patterns are we looking to accept:


## No parameter

If there is no parameter the program might use some defaults or might complain about the lack of parameters.

```
$ program
```

## A list of filenames of the command line

In this case all the parameters have the same type, the program is expected to go over the files one-by-one. There is no special treatment of the files except their order.

```
$ program file1.csv file2.csv file3.csv
```

## Positional meaning

We might have a program where the position of the parameters indicate their task. In a very simple case that might be: first is the input file, the second is the output file.
In more complex cases we might have 4-5 or even more positional arguments. This means the user will have to remember the order of the parameter both when executing the command
and later when looking at previously executed commands.

It also makes it impossible to leave out the earlier parameters and rely on a default value because that would change the position of all the other parameters.

```
$ program input.jpeg output.png
```

## Named parameters

A much more flexible way to accept and provide parameters is to expect and use named parameter. The most common way to accept named parameters
is to expect the [POSIX Sytanard](https://en.wikipedia.org/wiki/POSIX) which, if I am not mistaken, boils down to the following:

* There can be single-letter names prefixed by a single dash, such as `-v`, `-m`.
* Some of these might expect a value, e.g. `-m google.com`.
* Others might be "flags", their mere presence have an impact. e.g. `-v` might mean verbose mode.
* Lower- and upper-case letters have different meaning.
* The single-letter parameters can be combined, but only the last one might have a value. e.g. `-vm google.com`.

* There can be long names prefixed by two dashes. e.g. `--verbose` or `--machine`.

## Single-dash long-name

There are some systems that use long names with a single dash, e.g. `-verbose`. Mostly I saw that in Java.
I don't recommend that style and most likely we won't deal with that in this booklet.

## Subcommands

Some system use "subcommands". Probably the most popular among programmers is "git". o
If you look at the [documentation of git](https://git-scm.com/docs/git) you will see it accepts various single-letter and long parameters, then a command, and then additional arguments.
`add`, `clone`, `commit`, `push`, `pull` there are all subcommands and each one of them has its own arguments. For example this is what [commit](https://git-scm.com/docs/git-commit) has.
So you can have a command like

```
git --verbose commit -m "some text"
```


We are going to cover all of these cases.








