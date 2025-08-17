# Liquid: Embed HTML - escape HTML


By default liquid inserts values as they are.

This means if a value we use in a template contains any HTML special character, that will be included in the resulting HTML. This can break the HTML and can open your site to HTML injection attack.

We can use the **escape** filter on each field where we would like to avoid this.

{% embed include file="src/examples/liquid/embed-html-tags/src/main.rs" %}

{% embed include file="src/examples/liquid/embed-html-tags/out.out" %}


