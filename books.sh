#!/usr/bin/bash -ex

mkdir _site/books

for book in why-rust regex json
do
    echo Book $book

    ./mdbook build books/$book
    perl -i -p -e 's{href="index.html"}{href="."}g'           books/${book}/book/toc.js
    perl -i -p -e 's{(href="[a-zA-Z0-9.][^"]+)\.html"}{$1"}g' books/${book}/book/toc.js
    find books/${book}/book/ -name *.html | xargs perl -i -p -e 's{href="index.html"}{href="."}g'
    find books/${book}/book/ -name *.html | xargs perl -i -p -e 's{(href="[a-zA-Z0-9.][^"]+)\.html"}{$1"}g'

    mv books/${book}/book _site/$book

done




