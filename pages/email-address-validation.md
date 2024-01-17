---
title: Email address validation in Rust
timestamp: 2024-01-04T06:30:01
author: szabgab
published: true
description: When accepting and email address in a registration from it is recommended to validate that it is in the correct format and it belongs to the person who typed it in.
tags:
    - validator
    - validate_email
    - email
todo:
    - show how to use as part of a struct while accepting input in a web application
---

The main reason I encountered the need to verify that an email address is valid is when writing web applications and wanting to let people register with their valid e-mail address.

When we are asking if an email address is valid we can ask it on several levels.

1. Is it valid according to the Internet Standards?
2. Is it acceptable at the provider? (e.g. could I register that address on Gmail?)
3. Does this address belong to the person who used it as part of the registration?


There is no tool that can verify that an email address typed in a web form belong to anyone, yet alone to the person who typed it in.
In order to answer the last question we don't have any choice, but to send an e-mail to that address with a unique code, probably embedded in a link and ask the recipient to click on the link or copy-paste the code into a form on our site in order to verify their intention to register on our web site.

In order to verify if the address is a valid address on the specific email hosting service one has to be familiar with the rules of that specific service. However this question is usually only important for the developers of that service.

So what we can do here is to verify that the string we received is valid according to the Internet Standards.

For this we can use the [validator](https://crates.io/crates/validator) crate.

This crate can be used to validate all kinds of input, but we are going to focus on email addresses.

As I can see from the README of the crate the main suggested usage is to be part of the definition of a `struct`, but in this example I used the procedural approach.

## Dependencies

![](examples/validate-email/Cargo.toml)

## Code

![](examples/validate-email/src/main.rs)

The whole thing is calling the `validator::validate_email` function, passing a string to it an receiving a boolean `true` or `false`.

I created two lists, one with valid addresses and one with invalid addresses.

Actually I was surprised to see that characters such as `|` and `/` are valid in the local part of the address, but then I [looked it up](https://en.wikipedia.org/wiki/Email_address).

Note that also "foo@bar.nosuchname" is a valid address despite the fact that no such [top-level domain name](https://en.wikipedia.org/wiki/Top-level_domain) exists.


