---
title: What is the file extension if the filename has more than one dots in them?
timestamp: 2024-04-09T09:18:01
author: szabgab
published: true
description:
tags:
    - extension
---

I had a bug in one of my applications so I was wondering what happens if a filename has more than one dot in them. e.g. "hello.world.txt" Will the `extension` method return `txt` as I'd expect?

The answer is: yes.

My bug is somewhere else.

{% include file="examples/extenstion-for-files-with-dot/src/main.rs" %}

```
"code.rs" "rs"
"my-data-1.70.html" "html"
```


