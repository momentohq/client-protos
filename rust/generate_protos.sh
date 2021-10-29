#!/bin/bash
set -e
set -x

protoc -I=../proto --rust_out=./src cacheclient.proto controlclient.proto
