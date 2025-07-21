# Tasks with different processing time

* TODO

Experimental example to show how Rayon distributes the load when the tasks have random processing time
(betwee 1-1000 ms) and/or when there are a few long-running tasks and many short tasks.

{% embed include file="src/examples/rayon/tasks-with-random-time/src/main.rs" %}


