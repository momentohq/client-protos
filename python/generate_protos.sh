#!/bin/bash
set -e
set -x

protoc -I=../proto --python_out=momento/wire_types cacheclient.proto controlclient.proto
