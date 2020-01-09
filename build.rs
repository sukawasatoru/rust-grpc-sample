use std::fs::File;
use std::io::{Read, Write};

fn main() {
    generate_grpc_stub()
}

fn generate_grpc_stub() {
    let out_path = std::path::Path::new("src/grpc_stub");
    let protobuf_dir_name = "protobuf";
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let protobuf_dir = std::path::PathBuf::from(&manifest_dir).join(protobuf_dir_name);

    let proto_files = std::fs::read_dir(&protobuf_dir)
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

    let package_reg = regex::Regex::new("package *([a-z][^ ;]*);").unwrap();

    let need_update = proto_files
        .iter()
        .filter(|entry| {
            // check timestamp for build cache.

            let proto_file = File::open(entry).unwrap();
            let mut reader = std::io::BufReader::new(proto_file);
            let mut proto_string = String::new();
            reader.read_to_string(&mut proto_string).unwrap();
            let proto_cap = package_reg.captures_iter(&proto_string).next().unwrap();
            let generated_file_stem = proto_cap.get(1).unwrap().as_str().replace(".", "_");

            let generated_file_modified =
                match std::fs::metadata(out_path.join(generated_file_stem).with_extension("rs")) {
                    Ok(data) => data.modified().unwrap(),
                    Err(_) => return true,
                };

            let proto_file_modified = std::fs::metadata(entry).unwrap().modified().unwrap();
            generated_file_modified < proto_file_modified
        })
        .map(|data| data.strip_prefix(&manifest_dir).unwrap().to_owned())
        .next()
        .is_some();

    if !need_update {
        return;
    }

    for entry in std::fs::read_dir(&out_path).unwrap() {
        std::fs::remove_file(entry.unwrap().path()).unwrap();
    }

    // currently working correctly: https://github.com/danburkert/prost/issues/226
    tonic_build::configure()
        .out_dir(out_path)
        .compile(
            &proto_files.iter().map(|data| data).collect::<Vec<_>>(),
            &[&protobuf_dir],
        )
        .unwrap();

    let mod_file = File::create(out_path.with_extension("rs")).unwrap();
    let mut mod_file_writer = std::io::BufWriter::new(mod_file);
    mod_file_writer
        .write_all(
            br#"#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
"#,
        )
        .unwrap();
    for entry in std::fs::read_dir(&out_path).unwrap() {
        let file_path = entry.unwrap().path();
        let new_file_stem = file_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".", "_");
        let mut new_file_path = file_path.clone();
        new_file_path.set_file_name(format!("{}.rs", new_file_stem));

        std::fs::rename(file_path, new_file_path).unwrap();

        mod_file_writer
            .write_all(format!("pub mod {};\n", new_file_stem).as_bytes())
            .unwrap();
    }
    mod_file_writer.flush().ok();
}
