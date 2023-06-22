#!/bin/bash
set -e
set -x

VERSION=3.18.1

if [[ "$OSTYPE" == "darwin"* ]]; then
  # Install protoc for local development on MacOS (requires Homebrew)
  brew install zip tree
  PROTOC_ZIP=protoc-$VERSION-osx-x86_64.zip
else # Linux
  sudo apt-get -y install zip tree
  PROTOC_ZIP=protoc-$VERSION-linux-x86_64.zip
fi

curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/$PROTOC_ZIP
sudo unzip -o $PROTOC_ZIP -d /usr/local bin/protoc
sudo chmod a+rx /usr/local/bin/protoc
sudo unzip -o $PROTOC_ZIP -d /usr/local 'include/*'
sudo chmod -R a+rx /usr/local/include/google
rm $PROTOC_ZIP

tree /usr/local/include/google
echo "Installed protoc $VERSION"
protoc --version