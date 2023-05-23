#!/bin/bash
set -e
set -x

src_path=src/momento_wire_types

# Generate python proto artifacts
poetry run python -m grpc_tools.protoc -I../proto --python_out=$src_path --pyi_out=$src_path --grpc_python_out=$src_path cacheclient.proto controlclient.proto auth.proto

# A shortcoming of the generated code is in the grpc generated code,
# the protobuf imports are absolute instead of relative.
# Left as is, the imports will fail when we install the package. See:
# https://github.com/protocolbuffers/protobuf/issues/1491
for grpc_src in $src_path/*_grpc.py
do
    filename=$(basename "$grpc_src")

    # cacheclient_pb2_grpc.py -> cacheclient_pb2
    grpc_module_name=${filename%_grpc.py}

    # Replace gRPC absolute imports with relative imports so the package
    # will work when installed, eg:
    # `import cacheclient_pb2 as cacheclient__pb2` -> `from . import cacheclient_pb2 as cacheclient__pb2`
    sed -i.old "s/^\(import $grpc_module_name as \)/from . \1/g" $grpc_src
done

rm $src_path/*.old
