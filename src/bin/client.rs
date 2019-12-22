use std::net::SocketAddr;
use std::sync::Arc;

use grpc::ClientStub;
use log::info;
use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;

use rust_grpc_sample::asset::Asset;
use rust_grpc_sample::grpc_stub::hello_rpc::HelloRequest;
use rust_grpc_sample::grpc_stub::hello_rpc_grpc::{HelloRpcService, HelloRpcServiceClient};
use rust_grpc_sample::grpc_stub::hello_stream_response::HelloStreamRequest;
use rust_grpc_sample::grpc_stub::hello_stream_response_grpc::{
    HelloStreamService, HelloStreamServiceClient,
};
use rust_grpc_sample::prelude::*;

fn request_hello_rpc(grpc_client: Arc<grpc::Client>) -> Fallible<()> {
    let service_client = HelloRpcServiceClient::with_client(grpc_client);
    let response = service_client.hello(
        Default::default(),
        HelloRequest {
            name: "Hello name".to_owned(),
            message: "Hello message".to_owned(),
            ..Default::default()
        },
    );

    println!("hello_rpc response: {:?}", response.wait());
    Ok(())
}

fn request_hello_stream(grpc_client: Arc<grpc::Client>) -> Fallible<()> {
    let service_client = HelloStreamServiceClient::with_client(grpc_client);
    let response = service_client.hello_stream(
        Default::default(),
        HelloStreamRequest {
            message: "Hello stream message".to_owned(),
            ..Default::default()
        },
    );
    let (response_meta, response_it) = response.wait()?;
    println!("hello_stream response meta: {:?}", response_meta,);

    for entry in response_it {
        println!("hello_stream response it: {:?}", entry);
    }

    Ok(())
}

fn create_https_client() -> Fallible<grpc::Client> {
    let server_der = Asset::get("server.der").ok_or_err()?;
    let root_ca = tls_api::Certificate::from_der(server_der.to_vec());
    let mut connector_builder = tls_api_native_tls::TlsConnector::builder()?;
    connector_builder.add_root_certificate(root_ca)?;
    let tls_connector = connector_builder.build()?;
    let tls_option = httpbis::ClientTlsOption::Tls("localhost".to_owned(), Arc::new(tls_connector));
    Ok(grpc::Client::new_expl(
        &SocketAddr::new("::1".parse()?, SERVER_PORT),
        "localhost",
        tls_option,
        Default::default(),
    )?)
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    let grpc_client = Arc::new(create_https_client()?);

    request_hello_rpc(grpc_client.clone())?;

    request_hello_stream(grpc_client.clone())?;

    info!("Bye");
    Ok(())
}
