# Tera
{id: tera}

## Tera Template system
{id: tera-template-system}

* [Tera](https://keats.github.io/tera/)
* Used in [Rocket](https://rocket.rs/)

## Tera Hello World
{id: tera-hello-world}
{i: Context}
{i: render}

![](examples/tera/hello-world/Cargo.toml)
![](examples/tera/hello-world/src/main.rs)
![](examples/tera/hello-world/templates/hello.html)

```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── templates
    └── hello.html
```

## Tera - list templates
{id: tera-list-templates}

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

![](examples/tera/list-templates/src/main.rs)
![](examples/tera/list-templates/out.out)



![](examples/tera/list-templates/Cargo.toml)
![](examples/tera/list-templates/templates/hello.html)
![](examples/tera/list-templates/templates/incl/header.html)


## Tera - use built-in filters on strings
{id: tera-use-built-in-filters-on-strings}

![](examples/tera/string-filters/src/main.rs)
![](examples/tera/string-filters/templates/hello.html)

![](examples/tera/string-filters/out.out)

## Tera - create your own filter
{id: tera-create-your-own-filter}
{i: register_filter}
{i: Result}
{i: Tera}
{i: Value}
{i: to_value}

![](examples/tera/create-filter/src/main.rs)
![](examples/tera/create-filter/templates/hello.html)

![](examples/tera/create-filter/out.out)


