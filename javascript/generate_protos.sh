#!/bin/bash
set -e
set -x
TS_OUT=./src
rm -rf $TS_OUT
mkdir $TS_OUT
protoc --plugin=protoc-gen-ts=node_modules/protoc-gen-ts/bin/protoc-gen-ts.js -I=../proto -I=/usr/local/include --ts_out=$TS_OUT cacheclient.proto controlclient.proto cachepubsub.proto auth.proto cacheping.proto
