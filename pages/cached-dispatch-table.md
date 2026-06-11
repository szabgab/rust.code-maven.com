---
title: Cached dispatch table
timestamp: 2026-06-11T13:00:01
author: szabgab
published: true
show_related: true
description:
tags:
    - cached
    - HashMap
    - Dispatch
---

We have lots of functions handling various cases.
These are divided into multiple modules based on the types of the cases.

The nice thing seem to be is creating a dispatch table mapping strings to functions.

Each module is responsible for returning a DispatchTable. There is a central function that
combines them.

We could generate this up-front and then pass the DispatchTable to each function but it seemed
better to get the DispatchTable internally.

If we do that we'll recreate the DispatchTable in every call. Using the [cached crate](https://crates.io/crates/cached)
allows us to eliminate all the calls but the first one.

{% include file="examples/cache-dispatch-table/src/main.rs" %}

{% include file="examples/cache-dispatch-table/src/locations.rs" %}

{% include file="examples/cache-dispatch-table/src/other.rs" %}

{% include file="examples/cache-dispatch-table/src/types.rs" %}


{% include file="examples/cache-dispatch-table/out.txt" %}

{% include file="examples/cache-dispatch-table/Cargo.toml" %}








