---
title: Rocket - process Query String of a GET request
timestamp: 2024-02-01T14:30:01
author: szabgab
published: true
description: How to handle parameters received on the URL of a GET request.
tags:
    - Rocket
    - web
    - GET
todo:
    - write text
    - use an enum to list the possible values of the shape
---

Part of the [Rocket](/rocket) series.


{% include file="examples/rocket/query-string/Cargo.toml" %}
{% include file="examples/rocket/query-string/src/main.rs" %}
{% include file="examples/rocket/query-string/src/tests.rs" %}
{% include file="examples/rocket/query-string/templates/index.html.tera" %}
{% include file="examples/rocket/query-string/templates/show.html.tera" %}

## TODO

* Add more data types to the form and how we handle it
* Add checkbox with multiple values.
