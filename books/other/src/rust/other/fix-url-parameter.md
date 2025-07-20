# Fix URL parameter

* trim_end_matches
* url

* The user can provide a URL, but I would like to be flexible and accept both with and without a trailing slash:
* https://rust.code-maven.com
* https://rust.code-maven.com/

At first I tried some over-engineered solutions, till I got the recommendation to use `trim_end_matches`.

{% embed include file="src/examples/other/fix-url-string/src/main.rs" %}

However using the [url](https://crates.io/crates/url) crate might be the best solution in this case:

{% embed include file="src/examples/other/fix-url/Cargo.toml" %}
{% embed include file="src/examples/other/fix-url/src/main.rs" %}


