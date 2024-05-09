use std::{env, fs};
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::PathBuf;

const CLIENTS_DIR: &str = "../../specs/clients";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Valid OUT_DIR"));
    let clients_dir = PathBuf::from(CLIENTS_DIR);

    for proto_path in fs::read_dir(clients_dir)?.flatten()
        .filter(has_proto_extension)
        .map(|entry| entry.path())
    {
        let proto_path_string = proto_path.to_str()
            .expect("Valid Proto File Path");
        let descriptor_file_name = proto_path.file_stem()
            .map(|stem| stem.to_str())
            .flatten()
            .expect("Pre-validated File Stem")
            ;
        let descriptor_path = out_dir.join(descriptor_file_name);
        println!("cargo:rerun-if-changed={}", proto_path_string);
        tonic_build::configure()
            .file_descriptor_set_path(descriptor_path)
            .build_server(false)
            .build_client(true)
            .compile(&[proto_path],
                     &[CLIENTS_DIR],
            )
            .unwrap();
    }

    Ok(())
}

fn has_proto_extension(entry: &DirEntry) -> bool {
    entry.path().extension().eq(&Some(OsStr::new("proto")))
}
