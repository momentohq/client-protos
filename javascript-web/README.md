# Usage
### package.json
```
dependencies: {
  "@gomomento/generated-types-webtext": "$current_version",
  "grpc-web": "1.3.1",
  "google-protobuf": "3.21.0"
}
```

### Basic usage shape
```typescript
const authToken = getClientAuthToken_never_harcode_server_api_keys();
const cacheName = 'your_cool_cache';
const momento = new cache.ScsClient('https://cache.cell-us-east-1-1.prod.a.momentohq.com', null, {})
function getMetadata(cacheName: string): grpcWeb.Metadata {
  return {
    'authorization': authToken,
    'cache': cacheName,
  };
}

const request = new _GetRequest();
request.setCacheKey(Buffer.from('some cache key'));

const getResult = await momento.get(request, getMetadata(props.cache));

if (getResult.getResult() === ECacheResult.HIT) {
  const body = Buffer.from(getResult.getCacheBody()).toString('utf-8')
  console.log('hit:', body);
} else {
  console.log('miss:', getResult.getMessage());
}

```

# Generating definitions
```bash
./generate_protos.sh [osx, linux, windows]
```
