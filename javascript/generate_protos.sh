#!/bin/bash
set -e
set -x

PATH=node_modules/protoc-gen-ts/bin/:$PATH protoc -I=../proto -I=/usr/local/include --ts_out=src permissions.proto cacheclient.proto controlclient.proto cachepubsub.proto auth.proto cacheping.proto vectorindex.proto token.proto
