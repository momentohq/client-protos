#!/bin/bash
set -e
set -x

if [ $# -lt 1 ]; then
  echo usage: $0 osx  [osx, linux, windows]
  exit 1
fi
if [ $1 == 'osx' ]; then
  platform='darwin-x86_64'
elif [ $1 == 'linux' ]; then
  platform='linux-x86_64'
elif [ $1 == 'windows' ]; then
  platform='windows-x86_64'
else
  echo usage: $0 osx  [osx, linux, windows]
  exit 1
fi

version='1.3.1'
plugin='/usr/local/bin/protoc-gen-grpc-web'
rm -f $plugin
curl -L https://github.com/grpc/grpc-web/releases/download/$version/protoc-gen-grpc-web-$version-$platform -o $plugin
chmod +x $plugin

out=./src
rm -rf $out
mkdir $out

# Is your protoc a broken release?
# brew install protobuf@3
# brew link --overwrite protobuf@3
# https://github.com/protocolbuffers/protobuf-javascript/issues/127#issuecomment-1204202844

protoc -I=../proto -I=/usr/local/include \
  --js_out=import_style=commonjs:$out \
  cacheclient.proto controlclient.proto auth.proto cacheping.proto

protoc -I=../proto -I=/usr/local/include \
  --grpc-web_out=import_style=typescript,mode=grpcwebtext:$out \
  cacheclient.proto controlclient.proto auth.proto cacheping.proto
