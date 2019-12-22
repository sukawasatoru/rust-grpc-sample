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

pub trait HelloRpcService {
    fn hello(&self, o: ::grpc::RequestOptions, p: super::hello_rpc::HelloRequest) -> ::grpc::SingleResponse<super::hello_rpc::HelloResponse>;
}

// client

pub struct HelloRpcServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Hello: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::hello_rpc::HelloRequest, super::hello_rpc::HelloResponse>>,
}

impl ::grpc::ClientStub for HelloRpcServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HelloRpcServiceClient {
            grpc_client: grpc_client,
            method_Hello: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/rust_grpc_sample.HelloRpcService/Hello".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl HelloRpcService for HelloRpcServiceClient {
    fn hello(&self, o: ::grpc::RequestOptions, p: super::hello_rpc::HelloRequest) -> ::grpc::SingleResponse<super::hello_rpc::HelloResponse> {
        self.grpc_client.call_unary(o, p, self.method_Hello.clone())
    }
}

// server

pub struct HelloRpcServiceServer;


impl HelloRpcServiceServer {
    pub fn new_service_def<H : HelloRpcService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/rust_grpc_sample.HelloRpcService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/rust_grpc_sample.HelloRpcService/Hello".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.hello(o, p))
                    },
                ),
            ],
        )
    }
}
