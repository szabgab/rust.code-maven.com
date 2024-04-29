---
title: Keep you data safe using advisory lock on your files
timestamp: 2024-04-29T17:30:01
author: szabgab
published: true
description: Lock your files so two or more processes accessing them won't interfere.
tags:
    - lock
    - flock
    - advisory lock
    - Seek
    - SeekFrom
    - truncate
    - set_len
    - FileLockMode::Exclusive
    - open
    - read
    - write
    - create
    - options
---

There are many cases when I need some kind of a "database", but in reality a simple text file or a simple JSON file can do the job.
No problem, I can tell myself I know how to [read from a file](/slurp) and how to [write to a file](/write-to-a-file).

There is, however, a small possibility for disaster. What if I have multiple processes (or multiple threads) accessing the same file.
How can I make sure they don't interfere.

Let's say I have process A and B both trying to update the file.

There can be a possibility that process A read the content and while it is still working out what  to write to the file process B
also reads the content. The A write the updated data including the original data + what A has to add, and then B overwrites this
with the original data + what B has to add. This means we lose the addition of A.

In the simple case when both just try to increment a counter, this might mean that both A and B read the value which is, let' say 3.
Both increment by one and both write back 4. That way although we were expecting 5 the actual number is still only 4.

It might also happen that B is trying to read the file while A is in the middle of writing it in which case B might see an empty or
a partially written file.

## Not good.

If the two processes rarely update the shared file you won't encounter the problem.
As the frequency increases it is more and more likely to happen.

So how to avoid this problem?

## Windows

I have not checked this on Windows, but as far as I recall when you open a file on Windows the operating system locks it and other
processes cannot access it. So there the solution is to open the file for both reading and writing at the same time and close it
only when it is updated. The other process then will have to wait before it can touch it.


## Linux / MacOS

On Unix-like system, so both Linux and MacOS, there is no way to lock a file, but there is something called "advisory lock".
The "standard" explanation is that they work like traffic lights. If everyone abides by them then there is no collision, but if even
just one of the processes misbehave then there is a chance for collision.

This is usually called **flock** as in **file lock**.

As far as I can see Rust does not provide the locking API in the standard library, but I found several crates that offer to do it.
First I tried [file-lock](https://crates.io/crates/file-lock/), but my experiment failed.
I opened an [issue](https://github.com/alfiedotwtf/file-lock/issues/12) to figure out if I was using it incorrectly or if there is a bug.
I'll update this article once I get a response.

Then I tried [advisory-lock](https://crates.io/crates/advisory-lock) and that worked.

Here is how we can use it:

We use the [options](https://doc.rust-lang.org/std/fs/struct.File.html#method.options) you might have seen in the article about
[appending to a file](/append-to-a-file) to open the file for both reading and writing. I also turned on the `create` flag
so the file will be created if it did not exist earlier, this is not needed if you are sure the file exists.

Then we call the `lock` method.

```rust
let mut file =  File::options().read(true).write(true).create(true).open(filename)?;
file.lock(FileLockMode::Exclusive)?;
```

Then we read the content of the file. This can be done probably in several different ways.

```
let res = file.read(&mut buffer)?;
let content = String::from_utf8(buffer[0..res].to_vec())?;
```


When we are done updating he "content" we rewind the file using the [Seek](https://doc.rust-lang.org/std/io/trait.Seek.html) trait
and then we **truncate** the file to have 0 length using the `set_len` method.

The rewinding allows us to write the new content from the beginning of the file.

Truncating the file is necessary, otherwise, if the new content is shorter than the old content we will be left with some garbage at the end of the file.


```rust
file.seek(SeekFrom::Start(0))?;
file.set_len(0)?;
```

Finally we write the updated content to the file.

```rust
file.write_all(content.as_bytes())?;
```

When the `file` variable goes out of scope the [drop](/drop-the-destructor-of-rust-structs) method will close the file and free the lock.

## Using the example

This specific example will accept a string on the command line, update the "message.txt" file with the text printing the previous content of the file.

```
cargo run -q "first content that is not very short"
cargo run -q "short content"
cargo run -q "some longer content"
```


## Dependencies

{% include file="examples/update-locked-file-advisory-lock/Cargo.toml" %}

## The code


{% include file="examples/update-locked-file-advisory-lock/src/main.rs" %}




