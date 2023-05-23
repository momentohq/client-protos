.PHONY: all
## Generate sync unit tests, format, lint, and test
all: build

.PHONY: install-poetry
## Bootstrap installing poetry
install-poetry:
	curl -sSL https://install.python-poetry.org | python3 -

.PHONY: install
## Install project and dependencies
install:
	@poetry install

.PHONY: clean
## Remove intermediate files
clean:
	@rm -rf dist .mypy_cache .pytest_cache momento_wire_types/*pb2*
	@find . -name "*__pycache__*" | xargs rm -rf

.PHONY: protoc
## Run protobuf compiler and post-processing
protoc: clean
	@./run-protoc.sh

.PHONY: build
## Build project
build: protoc
	@poetry build

# See <https://gist.github.com/klmr/575726c7e05d8780505a> for explanation.
.PHONY: help
help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=19 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'|more $(shell test $(shell uname) == Darwin && echo '-Xr')
