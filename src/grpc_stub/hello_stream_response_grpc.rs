// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait HelloStreamService {
    fn hello_stream(&self, o: ::grpc::RequestOptions, p: super::hello_stream_response::HelloStreamRequest) -> ::grpc::StreamingResponse<super::hello_stream_response::HelloStreamResponse>;
}

// client

pub struct HelloStreamServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_HelloStream: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::hello_stream_response::HelloStreamRequest, super::hello_stream_response::HelloStreamResponse>>,
}

impl ::grpc::ClientStub for HelloStreamServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HelloStreamServiceClient {
            grpc_client: grpc_client,
            method_HelloStream: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/rust_grpc_sample.HelloStreamService/HelloStream".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl HelloStreamService for HelloStreamServiceClient {
    fn hello_stream(&self, o: ::grpc::RequestOptions, p: super::hello_stream_response::HelloStreamRequest) -> ::grpc::StreamingResponse<super::hello_stream_response::HelloStreamResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_HelloStream.clone())
    }
}

// server

pub struct HelloStreamServiceServer;


impl HelloStreamServiceServer {
    pub fn new_service_def<H : HelloStreamService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/rust_grpc_sample.HelloStreamService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/rust_grpc_sample.HelloStreamService/HelloStream".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.hello_stream(o, p))
                    },
                ),
            ],
        )
    }
}
