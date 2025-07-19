# Rocket - Two applications in separate files

* TODO

* We created a separate file with its own routes
* We then mounted it under a path called /blog
* We provide a function called `routes` listing all the routes in this applcation and use that in the `mount`.

Limitation of this solution:

* in the `blog_test` we need to use `super::super::rocket()` instead of `super::rocket()`.
* in the `blog_test` we need to access `/blog` that mean we need to know where it will be mounted.

{% embed include file="src/examples/rocket/separate-files/src/main.rs" %}
{% embed include file="src/examples/rocket/separate-files/src/tests.rs" %}

{% embed include file="src/examples/rocket/separate-files/src/blog.rs" %}
{% embed include file="src/examples/rocket/separate-files/src/blog/blog_tests.rs" %}


