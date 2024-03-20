---
title: Strings and memort reallocation
timestamp: 2024-03-20T22:30:01
author: szabgab
published: true
description:
tags:
    - push
    - push_str
    - as_ptr
    - ":p"
todo:
    - TODO
---

{% youtube id="UqVgTafRCCU" %}


{% include file="examples/strings-and-memory-reallocation/src/main.rs" %}


```
 0  0 0x7fff828f9e70             0x1 ''
 1  8 0x7fff828f9e70  0x5ed486472ba0 'a'
 6  6 0x7fff828fa5b8  0x5ed486472bc0 'foobar'
 2  8 0x7fff828f9e70  0x5ed486472ba0 'ab'
 8  8 0x7fff828f9e70  0x5ed486472ba0 'ab123456'
 9 16 0x7fff828f9e70  0x5ed486472ba0 'ab123456x'
33 33 0x7fff828f9e70  0x5ed486472be0 'ab123456x123456789123143274368741'
```

