# Setup

This project has a Makefile to help with the setup and build process.

## Prerequisites

### Python

This depends on Python. We recommend managing python versions with [pyenv](https://github.com/pyenv/pyenv). We have tested on Ubuntu and MacOS with Python 3.10.

### Poetry

Install the poetry package manager by running:

```bash
make install-poetry
```

## Install

Install the project runtime and dev dependencies by running. This shows how to install Python 3.10.9 with pyenv, and then install the project dependencies with poetry.
If you already have Python 3.10 installed, you can skip the first two commands.

```bash
pyenv install 3.10.9
pyenv local 3.10.9
poetry env use 3.10
make install
```

# Build

Build the project by running:

```bash
make build
```

This runs the protobuf compiler, postprocesses the generated code (see `run-protoc.sh`), and builds the distribution package. See the `momento_wire_types` directory for the generated code and type stub files (see [PEP-484](https://peps.python.org/pep-0484/#stub-files)).
