# Rocket - Return Status (arbitrary HTTP code)

* Returning a Status allows us to either return some content or return an arbitrary HTTP status code
* Then we can - if we want - setup a catcher for that error code to show content we would like to show.

{% embed include file="src/examples/rocket/return-result/src/main.rs" %}
{% embed include file="src/examples/rocket/return-result/src/tests.rs" %}


