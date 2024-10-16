# liquid templating
{id: liquid}

## What is a template engine?
{id: liquid-what-is-a-template-engine}

* We would like to create several texts (e.g. web pages, content of email, some report) that look exactly the same but will have different data.
* We design the page, but instead of values we use vairables (or placeholders, if you prefer that word) that look like this: `{{ ttitle }}`.
* The template engine then can replace those placeholder variables by the appropriate value.

* Besides replacing individual values, a template system usually has more complex syntax as well. Loops, conditionals, includes etc.

## Install
{id: install-liquid}

* Based on original [liquid written in Ruby](https://shopify.github.io/liquid/) (published by Shopify)
* The [liquid crate](https://crates.io/crates/liquid)
* [documentation](https://docs.rs/liquid/)

```
cargo add liquid
```

This will update the Cargo.toml to include:

```
[dependencies]
liquid = "0.26.4"
```

## Liquid use-cases
{id: liquid-use-cases}

* The [Virtual events site](https://events.code-maven.com/) - [source](https://github.com/szabgab/virtual-events)
* The [Rust Maven site](https://rust.code-maven.com/) uses the [Code Maven Static Site Generator](https://ssg.code-maven.com/) - [source](https://github.com/szabgab/code-maven.rs)
* The [Rust Digger](https://rust-digger.code-maven.com/) - [source](https://github.com/szabgab/rust-digger)

## Liquid Hello World
{id: liquid-hello-world}
{i: parse}
{i: build}
{i: object!}
{i: render}

* Depened on the liquid crate

![](examples/liquid/liquid-hello-world/Cargo.toml)

* Start with a template that is part of the Rust source code.
* We use the `parse` and `build` methods to create the template object.
* We use `unwrap` here which is probably not ideal, but it simlifies the examples.
* Using the `liquid::object!` macro we create an object from the data we would like to pass to the template.
* Using the `render` method we combine the data with the template and generate (render) the resuling text.


![](examples/liquid/liquid-hello-world/src/main.rs)
![](examples/liquid/liquid-hello-world/out.txt)

## Liquid Hello World with variable
{id: liquid-hello-world-with-variable}

* Using the same template as earlier we see how we can reuse the template in 3 different ways:

* The value of "name" is hard-coded in the call to `object!`
* The value of "name" is hard-coded in a variable as `str`.
* The value of "name" is a `String` that could come from the outside world (e.g. from a file).

![](examples/liquid/liquid-hello-world-variables/out.txt)
![](examples/liquid/liquid-hello-world-variables/src/main.rs)

## Liquid Hello World read template from file
{id: liquid-hello-world-read-template-from-file}
{i: parse_file}

* Templates are usually much biggger than what we had in the first example.
* We usually prefer to keep the templates as an external files.
* Instead of `parse` we can use `parse_file` to load the template from an external file.
* This happens at run-time so you will need to supply the templates outside of the binary or the user will need to create the templates.

![](examples/liquid/liquid-hello-world-from-file/out.txt)
![](examples/liquid/liquid-hello-world-from-file/src/main.rs)
![](examples/liquid/liquid-hello-world-from-file/template.txt)

## Liquid Hello World embed template file
{id: liquid-hello-world-embed-template-file}
{i: parse}
{i: include_str!}

* If you would like to supply the temlates, probably the easiest is to embed them in the binary.
* Using `include_str!` we can embed a text-file in the compiled binary of our Rust code.
* In the source repository we have the templates as external files, but during `build` they are embedded in the code.

![](examples/liquid/liquid-hello-world-embedded-file/src/main.rs)

![](examples/liquid/liquid-hello-world-embedded-file/template.txt)


## Liquid flow control: if - else
{id: liquid-flow-control-if-else}
{i: if}
{i: else}
{i: endif}

* Liquid has simple conditionals: `if` that we end with `endif` and the optional `else`.

![](examples/liquid/liquid-if-else/src/main.rs)
![](examples/liquid/liquid-if-else/out.txt)

## Liquid flow control: else if written as elsif
{id: liquid-flow-control-elsif}
{i: elsif}

![](examples/liquid/liquid-elsif/src/main.rs)
![](examples/liquid/liquid-elsif/out.txt)

## Liquid flow control: case/when
{id: liquid-flow-control-case-when}
{i: case}
{i: when}
{i: endcase}

* the `case` statement ends with `endcase`.

![](examples/liquid/liquid-case-when/src/main.rs)
![](examples/liquid/liquid-case-when/out.txt)

## Liquid passing more complex data
{id: liquid-passing-more-complex-data}

![](examples/liquid/liquid-objects/src/main.rs)
![](examples/liquid/liquid-objects/out.txt)

## Liquid for loop passing a vector or an array
{id: liquid-for-loop}
{i: for}
{i: endfor}

* We are probably more interested in passing values from variables.
* In this example we pass a vector of strings.

![](examples/liquid/liquid-for-loop/src/main.rs)
![](examples/liquid/liquid-for-loop/out.txt)

## Liquid vector of tuples
{id: liquid-vector-of-tuples}

* Another example passing in a vector, but this time a vector of tuples.
* We use the square-brackets `[]` and indexes to access the elements of a tuple.

![](examples/liquid/liquid-vector-of-tuples/src/main.rs)

![](examples/liquid/liquid-vector-of-tuples/out.txt)


## Liquid HashMap
{id: liquid-hash}
{i: HashMap}

* We can pass in a HashMap and inside we can iterate over the key-value pairs as tuples.
* So here too we use the `[]` with index 0 and 1 to access the key and the value.

![](examples/liquid/liquid-hashmap/src/main.rs)

![](examples/liquid/liquid-hashmap/out.txt)

## Liquid for loop with if conditions
{id: liquid-for-loop-with-if-conditions}
{i: for}
{i: endfor}
{i: if}
{i: endif}

![](examples/liquid/liquid-loop-and-if/src/main.rs)
![](examples/liquid/liquid-loop-and-if/out.txt)

## Liquid with struct
{id: liquid-with-struct}
{i: serde}

![](examples/liquid/liquid-with-struct/Cargo.toml)
![](examples/liquid/liquid-with-struct/src/main.rs)
![](examples/liquid/liquid-with-struct/out.txt)


## Liquid with Option in a struct
{id: liquid-with-option-struct}
{i: Option}

![](examples/liquid/liquid-with-option/Cargo.toml)

![](examples/liquid/liquid-with-option/src/main.rs)


## Liquid include
{id: liquid-include}
{i: include}
{i: partials}
{i: read_to_string}

![](examples/liquid/liquid-include/templates/page.txt)
![](examples/liquid/liquid-include/templates/incl/header.txt)

![](examples/liquid/liquid-include/src/main.rs)

![](examples/liquid/liquid-include/out.txt)

## Liquid include header and footer
{id: liquid-include-header-and-footer}
{i: include}

![](examples/liquid/liquid-include-header-footer/out.txt)
![](examples/liquid/liquid-include-header-footer/src/main.rs)
![](examples/liquid/liquid-include-header-footer/templates/incl/footer.txt)
![](examples/liquid/liquid-include-header-footer/templates/incl/header.txt)
![](examples/liquid/liquid-include-header-footer/templates/page.txt)


## Liquid layout (include and capture)
{id: liquid-layout}
{i: include}
{i: capture}

![](examples/liquid/liquid-layout/out.txt)
![](examples/liquid/liquid-layout/src/main.rs)
![](examples/liquid/liquid-layout/templates/layout.txt)
![](examples/liquid/liquid-layout/templates/page.txt)

## Liquid assign to variable in template
{id: liquid-assign}
{i: assign}

![](examples/liquid/liquid-assign/src/main.rs)

![](examples/liquid/liquid-assign/templates/page.txt)
![](examples/liquid/liquid-assign/templates/incl/header.txt)

![](examples/liquid/liquid-assign/out.txt)

## Liquid filters on strings: upcase, downcase, capitalize
{id: liquid-filters-strings}
{i: upcase}
{i: downcase}
{i: capitalize}

![](examples/liquid/liquid-filters-strings/src/main.rs)
![](examples/liquid/liquid-filters-strings/out.txt)


## Liquid filters on numbers: plus, minus
{id: liquid-filters-numbers}
{i: plus}
{i: minus}

* Some filters can have parameters as well.
* Increment or decerement the number by the given number.

![](examples/liquid/liquid-filters-numbers/src/main.rs)
![](examples/liquid/liquid-filters-numbers/out.txt)

## Liquid filters: first, last
{id: liquid-filters-first-last}
{i: first}
{i: last}

first or last
* character in a string
* element in an array, a vector, or a tuple

![](examples/liquid/liquid-filters-order/src/main.rs)
![](examples/liquid/liquid-filters-order/out.txt)



## Liquid filter reverse array
{id: liquid-filter-reverse-array}

![](examples/liquid/liquid-filter-reverse/src/main.rs)

## Liquid for loop: limit, offset, reversed
{id: liquid-for-loop-limit-offset-reversed}
{i: limit}
{i: offset}
{i: reversed}

![](examples/liquid/liquid-loops/src/main.rs)
![](examples/liquid/liquid-loops/out.out)

## Liquid comma between every two elements (forloop.last)
{id: liquid-forloop-last}

* length
* index   (numbers start from 1)
* index0  (numbers start from 0)
* rindex
* rindex0
* first
* last

![](examples/liquid/liquid-loop-last/src/main.rs)
![](examples/liquid/liquid-loop-last/out.out)

## Liquid: create your own filter: reverse a string
{id: liquid-create-your-own-filter-reverse-a-string}

This is using the [liquid-filter-reverse-string](https://crates.io/crates/liquid-filter-reverse-string). Look at its [source code](https://github.com/szabgab/liquid-filter-reverse-string.rs)

![](examples/liquid/liquid-filter-reverse-string-use/Cargo.toml)
![](examples/liquid/liquid-filter-reverse-string-use/src/main.rs)

## Liquid: create your own filter: commafy
{id: liquid-create-your-own-filter-commafy}

![](examples/liquid/liquid-filter-commafy-use/Cargo.toml)
![](examples/liquid/liquid-filter-commafy-use/src/main.rs)

## Liquid: length of string, size of vector
{id: liquid-length-size}
{i: len}
{i: size}

* Sometimes we would like to display or compare the length of a string or the number of elements in a vector.
* We can do that using the `size` attribute.

![](examples/liquid/liquid-size/src/main.rs)
![](examples/liquid/liquid-size/out.out)

## Liquid: Embed HTML - escape HTML
{id: liquid-embed-html-escape-html}

By default liquid inserts values as they are.

This means if a value we use in a template contains any HTML special character, that will be included in the resulting HTML. This can break the HTML and can open your site to HTML injection attack.

We can use the **escape** filter on each field where we would like to avoid this.

![](examples/liquid/embed-html-tags/src/main.rs)

![](examples/liquid/embed-html-tags/out.txt)

## Liquid TODO
{id: liquid-todo}

* pass integer, string, bool, vector, tuple, struct
* Control structures (if, else)
* Loops (for)
* Filters
* Include template
* Layout template (embed template)


