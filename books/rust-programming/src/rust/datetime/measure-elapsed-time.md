# Measure elapsed time

* Instant
* now
* elapsed
* std::time::Instant::now
* as_secs
* as_millis

* [std::time](https://doc.rust-lang.org/std/time/index.html)
* The [Instant](https://doc.rust-lang.org/std/time/struct.Instant.html) type allows us to get snapshots if time that can help us measure elapesd time.

* In this example we have a function to decide if a number is a prime number. We only use it to have some code that can take substantial time.
* We get the timestamp before and after and we calculate the elapsed time.

{% embed include file="src/examples/datetime/instant-elapsed/src/main.rs" %}
{% embed include file="src/examples/datetime/instant-elapsed/out.out" %}


