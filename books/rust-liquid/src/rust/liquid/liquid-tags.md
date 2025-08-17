# Liquid tags


* `{% something %}` is called a tag in Liquid.
* There are several built-in tags in Liquid: `if`, `else`, `endif`, `for`, `assign` etc.
* You can also define your own tags. In the next few examples we are going to do that.

* `{% tagname %}` just a tagname.
* `{% tagname value %}` tag with a single value.
* `{% tagname number number %}` tag with two numbers.
* `{% youtube apple banana peach %}` tag with one or more values.
* `{% tagname key="value" %}` or `{% tagname key=42 %}` tag with single key-value pair.
* `{% youtube id="K6EvVvYnjrY" filename="some_name.mp4" %}` tag with two key-value pairs. The second one optional.
* `{% include file="example/code.py" %}` override the built-in `include` tag.
* Use scalar values passed to the render function.
* Use vector passed to the render function.
* `{% latest limit=5 %}` tag with key-value where the value must be a `u8`.
* `{% latest limit=3 tag="programming"  %}`  tag with key-value (where the value is a u8) and an optional key-value pair.



* TODO:
* `{% tagname value value ... %}` tag with multiple values
* `{% tagname key=value key=value %}` tag with multiple key-value pairs


