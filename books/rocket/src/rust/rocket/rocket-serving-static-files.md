# Rocket - Serving static files

* relative
* FileServer

* We can use the FileServer to return static files such as images, css files, javascript files etc.
* We need to mount it to "/".
* It sets the content type properly for each file.

{% embed include file="src/examples/rocket/static-files/src/main.rs" %}

{% embed include file="src/examples/rocket/static-files/src/tests.rs" %}

{% embed include file="src/examples/rocket/static-files/static/css/style.css" %}
{% embed include file="src/examples/rocket/static-files/static/js/demo.js" %}


