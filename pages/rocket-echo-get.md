---
title: Rocket - echo GET
timestamp: 2024-02-01T11:30:01
author: szabgab
published: true
description: An HTTP GET request with a Query string parameter and a template.
tags:
    - Rocket
    - web
    - GET
---

PArt of the [Rocket](/rocket) series, expecting a single parameter as Query String.

{% include file="examples/rocket/echo-get/Cargo.toml" %}

{% include file="examples/rocket/echo-get/src/main.rs" %}

{% include file="examples/rocket/echo-get/src/tests.rs" %}

{% include file="examples/rocket/echo-get/templates/echo.html.tera" %}

{% include file="examples/rocket/echo-get/templates/index.html.tera" %}

