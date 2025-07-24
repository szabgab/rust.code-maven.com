#!/usr/bin/bash -ex

mkdir -p _site/books

for full_book in books/*
do
    if [ -d "$full_book" ]; then
        book=$(basename "$full_book")
        echo Book $book

        cd books/$book
        mdbook build
        perl -i -p -e 's{href="index.html"}{href="."}g'           book/toc.js
        perl -i -p -e 's{(href="[a-zA-Z0-9.][^"]+)\.html"}{$1"}g' book/toc.js
        find book/ -name *.html | xargs perl -i -p -e 's{href="index.html"}{href="."}g'
        find book/ -name *.html | xargs perl -i -p -e 's{(href="[a-zA-Z0-9.][^"]+)\.html"}{$1"}g'
        cd ../..

        mv books/${book}/book _site/$book
    fi

done




