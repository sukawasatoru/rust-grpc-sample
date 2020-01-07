use hello_rpc::hello_rpc_service_server::{HelloRpcService, HelloRpcServiceServer};
use hello_rpc::{HelloRequest, HelloResponse};
use hello_stream::hello_stream_service_server::{HelloStreamService, HelloStreamServiceServer};
use hello_stream::{HelloStreamRequest, HelloStreamResponse};
use log::info;
use rust_grpc_sample::asset::Asset;
use rust_grpc_sample::prelude::*;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct HelloRpcServiceImpl;

#[tonic::async_trait]
impl HelloRpcService for HelloRpcServiceImpl {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        info!("on_hello: {:?}", request);
        Ok(Response::new(HelloResponse {
            reply_message: format!(
                "name: {}, message: {}",
                request.get_ref().name,
                request.get_ref().message
            ),
        }))
    }
}

#[derive(Debug, Default)]
struct HelloStreamServiceImpl;

#[tonic::async_trait]
impl HelloStreamService for HelloStreamServiceImpl {
    type HelloStreamStream = tokio::sync::mpsc::Receiver<Result<HelloStreamResponse, Status>>;

    async fn hello_stream(
        &self,
        request: Request<HelloStreamRequest>,
    ) -> Result<Response<Self::HelloStreamStream>, Status> {
        info!("on_hello_stream");
        let message = request.get_ref().message.to_owned();
        let create_result = move |num| format!("message: {}, count: {}", message, num);
        let (mut tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for i in 1..=5 {
                tokio::task::spawn_blocking(|| {
                    std::thread::sleep(std::time::Duration::from_secs(1))
                })
                .await
                .ok();
                info!(" => send {}", i);
                let ret = tx
                    .send(Ok(HelloStreamResponse {
                        reply_message: create_result(i),
                    }))
                    .await;
                if let Err(e) = ret {
                    info!("Client closed: {:?}", e);
                    break;
                }
            }
            info!(" /// done sending");
        });

        Ok(Response::new(rx))
    }
}

pub mod hello_rpc {
    tonic::include_proto!("rust_grpc_sample.hello_rpc");
}

pub mod hello_stream {
    tonic::include_proto!("rust_grpc_sample.hello_stream");
}

/// 1. add async keyword to main.
/// 1. include generated file the following code:
///    `pub mod your_any_name {tonic::include_proto!("package_name_defined_in_proto");}`
/// 1. import:
///    `your_any_name::<package_name_defined_in_proto>_server::{<ServiceName>Service, <ServiceName>Server};` and
///    `use your_any_name::YourMessageName;`
/// 1. implement struct:
///    ```
///    #[tonic::async_trait]
///    impl HelloRpcService for HelloRpcServiceImpl {
///        see target/debug/build/your_crate_name_with_hash/out/package_name_defined_in_proto.rs
///    }
///    ```
#[tokio::main]
async fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(Identity::from_pem(
            Asset::get("server.crt").ok_or_err()?,
            Asset::get("server.key").ok_or_err()?,
        )))
        .add_service(HelloRpcServiceServer::new(HelloRpcServiceImpl::default()))
        .add_service(HelloStreamServiceServer::new(
            HelloStreamServiceImpl::default(),
        ))
        .serve(format!("[::1]:{}", SERVER_PORT).parse()?)
        .await?;

    info!("Bye");
    Ok(())
}
