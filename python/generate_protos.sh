#!/bin/bash
set -e
set -x

protoc -I=../proto --python_out=src/momento_wire_types cacheclient.proto controlclient.proto
