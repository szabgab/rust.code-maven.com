---
title: YAML and Rust
timestamp: 2023-12-18T15:30:01
author: szabgab
published: true
description: YAML is a file format often used as a configuration file. Most of the programming languages have a way to deserialize YAML into some internal data structure. So does Rust via serde.
tags:
    - YAML
    - serde
todo:
    - complex data
    - optional fields
    - field that can either be a single value or a list
    - notice typo in a field
---

[YAML](https://yaml.org/) is a human-readable and human writable file format often used for configuration.
I maintain several project where people collect data into thousands of YAML files and the some program collects the data and generats a web site.

This is a collection of articles on dealing with YAML in the Rust programming language.

* [Read arbitrary YAML files in Rust ](/read-arbitrary-yaml) without the need to know the whole structure of the file. `serde`, `serde_yaml`
* [Read a simple, flat YAML file into a struct](/read-simple-yaml) `struct`, `Deserialize`
* [Set default values while deserializing YAML](/default-values-deserializing-yaml) `default`
* [Deserializing YAML - deny unknown fields](/yaml-deny-unknown-fields) `deny_unknown_fields`
