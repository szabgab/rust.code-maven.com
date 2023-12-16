---
title: Create QR code using Rust
timestamp: 2023-10-02T08:30:01
published: true
description:
tags:
    - image
    - qrcode
---

It is very easy to create an image containing a QR code using the [qrcode](https://crates.io/crates/qrcode) Rust crate, but as of today
there are some compatibility issues so the example in the README needs a tiny tweaking.

Specifically, the most recent version of [qrcode](https://crates.io/crates/qrcode)  (0.12.0) only works with the 0.23 version of the [image](https://crates.io/crates/image) crate.

So I had to fix the versions in the **Cargo.toml** file:

![](examples/create-qrcode/Cargo.toml)

The code itself does not need to change, but I replaced the content of the QR code with a URL to my web site and saved the file in the same folder where the code is:

![](examples/create-qrcode/src/main.rs)

This is the result, go ahead, scan it.

![](examples/create-qrcode/qrcode.png)

## Issue

In case you'd like to get alerted when this issue is fixed, the incompability issue was already [reported by others](https://github.com/kennytm/qrcode-rust/issues/56). You can follow that issue.

## Exact dependencies

In order to allow you to see the exact versions of all the crates that were involved in the creation of this image, I am also including the **Cargo.lock** file:

![](examples/create-qrcode/Cargo.lock)
