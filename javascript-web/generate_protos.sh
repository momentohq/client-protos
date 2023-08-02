#!/bin/bash
set -e
set -x

if [[ "$OSTYPE" == "darwin"* ]]; then
  platform='darwin-x86_64'
elif [[ "$OSTYPE" == "linux"* ]]; then
  platform='linux-x86_64'
elif [[ "$OSTYPE" == "cygwin"* || "$OSTYPE" == "msys"* ]]; then
  platform='windows-x86_64'
else
  echo "Unknown OS: $OSTYPE"
  exit 1
fi

version='1.3.1'
plugin='/usr/local/bin/protoc-gen-grpc-web'
sudo rm -f $plugin
sudo curl -L https://github.com/grpc/grpc-web/releases/download/$version/protoc-gen-grpc-web-$version-$platform -o $plugin
sudo chmod a+x $plugin

out=./src
rm -rf $out
mkdir $out

# Is your protoc a broken release?
# brew install protobuf@3
# brew link --overwrite protobuf@3
# https://github.com/protocolbuffers/protobuf-javascript/issues/127#issuecomment-1204202844

# The `--js_out` plugin will generate JavaScript code (`echo_pb.js`), and the
# `-grpc-web_out` plugin will generate a TypeScript definition file for it
# (`echo_pb.d.ts`). This is a temporary hack until the `--js_out` supports
# TypeScript itself. See https://github.com/grpc/grpc-web/blob/7c528784576abbbfd05eb6085abb8c319d76ab05/README.md?plain=1#L246

# Ramya: We need strict commonjs to allow Cloudflare workers to run the web sdk.
# After changing to commonjs_strict, the web SDK will build but unit tests and integ tests fail with 
# ```
# Test suite failed to run
#    TypeError: Cannot read properties of undefined (reading 'Never')
#
#       7 | } from '@gomomento/generated-types-webtext/dist/auth_pb';
#       8 | import {cacheServiceErrorMapper} from '../errors/cache-service-error-mapper';
#    >  9 | import Never = _GenerateApiTokenRequest.Never;
#         |                                         ^
#      10 | import Expires = _GenerateApiTokenRequest.Expires;
#      11 | import {
#      12 |   CredentialProvider,

#      at Object.<anonymous> (src/internal/auth-client.ts:9:41)
#      at Object.<anonymous> (src/auth-client.ts:5:1)
#      at Object.<anonymous> (src/index.ts:2:1)
#      at Object.<anonymous> (test/integration/integration-setup.ts:13:1)
#      at Object.<anonymous> (test/integration/shared/auth-client.test.ts:2:1)
# ```
# I believe this is related to https://github.com/protocolbuffers/protobuf-javascript/issues/40
# From a hint in https://medium.com/expedia-group-tech/the-weird-world-of-grpc-tooling-for-node-js-part-2-daafed94cc32,
# I found that removing the `package <pkg-name>` from the protos gives us the JS exports we want.
# Removing the package permanently is not possible now because it is part of the GRPC method descriptor.
# So we do a terrible hack to comment out the package declaration before generating the JS types,
# but add them back before generating the GRPC web bindings

proto_file_list=" extensions.proto cacheclient.proto controlclient.proto auth.proto cacheping.proto cachepubsub.proto "

echo "Commenting out package declarations"
for f in $proto_file_list
do
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/^\s*package \(.*\)/\/\/package \1/g' ../proto/${f}
  else
    sed -i 's/^\s*package \(.*\)/\/\/package \1/g' ../proto/${f}
  fi
done

protoc -I=../proto -I=/usr/local/include \
  --js_out=import_style=commonjs_strict:$out \
  ${proto_file_list}

echo "Uncommenting package declarations"
for f in $proto_file_list
do
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/^\/\/package \(.*\)/package \1/g' ../proto/${f}
  else
    sed -i 's/^\/\/package \(.*\)/package \1/g' ../proto/${f}
  fi
done

protoc -I=../proto -I=/usr/local/include \
  --grpc-web_out=import_style=typescript,mode=grpcwebtext:$out \
  ${proto_file_list}
