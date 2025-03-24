#!/bin/bash
set -e
set -x

if [[ "$OSTYPE" == "darwin"* ]]; then
  platform='darwin-x86_64'
  sed_command="sed -i ''"
elif [[ "$OSTYPE" == "linux"* ]]; then
  platform='linux-x86_64'
  sed_command="sed -i"
elif [[ "$OSTYPE" == "cygwin"* || "$OSTYPE" == "msys"* ]]; then
  platform='windows-x86_64'
  sed_command="sed -i"
else
  echo "Unknown OS: $OSTYPE"
  exit 1
fi

version='1.3.1'
mkdir -p $HOME/bin
export PATH=$PATH:$HOME/bin
plugin="$HOME/bin/protoc-gen-grpc-web"
rm -f $plugin
curl -L https://github.com/grpc/grpc-web/releases/download/$version/protoc-gen-grpc-web-$version-$platform -o $plugin
chmod a+x $plugin

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
function generate_proto() {
  local proto_file_list=("$@")
  echo "Backing up protos dir"
  cp -r ../proto ../proto.bak

  # resolves the naming conflicts between permissionmessages and permissionrules
  $sed_command 's/SuperUserPermissions/SuperUserPermissionSet/g' ../proto/permissionrules.proto
  $sed_command 's/ExplicitPermissions/ExplicitPermissionSet/g' ../proto/permissionrules.proto
  $sed_command 's/SuperUser/SuperUserSet/g' ../proto/permissionrules.proto

  echo "Commenting out package declarations"
  for f in $proto_file_list
  do
    $sed_command 's/^\s*package \(.*\)/\/\/package \1/g' ../proto/${f}
    $sed_command 's/permission_rules.PermissionSet/PermissionSet/g' ../proto/"${f}"
    $sed_command 's/permission_messages.Permissions/Permissions/g' ../proto/${f}
    $sed_command 's/common._Unbounded/_Unbounded/g' ../proto/${f}
    $sed_command 's/common._Empty/_Empty/g' ../proto/${f}
    $sed_command 's/common.Present/Present/g' ../proto/${f}
    $sed_command 's/common.PresentAndNotEqual/PresentAndNotEqual/g' ../proto/${f}
    $sed_command 's/common.Absent/Absent/g' ../proto/${f}
    $sed_command 's/common.Equal/Equal/g' ../proto/${f}
    $sed_command 's/common.AbsentOrEqual/AbsentOrEqual/g' ../proto/${f}
    $sed_command 's/common.NotEqual/NotEqual/g' ../proto/${f}
    $sed_command 's/common.PresentAndNotHashEqual/PresentAndNotHashEqual/g' ../proto/${f}
    $sed_command 's/common.PresentAndHashEqual/PresentAndHashEqual/g' ../proto/${f}
    $sed_command 's/common.AbsentOrHashEqual/AbsentOrHashEqual/g' ../proto/${f}
    $sed_command 's/common.AbsentOrNotHashEqual/AbsentOrNotHashEqual/g' ../proto/${f}
    $sed_command 's/common.Unconditional/Unconditional/g' ../proto/${f}
  done

  protoc -I=../proto -I=/usr/local/include \
    --js_out=import_style=commonjs_strict:$out \
    ${proto_file_list}

  echo "Restoring backed up protos"
  rm -rf ../proto
  mv ../proto.bak ../proto

  protoc -I=../proto -I=/usr/local/include \
    --grpc-web_out=import_style=typescript,mode=grpcwebtext:$out \
    ${proto_file_list}
}

proto_file_list=" common.proto permissionrules.proto permissionmessages.proto extensions.proto cacheclient.proto controlclient.proto auth.proto cacheping.proto cachepubsub.proto token.proto webhook.proto leaderboard.proto global_admin.proto store.proto "
generate_proto "${proto_file_list[@]}"
