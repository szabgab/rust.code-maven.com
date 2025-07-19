# Rocket - Path parameters

Instead of passing parameters in the Query string in a GET request we can also use the path to pass parameters.
This is especially interesgint if we would like to make the pages indexable by search engines.

* e.g. in a blog engine the path can be mapped to a blog entry
* In a social site we might want to have a separate page for each users.

{% embed include file="src/examples/rocket/path-parameters/src/main.rs" %}
{% embed include file="src/examples/rocket/path-parameters/src/tests.rs" %}


