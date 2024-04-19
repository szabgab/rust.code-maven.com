# [Rust Maven](https://rust.code-maven.com/)


This repostory contains the source of all the articles and all the examples of the [Rust Maven](https://rust.code-maven.com/) web site.

It now also includes all the slides in the `slides` folder.

Feel free to use any of the examples under the MIT OR Apache-2.0 license.

## Generate the site

Download `code-maven` from https://ssg.code-maven.com/

Run

```
code-maven web
```

It will generate the site in the `_site` folder.


## In order to generate the slides

* Clone slider-py

```
ln -s ../slider-py
```

```
cd slides

ln -s ../../slides/templates
ln -s ../../slides/static/
ln -s ../../slides/generate_slides.py
cd ..
```

```
./generate_slides.py
```

## View site locally

Install [rustatic](https://rustatic.code-maven.com/)

and after generating the static pages with the previously described command, run

```
rustatic --host localhost --port 5000 --indexfile index.html --nice --path _site/
```

## Clean up all the target folders

find -name target | xargs rm -rf
