# Generating definitions
```bash
npm install
mkdir dist
PATH=node_modules/protoc-gen-ts/bin/:$PATH protoc -I=../proto --ts_out=dist cacheclient.proto controlclient.proto cachepubsub.proto vectorindex.proto
```
