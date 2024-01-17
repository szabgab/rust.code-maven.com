---
title: Generate uuid, a Universally unique identifier in Rust
timestamp: 2024-01-04T09:20:01
author: szabgab
published: true
description: It is very simple to generate a UUID in Rust, but there are a lot more option than I expected.
tags:
    - UUID
    - new_v4
todo:
    - How to create a shorter ID that is unique in this server/dataset?
---

There are many cases when you might want to create a unique identifier. For example when we upload a video to YouTube the video gets an identifier that is unique to YouTube.
For example [w0VFcVYIfhg](https://www.youtube.com/watch?v=w0VFcVYIfhg).

If you let people register on your web site you probably save their information in a database and assign a unique number to their information just as you do with any other piece of data,
but you probably also let them select a username which needs to be **unique** in your system.

If you are inserting data into a relational database you probably have an id for each row. It is usually the next value in an automatically increasing sequence of whole numbers, 1,2,3.
It is usually handled by a service provided by the database.

If you are inserting data into a distributed database, eg. MongoDB, then two pieces of data might be recorded on different servers. The database system cannot use a
centralized place to increase a counter to assign the id. One solution would be to give each server a unique value that would be used as part of the ID. That way
each server can has its own counter and the IDs that are a combination of the server-specific ID and the counter will be unique in the whole system.
Then of course the question arises, how will a new server that joins this cluster of servers get its own unique id?

That's where UUIDs help.

A [Universally Unique Identifier (UUID)](https://en.wikipedia.org/wiki/Universally_unique_identifier)  is a 128-bit label that is going to be unique for practical purposes.

In Rust the [uuid crate](https://crates.io/crates/uuid) makes it easy to generate one.

The README of the crate provides you with the basic usage. However I was surprised to see that there are many versions of UUID and this crate has all kinds of features.

For now, however I only looked at the basic usage as that was enough for me.

## Dependencies

It is not enough to add the uuid crate as dependency, we also need to add one or more features to make the crate usable.

![](examples/generate-uuid/Cargo.toml)

## The code

![](examples/generate-uuid/src/main.rs)

## running the code

```
cargo run -q

uuid v4: 54993b26-ccd5-4fa3-9174-0f0c90cada27
```

## Conclusion

I did not have much to show here beyond what the README already shows, but I think I'll have a few more additions later as I implement [email address validation](/email-address-validation)
for the [Meet-OS](https://meet-os.com/) project.

