# Files
{id: files}

## Rust - write to file
{id: rust-write-to-file}
{i: File}
{i: Write}
{i: create}
{i: writeln!}

* [std::fs::File](https://doc.rust-lang.org/std/fs/struct.File.html)
* [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html)

![](examples/files/write/src/main.rs)

## Rust - read content of a file as a string
{id: rust-read-content-of-a-file-as-a-string}
{i: open}
{i: read_to_string}

![](examples/files/read-whole-file/src/main.rs)

## Rust - read file line-by-line
{id: rust-read-file-line-by-line}
{i: BufRead}
{i: BufReader}
{i: lines}

![](examples/files/read-line-by-line/src/main.rs)

## Rust - read file line-by-line with row number (enumerate)
{id: rust-read-file-line-by-line-enumerate}
{i: enumerate}
{i: lines}

![](examples/files/read-line-by-line-enumerate/src/main.rs)

## Rust - counter
{id: rust-counter}
{i: Read}
{i: Write}
{i: File}
{i: read_to_string}
{i: create}

![](examples/files/counter/src/main.rs)

## Rust list content of directory
{id: rust-list-directory}
{i: Path}
{i: read_dir}

* [read_dir](https://doc.rust-lang.org/std/path/struct.Path.html#method.read_dir)

![](examples/files/list-dir/src/main.rs)

## Rust list directory content recursively (tree)
{id: directory-tree-recursively}

* std::fs::File [struct.File](https://doc.rust-lang.org/std/fs/struct.File.html)

![](examples/files/tree/src/main.rs)

![](examples/files/list-tree/src/main.rs)

## Makedir
{id: makedir}
{i: mkdir}
{i: create_dir}

* [create_dir](https://doc.rust-lang.org/std/fs/fn.create_dir.html)

![](examples/files/makedir/src/main.rs)

## Makedirs
{id: makedirs}
{i: mkdir}
{i: create_dir_all}

* [create_dir_all](https://doc.rust-lang.org/std/fs/fn.create_dir_all.html)

![](examples/files/makedirs/src/main.rs)

## Get the temporary directory
{id: get-the-temporary-directory}
{i: temp_dir}

![](examples/files/temp-dir/src/main.rs)

## Create temporary directory
{id: temporary-directory}
{i: tempdir}

![](examples/files/tempdir-demo/Cargo.toml)
![](examples/files/tempdir-demo/src/main.rs)

## Current working directory
{id: current-working-directory}
{i: cwd}
{i: pwd}
{i: current_dir}

* [current_dir](https://doc.rust-lang.org/std/env/fn.current_dir.html)

![](examples/files/pwd/src/main.rs)

## Change directory
{id: change-directory}
{i: set_current_dir}
{i: chdir}
{i: cd}


![](examples/files/chdir/src/main.rs)

## open file error handling
{id: open-file-error-handling}

![](examples/files/open-file-handling/src/main.rs)

## Append to file
{id: append-to-file}
{i: append}
{i: open}

![](examples/files/append/src/main.rs)

## Show size of file
{id: show-size-of-file}
{i: size}
{i: len}

![](examples/files/show-size-of-file/src/main.rs)

## du - disk usage
{id: du}
{i: metadata}
{i: len}
{i: read_dir}

![](examples/files/du/src/main.rs)

## Exercise count digits in file
{id: exercise-count-digits-in-file}

Write a function that receives a string as a parameter and returns an array of 10 numbers,
the number of each digit in the file. At first you can assume the file contains a single
row containing only digits: `234566`. Then make the input file more complex

* Having multiple rows and including digits and spaces.
* Having multiple rows and including any character.

In each case we only need to count the number of each digit.



## Exercise - wc (word count)
{id: exercise-word-count}

Implement the default behaviour of the `wc` command of Linux/Unix. For each file showing
* number of lines
* number of words
* number of bytes


```
$ wc intro.md files.md strings.md
  182   519  5658 intro.md
  162   273  3133 files.md
  345   943  9708 strings.md
  689  1735 18499 total
```

## Exercise - simple grep
{id: exercise-simple-grep}

* Implement a simple version of grep that receives a search-term and a file and shows the lines in the file that match.

* Extend it to be able to work on more than one file.
* Extend it to accept regex as the search-term
* Extend it to accept an `-r` or `--recursive` flag and if given a folder then process each file recursively.

## Exercise - du (disk usage)
{id: exercise-disk-usage}

* Implement the Linux/Unix `du` command:

* Given a filename shows the size of the file.
* Given a folder shows the file sizes in the whole directory tree.
* Given the `-s` flag shows a summary instead of all the files.

## Solution: count digits in file
{id: solution-count-digits-in-file}
{i: chars}
{i: usize}
{i: enumerate}

![](examples/files/count-digits/src/main.rs)
![](examples/files/count-digits/out.out)







