# Releasing
```bash
npm adduser --scope @momento 

npm install
mkdir dist
PATH=node_modules/protoc-gen-ts/bin/:$PATH protoc -I=../proto --ts_out=dist cacheclient.proto controlclient.proto
```
