# [Rust Maven](https://rust.code-maven.com/)


This repostory contains the source of all the articles and all the examples of the [Rust Maven](https://rust.code-maven.com/) web site.

It now also includes all the slides in the `slides` folder.

Feel free to use any of the examples under the MIT OR Apache-2.0 license.


## In order to generate the slides

* Clone slides
* Cloone slider-py

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

