#!/bin/bash
set -e
set -x

CURRENT_DIR=${PWD##*/}
if [ "$CURRENT_DIR" != "rust" ]; then
    echo "run this script rust directory, current directory is ${CURRENT_DIR}"
    exit 1
fi

mkdir src

pushd src
    echo "pub mod cache_client;" >> lib.rs
    echo "pub mod control_client;" >> lib.rs
    echo "// pub mod momento.auth;" >> lib.rs
popd
