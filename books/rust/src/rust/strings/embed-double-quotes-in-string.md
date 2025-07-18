# Embed double quotes in string

* String in Rust are inside double quotes.
* It is easy to inlcude single quote in a string as it is not special.
* In order to include a double quote we need to add the escape character, the back-slash, in-front of it.
* Alternatively we can start the string using `r#"` and then we can end it with `"#`. This allows us to freely include double-quote in the string.

{% embed include file="src/examples/strings/embed-double-quotes/src/main.rs" %}
{% embed include file="src/examples/strings/embed-double-quotes/out.out" %}


