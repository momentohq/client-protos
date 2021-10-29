#!/bin/bash
set -e
set -x

CURRENT_DIR=${PWD##*/}
if [ "$CURRENT_DIR" != "rust" ]; then
    echo "run this script rust directory, current directory is ${CURRENT_DIR}"
    exit 1
fi

protoc -I=../proto --rust_out=./src cacheclient.proto controlclient.proto
