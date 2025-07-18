# map is lazy that can cause problems

* We increment the counter and that's how we get the increasing numbers next to the letters.

{% embed include file="src/examples/vectors/map1/src/main.rs" %}
{% embed include file="src/examples/vectors/map1/out.out" %}

* If we call `rev` before we call `map` then the letters will be reversed, and the numbers will be attached to the letters after the reversal.

{% embed include file="src/examples/vectors/map2/src/main.rs" %}
{% embed include file="src/examples/vectors/map2/out.out" %}

* If we first call `map` and only then `rev` we would expect that first the numbers are generated and then the wholething is reversed, but that's not the case.

```
c 3
b 2
a 1
```

* Because `map` is lazy it will be executed only after the reversal. Just as in the previous case.


{% embed include file="src/examples/vectors/map3/src/main.rs" %}
{% embed include file="src/examples/vectors/map3/out.out" %}


