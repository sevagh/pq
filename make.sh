#!/usr/bin/env bash

set -o errexit
set -o errtrace
set -o pipefail
set -o nounset

function gen_schemata() {
    protoc --rust_out ./src/schemata ./schemata/*.proto
}

function run_cargo() {
    touch src/main.rs
    cargo build $1
}

gen_schemata
run_cargo $1
