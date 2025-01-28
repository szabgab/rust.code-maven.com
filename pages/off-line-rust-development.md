---
title: Off-line Rust Development
timestamp: 2025-01-27T11:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - crates
    - cargo
todo:
---

Throughout the years I worked with a number of organizations where even the development network was not connected to the Internet.

It made the installation of 3rd party libraries a nightmare. In some cases we had to use an external computer to manually  download each one of the libraries recursively dealing with all the dependencies.  Then write them on a CD and make that CD available  on the internal network

I wonder if there are any tools in Rust supporting such a development environment that would eliminate most of the manual work?

Explore [panamax](https://github.com/panamax-rs/panamax)

