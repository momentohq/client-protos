#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResponse {
    #[prost(oneof = "login_response::State", tags = "1, 2, 3, 4")]
    pub state: ::core::option::Option<login_response::State>,
}
/// Nested message and enum types in `_LoginResponse`.
pub mod login_response {
    /// Terminal state, login success
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoggedIn {
        #[prost(string, tag = "1")]
        pub session_token: ::prost::alloc::string::String,
        /// How many seconds the token was valid for when issued.
        /// Will vary slightly from reality upon receipt, as
        /// time has passed since the token was minted.
        /// You might expect to see this true to within 10
        /// seconds of client-side timekeeping but as is
        /// ever the case, there are no guarantees with
        /// public network timing.
        #[prost(uint32, tag = "2")]
        pub valid_for_seconds: u32,
    }
    /// Terminal state, login error
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        #[prost(string, tag = "1")]
        pub description: ::prost::alloc::string::String,
    }
    /// Open a browser to a url, for interactive login
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DirectBrowser {
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
    }
    /// Logging info about the login process
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum State {
        #[prost(message, tag = "1")]
        DirectBrowser(DirectBrowser),
        #[prost(message, tag = "2")]
        LoggedIn(LoggedIn),
        #[prost(message, tag = "3")]
        Message(Message),
        #[prost(message, tag = "4")]
        Error(Error),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateApiTokenRequest {
    #[prost(string, tag = "3")]
    pub auth_token: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub permissions: ::core::option::Option<generate_api_token_request::Permissions>,
    #[prost(oneof = "generate_api_token_request::Expiry", tags = "1, 2")]
    pub expiry: ::core::option::Option<generate_api_token_request::Expiry>,
}
/// Nested message and enum types in `_GenerateApiTokenRequest`.
pub mod generate_api_token_request {
    /// generate a token that will never expire
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Never {}
    /// generate a token that has an expiry
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expires {
        /// how many seconds do you want the api token to be valid for?
        #[prost(uint32, tag = "1")]
        pub valid_for_seconds: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Permissions {
        #[prost(oneof = "permissions::Kind", tags = "1, 2")]
        pub kind: ::core::option::Option<permissions::Kind>,
    }
    /// Nested message and enum types in `Permissions`.
    pub mod permissions {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(enumeration = "super::SuperUserPermissions", tag = "1")]
            SuperUser(i32),
            #[prost(message, tag = "2")]
            Explicit(super::ExplicitPermissions),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExplicitPermissions {
        #[prost(message, repeated, tag = "1")]
        pub permissions: ::prost::alloc::vec::Vec<PermissionsType>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PermissionsType {
        #[prost(oneof = "permissions_type::Kind", tags = "1, 2")]
        pub kind: ::core::option::Option<permissions_type::Kind>,
    }
    /// Nested message and enum types in `PermissionsType`.
    pub mod permissions_type {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CachePermissions {
            #[prost(enumeration = "super::CacheRole", tag = "1")]
            pub role: i32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TopicPermissions {
            #[prost(enumeration = "super::TopicRole", tag = "1")]
            pub role: i32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "1")]
            CachePermissions(CachePermissions),
            #[prost(message, tag = "2")]
            TopicPermissions(TopicPermissions),
        }
    }
    /// Aliases for categories of functionality.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CacheRole {
        CachePermitNone = 0,
        /// Restricts access to apis that read and write data from caches: No higher level resource description or modification.
        CacheReadWrite = 1,
    }
    impl CacheRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CacheRole::CachePermitNone => "CachePermitNone",
                CacheRole::CacheReadWrite => "CacheReadWrite",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CachePermitNone" => Some(Self::CachePermitNone),
                "CacheReadWrite" => Some(Self::CacheReadWrite),
                _ => None,
            }
        }
    }
    /// Aliases for categories of functionality.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TopicRole {
        TopicPermitNone = 0,
        /// Restricts access to apis that read and write data from topics: No higher level resource description or modification.
        TopicReadWrite = 1,
    }
    impl TopicRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TopicRole::TopicPermitNone => "TopicPermitNone",
                TopicRole::TopicReadWrite => "TopicReadWrite",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TopicPermitNone" => Some(Self::TopicPermitNone),
                "TopicReadWrite" => Some(Self::TopicReadWrite),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SuperUserPermissions {
        SuperUser = 0,
    }
    impl SuperUserPermissions {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SuperUserPermissions::SuperUser => "SuperUser",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SuperUser" => Some(Self::SuperUser),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiry {
        #[prost(message, tag = "1")]
        Never(Never),
        #[prost(message, tag = "2")]
        Expires(Expires),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateApiTokenResponse {
    /// the api key used for authentication against Momento backend
    #[prost(string, tag = "1")]
    pub api_key: ::prost::alloc::string::String,
    /// the token that will allow the api token to be refreshed, which will
    /// give you back a new refresh and api token
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
    /// the Momento endpoint that this token is allowed to make requests against
    #[prost(string, tag = "3")]
    pub endpoint: ::prost::alloc::string::String,
    /// epoch seconds when the api token expires
    #[prost(uint64, tag = "4")]
    pub valid_until: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshApiTokenRequest {
    /// the existing api token to be refreshed
    #[prost(string, tag = "1")]
    pub api_key: ::prost::alloc::string::String,
    /// the refresh token that was generated with that api token
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshApiTokenResponse {
    /// the new api key used for authentication against Momento backend
    #[prost(string, tag = "1")]
    pub api_key: ::prost::alloc::string::String,
    /// the token that will allow the api token to be refreshed, which will
    /// give you back a new refresh and api token
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
    /// the Momento endpoint that this token is allowed to make requests against
    #[prost(string, tag = "3")]
    pub endpoint: ::prost::alloc::string::String,
    /// epoch seconds when the api token expires
    #[prost(uint64, tag = "4")]
    pub valid_until: u64,
}
/// Generated client implementations.
pub mod auth_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AuthClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthClient<tonic::transport::Channel> {
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
    impl<T> AuthClient<T>
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
        ) -> AuthClient<InterceptedService<T, F>>
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
            AuthClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::LoginResponse>>,
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
            let path = http::uri::PathAndQuery::from_static("/auth.Auth/Login");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("auth.Auth", "Login"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// api for initially generating api and refresh tokens
        pub async fn generate_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateApiTokenResponse>,
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
                "/auth.Auth/GenerateApiToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("auth.Auth", "GenerateApiToken"));
            self.inner.unary(req, path, codec).await
        }
        /// api for programmatically refreshing api and refresh tokens
        pub async fn refresh_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::RefreshApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RefreshApiTokenResponse>,
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
                "/auth.Auth/RefreshApiToken",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("auth.Auth", "RefreshApiToken"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod auth_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthServer.
    #[async_trait]
    pub trait Auth: Send + Sync + 'static {
        /// Server streaming response type for the Login method.
        type LoginStream: futures_core::Stream<
                Item = std::result::Result<super::LoginResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> std::result::Result<tonic::Response<Self::LoginStream>, tonic::Status>;
        /// api for initially generating api and refresh tokens
        async fn generate_api_token(
            &self,
            request: tonic::Request<super::GenerateApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateApiTokenResponse>,
            tonic::Status,
        >;
        /// api for programmatically refreshing api and refresh tokens
        async fn refresh_api_token(
            &self,
            request: tonic::Request<super::RefreshApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RefreshApiTokenResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AuthServer<T: Auth> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Auth> AuthServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthServer<T>
    where
        T: Auth,
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
                "/auth.Auth/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: Auth>(pub Arc<T>);
                    impl<
                        T: Auth,
                    > tonic::server::ServerStreamingService<super::LoginRequest>
                    for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type ResponseStream = T::LoginStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).login(request).await };
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
                        let method = LoginSvc(inner);
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
                "/auth.Auth/GenerateApiToken" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateApiTokenSvc<T: Auth>(pub Arc<T>);
                    impl<
                        T: Auth,
                    > tonic::server::UnaryService<super::GenerateApiTokenRequest>
                    for GenerateApiTokenSvc<T> {
                        type Response = super::GenerateApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).generate_api_token(request).await
                            };
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
                        let method = GenerateApiTokenSvc(inner);
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
                "/auth.Auth/RefreshApiToken" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshApiTokenSvc<T: Auth>(pub Arc<T>);
                    impl<
                        T: Auth,
                    > tonic::server::UnaryService<super::RefreshApiTokenRequest>
                    for RefreshApiTokenSvc<T> {
                        type Response = super::RefreshApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RefreshApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).refresh_api_token(request).await
                            };
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
                        let method = RefreshApiTokenSvc(inner);
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
    impl<T: Auth> Clone for AuthServer<T> {
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
    impl<T: Auth> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Auth> tonic::server::NamedService for AuthServer<T> {
        const NAME: &'static str = "auth.Auth";
    }
}
