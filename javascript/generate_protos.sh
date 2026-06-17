#!/bin/bash
set -e
set -x

# At some point, google removed the javascript stuff from the main protos project and separated it out into
# its own project. This means you can't build the JS stuff with a modern version of protoc, so it can be
# tricky to build on mac. This issue showed what I believe is the "official" way to do it now... question mark?
#   https://github.com/protocolbuffers/protobuf/pull/9874#issuecomment-1481023982
#
npx grpc_tools_node_protoc -I=../proto -I=/usr/local/include --ts_out=src permissionmessages.proto cacheclient.proto controlclient.proto cachepubsub.proto auth.proto cacheping.proto token.proto leaderboard.proto webhook.proto function_types.proto function.proto

# `function.proto` declares `package function`, but `function` is a reserved word in JS/TS, so protoc-gen-ts
# emits an invalid `export namespace function {`. Rename ONLY the generated TypeScript namespace to
# `function_client`; the gRPC wire package is unchanged — the method paths ("/function.FunctionRegistry/...")
# keep the real `function` package, so routing is unaffected.
perl -0pi -e 's/^export namespace function \{/export namespace function_client {/m' src/function.ts
# Fail loudly if a future protoc-gen-ts changes its output format and the rename silently no-ops (otherwise the
# only symptom is an opaque tsc "reserved word" error).
grep -q '^export namespace function_client {' src/function.ts || { echo 'ERROR: function namespace rename did not apply'; exit 1; }
