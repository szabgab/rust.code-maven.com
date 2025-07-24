
## run clippy on every example

./check.sh


## Remove the target folders

rm -rf  examples/*/target


## Update the dependencies in the Cargo.lock files.

./update.sh

## Update the dependencies in the Cargo.toml files.

TODO

## Add slides about these topics:

* Cow
* Box
* Create my own error types
* Anyhow
* iter() vs into_iter()   when would we want to use either of those?
* Create your own trait and implement it for some types
* Threads: Show why we cannot create counter without mutex or scope
* Threads: How can we mutate variable in Mutex that was not marked as `mut`?

