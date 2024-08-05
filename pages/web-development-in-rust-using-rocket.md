---
title: Web development in Rust using Rocket
timestamp: 2024-06-18T10:50:01
author: szabgab
published: true
description: Video
tags:
    - Rocket
---

{% youtube id="xaPZhZ-o2bI" %}

```
00:00 Introduction (Code-Maven, Perl Maven).
00:56 Self Introduction.
01:49 Meetup page about the event.
02:45 Sponsors
03.12 How does Rocket compare to  Axum or Actix-Web?
04:35 The website of Rocket.
05:50 Workshops / events.
06:55 Link to the articles about Rocket https://rust.code-maven.com/rocket
09:33 Cargo generate https://github.com/cargo-generate/cargo-generate
10:20 Rocket starter https://crates.io/crates/rocket-starter
11:23 rocket-starter demo1 --simple
13:50 Explaining about pathes.
15:24 Routing.
16:10 Making changes to the code, restarting the development web server. (using Ctrl-c; Ctrl-r)
16:45 Adding a second route.
20:00 Why do we need to mount the routes to the application? sub-applications (microservices).
24:55 What is the relationship between the login and the blog?
26:20 HTTP protocol the GET and POST requests.
32:00 Tests.
35:55 Content-type text/plain
36:45 Using curl -i sending a GET request.
37:55 Using curl -I sending a HEAD request.
41:40 How would you hide a password?
44:30 rocket-starter demo2 --tera1
45:25 The Tera template system. Template variables.
46:50 The evolution of project creating a new template system.
48:15 Jinja of Python Flask.
48:45 Cargo.toml
49:40 The route returning a Template. context!
51:45 Test - The Content-type is text/html.
52:00 curl
53:00 Handle page not found errors - 404 pages - using catch(404).
55:00 Set values inside the template.
56:00 attach Template::fairing and use carchers! to make the 404 catcher work.
58:28 Testing the main page.
59:15 Can this "Not found" page be translated into other languages depending on region IP address?
1:00:10 Does Rocket sit on top of Tokio? Does it work in async mode?
1:00:54 Testing the 404 page.
1:03:10 rocket-starter demo3 --tera2
1:03:37 Cargo.toml using serde as well to be able to use Rocket.toml to add configuration options.
1:05:00 serde::Deserialize, rocket::fairing::AdHoc
1:07:15 Rocket Discussions: https://github.com/rwf2/Rocket/discussions
1:08:20 rocket-starter demo4 --tera-module
1:11:07 Some more examples from https://rust.code-maven.com/rocket
1:11:35 Echo using GET request. Query String parameters.
1:15:41 Echo using POST request.
1:18:17 What happens when a POST request does not have all the fields?
1:19:39 Sending a POST request using curl.
```
