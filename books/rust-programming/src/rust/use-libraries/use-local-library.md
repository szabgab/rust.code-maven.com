# TODO: Crate library and use local library

* dependencies
* path


* Create a library:

```
cargo new add-lib --lib
```

The files created:

{% embed include file="src/examples/libraries/add-lib/Cargo.toml)
{% embed include file="src/examples/libraries/add-lib/src/lib.rs" %}

* Create another crate that will use this library:

```
cargo new add-app
cd add-app
cargo add --path ../add-lib/
```

{% embed include file="src/examples/libraries/add-app/Cargo.toml)
{% embed include file="src/examples/libraries/add-app/src/main.rs" %}


