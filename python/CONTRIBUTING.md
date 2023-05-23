# Setup

This project has a Makefile to help with the setup and build process.

## Prerequisites

Install the poetry package manager by running:

```bash
make install-poetry
```

## Install

Install the project runtime and dev dependencies by running:

```bash
make install
```

# Build

Build the project by running:

```bash
make build
```

This runs the protobuf compiler, postprocesses the generated code (see `run-protoc.sh`), and builds the distribution package. See the `momento_wire_types` directory for the generated code and type stub files (see [PEP-484](https://peps.python.org/pep-0484/#stub-files)).
