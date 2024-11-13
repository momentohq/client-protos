#!/bin/bash
set -e
set -x

package_path=momento_wire_types

# Array of versions
python_protobuf_versions=("protobuf<3.20" "protobuf>4")
generated_code_dirs=("$package_path/v319" "$package_path/v4")

# iterate over indexes of array python_protobuf_versions
for index in ${!python_protobuf_versions[*]}
do
    python_protobuf_version=${python_protobuf_versions[$index]}
    src_path=${generated_code_dirs[$index]}

    poetry add $python_protobuf_version

    # Generate python code from proto files
    # The versions less than 3.20 do not support the --pyi_out flag
    # Regardless we generate once and put in the main package location.
    pyi_out=""
    if [[ $python_protobuf_version == "protobuf>4" ]]; then
        pyi_out="--pyi_out=$src_path"
    fi
    poetry run python -m grpc_tools.protoc -I../proto --python_out=$src_path $pyi_out --grpc_python_out=$src_path permissionmessages.proto extensions.proto cacheclient.proto controlclient.proto auth.proto cachepubsub.proto token.proto common.proto

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
    done

    rm $src_path/*.old
done

# The older versions of protobuf do not support the --pyi_out flag.
# We can reuse the type interfaces from the newer versions without issue.
cp $package_path/v4/*pyi $package_path/v319

# Restore the version before running the script
poetry add "protobuf>=3,<5"

# Write the init template
v4_import_string=""
v319_import_string=""
all_string=""
for python_src in $package_path/v4/*pb2*.py
do
    filename=$(basename "$python_src")
    module_name=${filename%.py}
    v4_import_string="${v4_import_string}    from .v4 import $module_name as $module_name\n"
    v319_import_string="${v319_import_string}    from .v319 import $module_name as $module_name\n"
    # if all string is empty
    if [ -z "$all_string" ]; then
        all_string="\"$module_name\""
    else
        all_string="$all_string, \"$module_name\""
    fi
done

init_contents=$(cat <<EOF
import google.protobuf as protobuf

try:
    protobuf_version = protobuf.__version__.split(".")
    major = int(protobuf_version[0])
    minor = int(protobuf_version[1])
except ValueError:
    raise ValueError("Could not parse protobuf version: {}".format(protobuf.__version__))

# Import the correct version of the generated code.
# To accomodate the dynamic imports, we must now write code like:
#       from momento_wire_types import cacheclient_pb2 as cache_pb
#       request = cache_pb.GetRequest()
# instead of:
#       from momento_wire_types.cacheclient_pb2 import GetRequest
#       request = GetRequest()
# This because the multi-level from will not work with the dynamic imports.
if major >= 4 or (major == 3 and minor >= 20):
${v4_import_string}
else:
${v319_import_string}

__all__ = [$all_string]
EOF
)
printf "$init_contents" > $package_path/__init__.py
