<img src="https://docs.momentohq.com/img/logo.svg" alt="logo" width="400"/>

# Momento Client Protocols

This repo contains the protobuf protos that define the Momento wire format.  We provide client libraries for most programming languages; in most cases you will want to start with one of those.  See the [Getting Started](https://docs.momentohq.com/getting-started#next-steps) guide for links to the client library in your favorite language!

This repo is for developers who are working on Momento client libraries, or for special use cases where you want to write code that interacts directly with the Momento gRPC service.

In addition to the protos, this repo contains automation to build the language-specific client bindings for most languages.  These types are published to the respective language package repositories, independently from the Momento client libraries themselves.  You can find them here:

* Node.js: [npm.js: `@gomomento/generated-types`](https://www.npmjs.com/package/@gomomento/generated-types)
* Python: [PyPI: `momento-wire-types`](https://pypi.org/project/momento-wire-types/)
* .NET: [NuGet: `Momento.Protos`](https://www.nuget.org/packages/Momento.Protos)
* Rust: [crates.io: `momento-protos`](https://crates.io/crates/momento-protos)
* Java: [Maven Central: `client-protos`](https://central.sonatype.com/artifact/software.momento.java/client-protos/)

If you're interested in looking at the protos themselves, here are the most important ones:

* [cacheclient](proto/cacheclient.proto): APIs for interacting reading and writing cache values (data plane)
* [controlclient](proto/controlclient.proto): APIs for managing caches (create, list, delete: control plane)
