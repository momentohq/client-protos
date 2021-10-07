#!/bin/bash
set -e
set -x

VERSION=3.18.1

sudo apt-get -y install zip
curl -L https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-linux-x86_64.zip -o protoc.zip
unzip -o protoc.zip -d protoc
sudo mv protoc/include/* /usr/local/include/
