#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloStreamRequest {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloStreamResponse {
    #[prost(string, tag = "1")]
    pub reply_message: std::string::String,
}
#[doc = r" Generated server implementations."]
pub mod hello_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct HelloStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HelloStreamServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> HelloStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub async fn hello_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HelloStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rust_grpc_sample.hello_stream.HelloStreamService/HelloStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for HelloStreamServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod hello_stream_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with HelloStreamServiceServer."]
    #[async_trait]
    pub trait HelloStreamService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the HelloStream method."]
        type HelloStreamStream: Stream<Item = Result<super::HelloStreamResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn hello_stream(
            &self,
            request: tonic::Request<super::HelloStreamRequest>,
        ) -> Result<tonic::Response<Self::HelloStreamStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct HelloStreamServiceServer<T: HelloStreamService> {
        inner: Arc<T>,
    }
    impl<T: HelloStreamService> HelloStreamServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self { inner }
        }
    }
    impl<T: HelloStreamService> Service<http::Request<HyperBody>> for HelloStreamServiceServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rust_grpc_sample.hello_stream.HelloStreamService/HelloStream" => {
                    struct HelloStreamSvc<T: HelloStreamService>(pub Arc<T>);
                    impl<T: HelloStreamService>
                        tonic::server::ServerStreamingService<super::HelloStreamRequest>
                        for HelloStreamSvc<T>
                    {
                        type Response = super::HelloStreamResponse;
                        type ResponseStream = T::HelloStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloStreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.hello_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = HelloStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: HelloStreamService> Clone for HelloStreamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: HelloStreamService> tonic::transport::ServiceName for HelloStreamServiceServer<T> {
        const NAME: &'static str = "rust_grpc_sample.hello_stream.HelloStreamService";
    }
}
