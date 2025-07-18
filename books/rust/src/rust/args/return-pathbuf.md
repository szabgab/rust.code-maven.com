# Default path - return PathBuf

* PathBuf
* args

* The user must supply the path to root and optionally the path to the pages.
* If the user does not supply the path to the pages then we use root/pages

{% embed include file="src/examples/args/default-path/src/main.rs" %}

Returning the second parameter as the path to pages:

```
$ cargo run one two

root: one
page: "two"
```

Returning root/pages:

```
$ cargo run one
root: one
page: "one/pages"
```


