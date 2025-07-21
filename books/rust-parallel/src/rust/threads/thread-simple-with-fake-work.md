# Simple thread (with fake work)

* We can easily start a new thread using the [spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html) function of the [thread](https://doc.rust-lang.org/std/thread/) crate.
* We even have two loops one in the main thread and one in the created thread to "do some work". It seems to work.
* There is a slight problem though. Our main program might end before the thread can do the actual work and this example we don't even see that.

{% embed include file="src/examples/threads/simple-while-main-is-working/src/main.rs" %}
{% embed include file="src/examples/threads/simple-while-main-is-working/out.out" %}


