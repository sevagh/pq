#!/usr/bin/env bash

set -o errexit
set -o errtrace
set -o pipefail
set -o nounset

function clean_gen_code() {
    rm -rf ./src/schemata/*.rs
    rm -rf ./src/protob.rs
}

function gen_schemata() {
    protoc --rust_out ./src/schemata ./schemata/*.proto
}

function run_cargo() {
    touch src/main.rs
    cargo build
}

clean_gen_code
gen_schemata
run_cargo
