# Testing speed improvements with threads

* A CPU intensive task - computing the Fibonacci numbers up to N 10 times.
* Once in a single threaded process and once in a multi-threaded process with 10 threads.

Results

```
N    single     multi
     thread     thread
40:  7.1 sec vs 1.3 sec
41: 11.4 sec vs 2.1 sec
42: 18.1 sec vs 3.3 sec
```

{% embed include file="src/examples/threads/speed-test/src/main.rs" %}
{% embed include file="src/examples/threads/speed-test/out.out" %}



