# Setup

This project has a Makefile to help with the setup and build process.

## Prerequisites

Install the poetry package manager by running:

```bash
make install-poetry
```

## Install

Install the project dependencies by running:

```bash
make install
```

This installs the python runtime dependencies and dev (build) dependencies.

# Build

Build the project by running:

```bash
make build
```

This runs the protobuf compiler, postprocesses the generated code (see `run-protoc.sh`), and builds the distribution package.
