#!/usr/bin/bash -ex

cd preprocessing/user-groups
cargo run
cd -

cd preprocessing/companies
cargo run
cd -

