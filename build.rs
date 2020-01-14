use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    generate_grpc_stub(
        &[&PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("protobuf")],
        &Path::new("src/grpc_stub"),
    )
}

fn generate_grpc_stub(includes: &[&Path], out_path: &Path) {
    let convert_package_to_path =
        |package_name: &str| PathBuf::from(package_name.replace(".", "/")).with_extension("rs");

    struct WalkProto<'s> {
        f: &'s dyn Fn(&Self, &Path) -> Vec<PathBuf>,
    }

    let walk_proto = WalkProto {
        f: &|walk_proto, path| -> Vec<PathBuf> {
            std::fs::read_dir(path)
                .unwrap()
                .filter(|entry| match entry {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            entry
                                .path()
                                .extension()
                                .map_or(false, |data| &data.to_string_lossy() == "proto")
                        } else {
                            true
                        }
                    }
                    Err(e) => panic!("{:?}", e),
                })
                .map(|data| {
                    let target = data.unwrap().path();
                    if target.is_file() {
                        vec![target]
                    } else {
                        (walk_proto.f)(&walk_proto, &target)
                    }
                })
                .flatten()
                .collect::<Vec<_>>()
        },
    };

    let proto_files = includes
        .iter()
        .flat_map(|dir| (walk_proto.f)(&walk_proto, dir.as_ref()))
        .collect::<Vec<_>>();

    let package_reg = regex::Regex::new("package *([a-z][^ ;]*);").unwrap();

    let need_update = proto_files
        .iter()
        .filter(|entry| {
            // check timestamp for build cache.

            let proto_file = File::open(entry).unwrap();
            let mut reader = BufReader::new(proto_file);
            let mut proto_string = String::new();
            reader.read_to_string(&mut proto_string).unwrap();
            let proto_cap = package_reg.captures_iter(&proto_string).next().unwrap();
            let generated_file_modified = match std::fs::metadata(
                out_path.join(convert_package_to_path(proto_cap.get(1).unwrap().as_str())),
            ) {
                Ok(data) => data.modified().unwrap(),
                Err(_) => return true,
            };

            let proto_file_modified = std::fs::metadata(entry).unwrap().modified().unwrap();
            generated_file_modified < proto_file_modified
        })
        .next()
        .is_some();

    if !need_update {
        return;
    }

    struct RemoveRS<'s> {
        f: &'s dyn Fn(&Self, &Path),
    }

    let remove_rs = RemoveRS {
        f: &|remove_rs, path| {
            for entry in std::fs::read_dir(path).unwrap() {
                let target = entry.unwrap().path();
                if target.is_dir() {
                    (remove_rs.f)(remove_rs, &target);
                    std::fs::remove_dir(target).ok();
                    continue;
                }

                if &target.extension().unwrap().to_string_lossy() == "rs" {
                    std::fs::remove_file(target).unwrap();
                }
            }
        },
    };

    (remove_rs.f)(&remove_rs, &out_path);

    // currently working correctly: https://github.com/danburkert/prost/issues/226
    tonic_build::configure()
        .out_dir(out_path)
        .compile(
            &proto_files.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
            includes,
        )
        .unwrap();

    for entry in std::fs::read_dir(out_path).unwrap() {
        let source = entry.unwrap().path();
        let destination = out_path
            .join(
                &source
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .replace(".", "/"),
            )
            .with_extension("rs");
        std::fs::create_dir_all(&destination.parent().unwrap()).unwrap();
        std::fs::rename(source, &destination).unwrap();
    }

    struct ExportMod<'s> {
        f: &'s dyn Fn(&Self, &Path),
    }

    let export_mod = ExportMod {
        f: &|export_mod, mod_dir_path| {
            let mod_file = File::create(mod_dir_path.with_extension("rs")).unwrap();
            let mut mod_file_writer = BufWriter::new(mod_file);

            mod_file_writer
                .write_all(
                    br#"#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
"#,
                )
                .unwrap();

            for entry in std::fs::read_dir(mod_dir_path).unwrap() {
                let target_path = entry.unwrap().path();
                if target_path.is_dir() {
                    (export_mod.f)(export_mod, &target_path);
                }
                mod_file_writer
                    .write_all(
                        format!(
                            "pub mod {};\n",
                            target_path.file_stem().unwrap().to_string_lossy()
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        },
    };

    (export_mod.f)(&export_mod, &out_path);
}
