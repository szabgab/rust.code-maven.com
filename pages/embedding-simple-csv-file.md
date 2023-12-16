---
title: Embedding simple CSV file in Rust application
timestamp: 2023-11-08T06:30:01
published: true
description:
tags:
    - include_str!
    - CSV
    - HashMap
---

We already saw how to [embed a simple string](/embedding-text-file) in our Rust application and also how to
use preprocessing to [embed a list of values](/embed-list-of-values).

Now we need to embed a simple CSV file that looks like this:

![](examples/embedded-simple-csv-file/data/languages.csv)

It is actually part of the code-base running the Rust Maven web site that maps file extensions to format types.


We need to read in this file and store as a HashMap so we'll be able to easily get the format type from a file extension.

In the case of the [list of values](/embed-list-of-values) I wrote that storing the original text file in the memory would be a waste
of memory and thus opted to preprocessing it, but then I thought. The size of these data files is really small relatively to the
size of the compiled code. For example in our sample crate the result of `cargo build --release` is a file of 4,681,888 while the data file is only 36 bytes.


## Embedding the file

In this case we take a different approach and embed the file as it is.
For this we use the [include_str!](https://doc.rust-lang.org/std/macro.include_str.html) macro.

![](examples/embedded-simple-csv-file/src/main.rs)

We can now use `cargo build --release`, we can move the resulting executable anywhere, it will already have the CSV file baked
into the code so we won't need to distribute it separately.


## Compiled size change

Though I thought the change in the compiled size would be around the size of the file we embed, but I ran a little experiment and it was way more.
I commented out the code that the "rs" extension and compiled the code. The resulting file size was 4,677,496.
Then I emptied the CSV file and compiled the code again. This time I got a file of 4,670,952.
So the difference is 6,544 bytes. Still only 0.2% of the total file size but way more than the 36 bytes I expected. I'll have to investigate this.

This is especially strange as there was no size difference in the [embedding simple string](/embedding-text-file) case.

## Improved version

After publishing this I got some suggestion, based on those I created an improved version
with more functional programming elements which is probably way better than this solution.
Check out the [Embedding simple CSV file and processing in a functional way](/embedding-simple-csv-file-functional).

