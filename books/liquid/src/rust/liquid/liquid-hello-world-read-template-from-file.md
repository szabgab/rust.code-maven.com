# Liquid Hello World read template from file

* parse_file

* Templates are usually much biggger than what we had in the first example.
* We usually prefer to keep the templates as an external files.
* Instead of `parse` we can use `parse_file` to load the template from an external file.
* This happens at run-time so you will need to supply the templates outside of the binary or the user will need to create the templates.

{% embed include file="src/examples/liquid/liquid-hello-world-from-file/out.out" %}
{% embed include file="src/examples/liquid/liquid-hello-world-from-file/src/main.rs" %}
{% embed include file="src/examples/liquid/liquid-hello-world-from-file/template.txt)


