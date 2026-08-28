#!/bin/bash
set -e
set -x

package_path=momento_wire_types
src_path=$package_path/generated

# Generate python code from proto files against the protobuf version this package
# declares. Older protobuf runtimes are no longer supported, so there is a single
# generated tree rather than one per protobuf generation.
poetry run python -m grpc_tools.protoc -I../proto --python_out=$src_path --pyi_out=$src_path --grpc_python_out=$src_path permissionmessages.proto extensions.proto cacheclient.proto controlclient.proto auth.proto cachepubsub.proto token.proto common.proto

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

# The same shortcoming affects imports in *_pb2.py files, e.g., `cachepubsub_pb2` importing `extensions_pb2`
# `import extensions_pb2 as extensions__pb2` -> `from . import extensions_pb2 as extensions__pb2`
for pb2_src in $src_path/*_pb2.py
do
    filename=$(basename "$pb2_src")

    # cacheclient_pb2.py -> cacheclient_pb2
    pb2_module_name=${filename%.py}

    # Replace gRPC absolute imports with relative imports so the package
    # will work when installed, eg:
    # `import cacheclient_pb2` -> `from . import cacheclient_pb2`
    sed -i.old "s/^\(import $pb2_module_name as \)/from . \1/g" $src_path/*_pb2.py
    sed -i.old "s/^\(import $pb2_module_name as \)/from . \1/g" $src_path/*_pb2_grpc.py
done

rm $src_path/*.old

# Write the init template
import_string=""
all_string=""
for python_src in $src_path/*pb2*.py
do
    filename=$(basename "$python_src")
    module_name=${filename%.py}
    import_string="${import_string}from .generated import $module_name as $module_name\n"
    # if all string is empty
    if [ -z "$all_string" ]; then
        all_string="\"$module_name\""
    else
        all_string="$all_string, \"$module_name\""
    fi
done

init_contents=$(cat <<EOF
# Re-exported so callers can write:
#       from momento_wire_types import cacheclient_pb2 as cache_pb
#       request = cache_pb.GetRequest()
${import_string}
__all__ = [$all_string]
EOF
)
printf "$init_contents" > $package_path/__init__.py
