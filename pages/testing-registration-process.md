---
title: Testing registration with email verification process
timestamp: 2025-12-25T16:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - lettre
---

In many web-based application the process of registration includes a step of email verification.
The process is more or less this:

1. The user types in an email address (and some other date)
1. The system checks that the looks ok, including the format of the email address. (e.g. that it is not empty)
1. Then it generates a unique code and stores all the received data and the unique code in the database.
1. Then it send an email to the given address containing a URL with the unique code.
1. When the user receives the email and clicks on the link the web system receives this verification request and can update the records marking the email as verified.

There can and probably should be a number of extra things in the process.
* For example the database should contain an expiration date for the code.

The question though, how do you test this code? Do you let the system send out real emails on every run? That would be slow and the test would need to rely an external service that will receive the email from where the test can download it for verification.

One solution for this is to save the email message in a file during the tests and make the tests read that file.

This is what you can see in this similified example.

## Cargo.toml

{% include file="examples/testing-email-sending/Cargo.toml" %}


## main.rs

{% include file="examples/testing-email-sending/src/main.rs" %}

## Config test

We can either use

```rust
#[cfg(test)]
```

and

```rust
#[cfg(not(test))]
```

or we can use

```rust
if cfg!(test) {
    ...
} else {
    ...
}
```

