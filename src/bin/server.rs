use chrono::{Duration, Utc};
use futures::{Poll, Stream};
use grpc::SingleResponse;
use grpc::{RequestOptions, StreamingResponse};
use log::debug;
use log::info;
use tls_api::TlsAcceptorBuilder;

use rust_grpc_sample::asset::Asset;
use rust_grpc_sample::grpc_stub::hello_rpc::{HelloRequest, HelloResponse};
use rust_grpc_sample::grpc_stub::hello_rpc_grpc::{HelloRpcService, HelloRpcServiceServer};
use rust_grpc_sample::grpc_stub::hello_stream_response::{HelloStreamRequest, HelloStreamResponse};
use rust_grpc_sample::grpc_stub::hello_stream_response_grpc::{
    HelloStreamService, HelloStreamServiceServer,
};
use rust_grpc_sample::prelude::*;

struct HelloServiceImpl;

impl HelloRpcService for HelloServiceImpl {
    fn hello(&self, _o: RequestOptions, p: HelloRequest) -> SingleResponse<HelloResponse> {
        info!("on_hello: {:?}", p);

        SingleResponse::completed(HelloResponse {
            reply_message: format!("name: {}, message: {}", p.name, p.message),
            ..Default::default()
        })
    }
}

struct HelloStreamServiceImpl;

impl HelloStreamService for HelloStreamServiceImpl {
    fn hello_stream(
        &self,
        _o: RequestOptions,
        p: HelloStreamRequest,
    ) -> StreamingResponse<HelloStreamResponse> {
        info!("on_hello_stream");
        StreamingResponse::no_metadata(HelloStreamStream::new(3, p))
    }
}

struct HelloStreamStream {
    current: u32,
    max: u32,
    running_child: bool,
    request: HelloStreamRequest,
}

impl HelloStreamStream {
    fn new(max: u32, request: HelloStreamRequest) -> Self {
        Self {
            current: 0,
            max,
            running_child: false,
            request,
        }
    }
}

impl Stream for HelloStreamStream {
    type Item = HelloStreamResponse;
    type Error = grpc::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        debug!("poll message: {}", self.request.message);

        match self.current {
            ref mut x if *x < self.max => {
                if self.running_child {
                    self.running_child = false;
                    *x = *x + 1;

                    let reply_message =
                        format!("message: {}, count: {}", self.request.message, self.current);
                    debug!("server reply: {}", reply_message);
                    Ok(futures::Async::Ready(Some(HelloStreamResponse {
                        reply_message,
                        ..Default::default()
                    })))
                } else {
                    debug!("execute timer message: {}", self.request.message);

                    self.running_child = true;
                    let task = futures::task::current();
                    let lend = Utc::now() + Duration::seconds(5);
                    std::thread::spawn(move || {
                        while Utc::now() < lend {
                            let delta_sec = lend.timestamp() - Utc::now().timestamp();
                            if delta_sec > 0 {
                                std::thread::sleep(std::time::Duration::from_secs(
                                    delta_sec as u64,
                                ));
                            }
                            task.notify();
                        }
                    });
                    Ok(futures::Async::NotReady)
                }
            }
            _ => Ok(futures::Async::Ready(None)),
        }
    }
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    let mut builder = grpc::ServerBuilder::<tls_api_native_tls::TlsAcceptor>::new();

    let key = Asset::get("server.p12").ok_or_err()?;
    let tls_acceptor =
        tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(key.as_ref(), "grpc-sample")?
            .build()?;

    builder.http.set_port(SERVER_PORT);
    builder.http.set_cpu_pool_threads(10);
    builder.http.set_tls(tls_acceptor);
    builder.add_service(HelloRpcServiceServer::new_service_def(HelloServiceImpl));
    builder.add_service(HelloStreamServiceServer::new_service_def(
        HelloStreamServiceImpl,
    ));
    let _sv = builder.build()?;

    loop {
        std::thread::park();
    }
}
