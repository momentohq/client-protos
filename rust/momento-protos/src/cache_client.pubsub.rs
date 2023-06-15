#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// A value to publish through a topic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishRequest {
    /// Cache namespace for the topic to which you want to send the value.
    #[prost(string, tag = "1")]
    pub cache_name: ::prost::alloc::string::String,
    /// The literal topic name to which you want to send the value.
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    /// The value you want to send to the topic. All current subscribers will receive
    /// this, should the whims of the Internet prove merciful.
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<TopicValue>,
}
/// A description of how you want to subscribe to a topic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionRequest {
    /// Cache namespace for the topic to which you want to subscribe.
    #[prost(string, tag = "1")]
    pub cache_name: ::prost::alloc::string::String,
    /// The literal topic name to which you want to subscribe.
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    /// --> Providing this is not required. <--
    ///
    /// If provided, attempt to reconnect to the topic and replay messages starting from
    /// the provided sequence number. You may get a discontinuity if some (or all) of the
    /// messages are not available.
    /// If not provided (or 0), the subscription will begin with the latest messages.
    #[prost(uint64, tag = "3")]
    pub resume_at_topic_sequence_number: u64,
}
/// Possible message kinds from a topic. They can be items when they're from you, or
/// other kinds when we have something we think you might need to know about the
/// subscription's status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionItem {
    #[prost(oneof = "subscription_item::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<subscription_item::Kind>,
}
/// Nested message and enum types in `_SubscriptionItem`.
pub mod subscription_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// The subscription has yielded an item you previously published.
        #[prost(message, tag = "1")]
        Item(super::TopicItem),
        /// Momento wants to let you know we detected some possible inconsistency at this
        /// point in the subscription stream.
        ///
        /// A lack of a discontinuity does not mean the subscription is guaranteed to be
        /// strictly perfect, but the presence of a discontinuity is very likely to
        #[prost(message, tag = "2")]
        Discontinuity(super::Discontinuity),
        /// The stream is still working, there's nothing to see here.
        #[prost(message, tag = "3")]
        Heartbeat(super::Heartbeat),
    }
}
/// Your subscription has yielded an item you previously published. Here it is!
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicItem {
    /// Topic sequence numbers are **best-effort** and **informational**.
    /// They are not transactional.
    /// They exist:
    /// * to help reconnect to an existing topic while trying to avoid missing items.
    /// * to facilitate richer monitoring and logging.
    /// * to provide a best-effort awareness of stream contiguity, or lack thereof,
    ///    in case you need to know.
    /// You can safely ignore them if none of that matters to you!
    #[prost(uint64, tag = "1")]
    pub topic_sequence_number: u64,
    /// The value you previously published to this topic.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<TopicValue>,
}
/// A value in a topic - published, duplicated and received in a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicValue {
    /// Types of messages a topic may relay. You can mix types or you can make conventionally
    /// typed topics. Sticking with one kind will generally make your software easier to work
    /// with though, so we recommend picking the kind you like and using it for a topic!
    #[prost(oneof = "topic_value::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<topic_value::Kind>,
}
/// Nested message and enum types in `_TopicValue`.
pub mod topic_value {
    /// Types of messages a topic may relay. You can mix types or you can make conventionally
    /// typed topics. Sticking with one kind will generally make your software easier to work
    /// with though, so we recommend picking the kind you like and using it for a topic!
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(string, tag = "1")]
        Text(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Binary(::prost::alloc::vec::Vec<u8>),
    }
}
/// A message from Momento when we know a subscription to have skipped some messages.
/// We don't terminate your subscription in that case, but just in case you care, we
/// do our best to let you know about it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Discontinuity {
    /// The last topic value sequence number known to have been attempted (if known, 0 otherwise).
    #[prost(uint64, tag = "1")]
    pub last_topic_sequence: u64,
    /// The new topic sequence number after which TopicItems will ostensibly resume.
    #[prost(uint64, tag = "2")]
    pub new_topic_sequence: u64,
}
/// A message from Momento for when we want to reassure clients or frameworks that a
/// Subscription is still healthy.
/// These are synthetic meta-events and do not increase the topic sequence count.
/// Different subscribers may receive a different cadence of heartbeat, and no guarantee
/// is made about the cadence or even presence or absence of heartbeats in a stream.
/// They are a tool for helping ensure that socket timeouts and the like don't impact
/// subscriptions you may care about, but that aren't receiving a substantial publish rate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {}
/// Generated client implementations.
pub mod pubsub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// For working with topics in a cache.
    /// Momento topics are conceptually located on a cache. They are best-effort multicast.
    /// To use them, create a cache then start subscribing and publishing!
    ///
    /// Momento topic subscriptions try to give you information about the quality of the
    ///   stream you are receiving. For example, you might miss messages if your network
    ///   is slow, or if some intermediate switch fails, or due to rate limiting. It is
    ///   also possible, though we try to avoid it, that messages could briefly come out
    ///   of order between subscribers.
    ///   We try to tell you when things like this happen via a Discontinuity in your
    ///   subscription stream. If you do not care about occasional discontinuities then
    ///   don't bother handling them! You might still want to log them just in case ;-)
    #[derive(Debug, Clone)]
    pub struct PubsubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PubsubClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PubsubClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PubsubClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PubsubClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Publish a message to a topic.
        ///
        /// If a topic has no subscribers, then the effect of Publish MAY be either of:
        /// * It is dropped and the topic is nonexistent.
        /// * It is accepted to the topic as the next message.
        ///
        /// Publish() does not wait for subscribers to accept. It returns Ok upon accepting
        /// the topic value. It also returns Ok if there are no subscribers and the value
        /// happens to be dropped. Publish() can not guarantee delivery in theory but in
        /// practice it should almost always deliver to subscribers.
        ///
        /// REQUIRES HEADER authorization: Momento auth token
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRequest>,
        ) -> std::result::Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.pubsub.Pubsub/Publish",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.pubsub.Pubsub", "Publish"));
            self.inner.unary(req, path, codec).await
        }
        /// Subscribe to notifications from a topic.
        ///
        /// You will receive a stream of values and (hopefully occasional) discontinuities.
        /// Values will appear as copies of the payloads you Publish() to the topic.
        ///
        /// REQUIRES HEADER authorization: Momento auth token
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SubscriptionItem>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.pubsub.Pubsub/Subscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.pubsub.Pubsub", "Subscribe"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod pubsub_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PubsubServer.
    #[async_trait]
    pub trait Pubsub: Send + Sync + 'static {
        /// Publish a message to a topic.
        ///
        /// If a topic has no subscribers, then the effect of Publish MAY be either of:
        /// * It is dropped and the topic is nonexistent.
        /// * It is accepted to the topic as the next message.
        ///
        /// Publish() does not wait for subscribers to accept. It returns Ok upon accepting
        /// the topic value. It also returns Ok if there are no subscribers and the value
        /// happens to be dropped. Publish() can not guarantee delivery in theory but in
        /// practice it should almost always deliver to subscribers.
        ///
        /// REQUIRES HEADER authorization: Momento auth token
        async fn publish(
            &self,
            request: tonic::Request<super::PublishRequest>,
        ) -> std::result::Result<tonic::Response<super::Empty>, tonic::Status>;
        /// Server streaming response type for the Subscribe method.
        type SubscribeStream: futures_core::Stream<
                Item = std::result::Result<super::SubscriptionItem, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to notifications from a topic.
        ///
        /// You will receive a stream of values and (hopefully occasional) discontinuities.
        /// Values will appear as copies of the payloads you Publish() to the topic.
        ///
        /// REQUIRES HEADER authorization: Momento auth token
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscriptionRequest>,
        ) -> std::result::Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
    }
    /// For working with topics in a cache.
    /// Momento topics are conceptually located on a cache. They are best-effort multicast.
    /// To use them, create a cache then start subscribing and publishing!
    ///
    /// Momento topic subscriptions try to give you information about the quality of the
    ///   stream you are receiving. For example, you might miss messages if your network
    ///   is slow, or if some intermediate switch fails, or due to rate limiting. It is
    ///   also possible, though we try to avoid it, that messages could briefly come out
    ///   of order between subscribers.
    ///   We try to tell you when things like this happen via a Discontinuity in your
    ///   subscription stream. If you do not care about occasional discontinuities then
    ///   don't bother handling them! You might still want to log them just in case ;-)
    #[derive(Debug)]
    pub struct PubsubServer<T: Pubsub> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Pubsub> PubsubServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PubsubServer<T>
    where
        T: Pubsub,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cache_client.pubsub.Pubsub/Publish" => {
                    #[allow(non_camel_case_types)]
                    struct PublishSvc<T: Pubsub>(pub Arc<T>);
                    impl<T: Pubsub> tonic::server::UnaryService<super::PublishRequest>
                    for PublishSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PublishRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).publish(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PublishSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.pubsub.Pubsub/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: Pubsub>(pub Arc<T>);
                    impl<
                        T: Pubsub,
                    > tonic::server::ServerStreamingService<super::SubscriptionRequest>
                    for SubscribeSvc<T> {
                        type Response = super::SubscriptionItem;
                        type ResponseStream = T::SubscribeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).subscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Pubsub> Clone for PubsubServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Pubsub> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Pubsub> tonic::server::NamedService for PubsubServer<T> {
        const NAME: &'static str = "cache_client.pubsub.Pubsub";
    }
}
