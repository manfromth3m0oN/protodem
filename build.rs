use std::fs;

fn main() {
    let paths: Vec<std::ffi::OsString> = fs::read_dir("resources/proto")
        .unwrap()
        .map(|p| {
            let osstr = p.unwrap().path();
            println!("{}", osstr.display());
            osstr.into_os_string()
        })
        .collect();

    prost_build::compile_protos(&paths, &["resources/proto"]).unwrap();
}
