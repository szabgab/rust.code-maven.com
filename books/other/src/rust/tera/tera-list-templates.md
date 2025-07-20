# Tera - list templates


I am not sure if this is ever needed in an application, but it helped me when first I could not figure out
the expected directroy layout. It might be used for debugging to see if Tera can really find your templates.

```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── out.out
├── src
│   └── main.rs
└── templates
    ├── hello.html
    └── incl
        └── header.html
```

{% embed include file="src/examples/tera/list-templates/src/main.rs" %}
{% embed include file="src/examples/tera/list-templates/out.out" %}



{% embed include file="src/examples/tera/list-templates/Cargo.toml" %}
{% embed include file="src/examples/tera/list-templates/templates/hello.html" %}
{% embed include file="src/examples/tera/list-templates/templates/incl/header.html" %}



