# Tera
{id: tera}

## Tera Template system
{id: tera-template-system}

* [Tera](https://keats.github.io/tera/)
* Used in [Rocket](https://rocket.rs/)

## Tera Hello World
{id: tera-hello-world}

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


