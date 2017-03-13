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

if [ z"${1-}" == z"build" ]; then
    clean_gen_code
    gen_schemata
    touch src/main.rs
    cargo build
elif [ z"${1-}" == z"clean" ]; then
    clean_gen_code
elif [ z"${1-}" == z"test" ]; then
    cargo test
else
    printf "usage: $0 build|clean|test\n" $0
    exit 255
fi
