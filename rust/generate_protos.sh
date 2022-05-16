#!/bin/bash
set -e
set -x

CURRENT_DIR=${PWD##*/}
if [ "$CURRENT_DIR" != "rust" ]; then
    echo "run this script rust directory, current directory is ${CURRENT_DIR}"
    exit 1
fi

mkdir src
protoc -I=../proto -I=/usr/local/include --rust_out=./src cacheclient.proto controlclient.proto momento/auth.proto

pushd src
    echo "pub mod cacheclient;" >> lib.rs
    echo "pub mod controlclient;" >> lib.rs
    echo "pub mod auth;" >> lib.rs
popd
