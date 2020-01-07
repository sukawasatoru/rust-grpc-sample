fn main() {
    tonic_build::compile_protos("protobuf/hello_rpc.proto").unwrap();
    tonic_build::compile_protos("protobuf/hello_stream.proto").unwrap();
}
