use log::info;
use rust_grpc_sample::asset::Asset;
use rust_grpc_sample::grpc_stub::rust_grpc_sample::api::hello_rpc_service_client::HelloRpcServiceClient;
use rust_grpc_sample::grpc_stub::rust_grpc_sample::api::hello_stream_service_client::HelloStreamServiceClient;
use rust_grpc_sample::grpc_stub::rust_grpc_sample::api::{
    HelloRequest, HelloStreamRequest, HelloStreamResponse,
};
use rust_grpc_sample::prelude::*;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::Request;

async fn request_hello_rpc(channel: Channel) -> Fallible<()> {
    let mut client = HelloRpcServiceClient::new(channel);

    let response = client
        .hello(Request::new(HelloRequest {
            name: "Hello name".into(),
            message: "Hello message".into(),
        }))
        .await?;

    println!("hello_rpc response: {:?}", response);
    Ok(())
}

async fn request_hello_stream(channel: Channel) -> Fallible<()> {
    let mut stream_client = HelloStreamServiceClient::new(channel);
    let mut stream_response: tonic::Streaming<HelloStreamResponse> = stream_client
        .hello_stream(Request::new(HelloStreamRequest {
            message: "Hello stream message".into(),
        }))
        .await?
        .into_inner();
    println!("send stream request");

    while let Some(entry) = stream_response.message().await? {
        let entry: HelloStreamResponse = entry;
        println!("hello_stream response it: {:?}", entry);
    }

    Ok(())
}

async fn create_https_channel() -> Fallible<Channel> {
    Ok(
        Channel::builder(format!("https://[::1]:{}", SERVER_PORT).parse()?)
            .tls_config(
                ClientTlsConfig::new()
                    .ca_certificate(Certificate::from_pem(Asset::get("root.crt").ok_or_err()?))
                    .domain_name("localhost"),
            )
            .connect()
            .await?,
    )
}

/// 1. add async keyword and `#[tokio::main]` attribute to main.
/// 1. include generated file the following code:
///    `pub mod your_any_name {tonic::include_proto!("package_name_defined_in_proto");}`
/// 1. import:
///    `use your_any_name::<package_name_defined_in_proto>_client::<ServiceName>Client;` and
///    `use your_any_name::YourMessageName;`
#[tokio::main]
async fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    let channel = create_https_channel().await?;

    request_hello_rpc(channel.clone()).await?;

    request_hello_stream(channel.clone()).await?;

    info!("Bye");
    Ok(())
}
