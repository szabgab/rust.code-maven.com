---
title: Sort a graph of dependencies using Toplogical Sort
timestamp: 2025-04-08T10:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - sort
---

The [topological-sort](https://crates.io/crates/topological-sort) crate helps you find order in a directed graph.
In other words if you have a bunch of dependencies it helps you untangle the order you need to do them.

## Example: Simplified Spaghetti Bolognese

* Simplified Spaghetti Bolognese depends on Cooked pasta and Sauce
* Cooked pasta depends on Dry Pasta and Boilding Water
* Sauce depends on Pealed Onion, Pepper, Pealed Tomato

Based on this one could establish an order in which things need to be prepared and one could also establish the list of things that can be done in parallel.

## Example: Omlet

Sometimes, however you find yourself in trouble. For example:

* Omlet depends on Egg
* Egg depends on Chicken
* Chicken depends on Egg

Here we don't know if we'll need the Egg or the Chick firts. We have a circular dependency.

## Examples in code

{% include file="examples/try-toplogical-sort/src/main.rs" %}


{% include file="examples/try-toplogical-sort/Cargo.toml" %}

