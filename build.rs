use std::fs::File;
use std::io::Write;

fn main() {
    generate_grpc_stub()
}

fn generate_grpc_stub() {
    let out_path = std::path::Path::new("src/grpc_stub");
    let protobuf_dir_name = "protobuf";
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let protobuf_dir = std::path::PathBuf::from(&manifest_dir).join(protobuf_dir_name);

    let proto_files_all = std::fs::read_dir(protobuf_dir)
        .unwrap()
        .filter(|entry| match entry {
            Ok(entry) => entry
                .path()
                .extension()
                .map_or(false, |data| &data.to_string_lossy() == "proto"),
            Err(e) => panic!("{:?}", e),
        })
        .map(|data| data.unwrap().path())
        .collect::<Vec<_>>();

    let proto_files = proto_files_all
        .iter()
        .filter(|entry| {
            // check timestamp for build cache.

            let proto_stem = entry.file_stem().unwrap().to_str().unwrap();
            let generated_file_modified =
                match std::fs::metadata(out_path.join(proto_stem).with_extension("rs")) {
                    Ok(data) => data.modified().unwrap(),
                    Err(_) => return true,
                };

            let proto_file_modified = std::fs::metadata(entry).unwrap().modified().unwrap();
            generated_file_modified < proto_file_modified
        })
        .map(|data| data.strip_prefix(&manifest_dir).unwrap().to_owned())
        .collect::<Vec<_>>();

    if proto_files.is_empty() {
        return;
    }

    for proto_path in &proto_files {
        let name = proto_path.file_stem().unwrap().to_str().unwrap();
        let target_path = out_path.join(name).with_extension("rs");
        if target_path.exists() {
            std::fs::remove_file(target_path).unwrap();
        }

        let target_grpc_path = out_path.join(format!("{}_grpc.rs", name));
        if target_grpc_path.exists() {
            std::fs::remove_file(target_grpc_path).unwrap();
        }
    }

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: out_path.to_str().unwrap(),
        includes: &[protobuf_dir_name],
        input: &proto_files
            .iter()
            .map(|data| data.to_str().unwrap())
            .collect::<Vec<_>>(),
        rust_protobuf: true,
    })
    .unwrap();

    let mod_file = File::create(out_path.with_extension("rs")).unwrap();
    let mut mod_file_writer = std::io::BufWriter::new(mod_file);
    mod_file_writer
        .write_all(
            br#"#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
pub mod empty;
"#,
        )
        .unwrap();
    for proto_path in proto_files_all {
        let name = proto_path.file_stem().unwrap().to_str().unwrap();
        mod_file_writer
            .write_all(format!("pub mod {};\n", name).as_bytes())
            .unwrap();
        if out_path.join(format!("{}_grpc.rs", name)).exists() {
            mod_file_writer
                .write_all(format!("pub mod {}_grpc;\n", name).as_bytes())
                .unwrap();
        }
    }
    mod_file_writer.flush().ok();
}
