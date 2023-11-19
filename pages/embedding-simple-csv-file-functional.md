---
title: Embedding simple CSV file and processing in a functional way
timestamp: 2023-11-19T14:30:01
description: A much nicer way to process a simple CSV file embedded in the code.
tags:
    - include_str!
    - lines
    - filter
    - is_empty
    - map

---

In an earlier article I described how to [embed a simple CSV file in the code](/embedding-simple-csv-file).

I got some suggestions to improve the code from Marcus Bosten. So let's see those.

To remind you the idea was to embed a simple CSV file in the code and then read it into a HashMap.

This is how the CSV file looks like:

![](examples/embedded-simple-csv-file-functional/data/languages.csv)

This is the new code based on the suggestions from Marcus.

A few notes

* [lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines) method allows us to iterate over the lines of a string.
* [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) method allow use to, well, filter out some of the entries. In this case we wanted to process only the lines that are not empty.
* [map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) method processes each entry and for each entry returns something. In this case returns a tuple.
* [split_once](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) is also used to makes sure if there are multiple commas in the string we only split on the first one.
* This code will panic if there is a non-empty line that has no comma in it. We could also use the `filter` to filter out and disregard those lines.

![](examples/embedded-simple-csv-file-functional/src/main.rs)


