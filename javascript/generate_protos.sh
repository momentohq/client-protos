#!/bin/bash
set -e
set -x

# At some point, google removed the javascript stuff from the main protos project and separated it out into
# its own project. This means you can't build the JS stuff with a modern version of protoc, so it can be
# tricky to build on mac. This issue showed what I believe is the "official" way to do it now... question mark?
#   https://github.com/protocolbuffers/protobuf/pull/9874#issuecomment-1481023982
#
npx grpc_tools_node_protoc -I=../proto -I=/usr/local/include --ts_out=src permissionmessages.proto cacheclient.proto controlclient.proto cachepubsub.proto auth.proto cacheping.proto vectorindex.proto token.proto leaderboard.proto
