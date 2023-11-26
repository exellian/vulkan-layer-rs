use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Unable to get OUT_DIR from the environment");

    let json_path = Path::new(&out_dir)
        .join("../../../")
        .canonicalize()
        .unwrap();
    let path = json_path.to_string_lossy();
    println!("cargo:VK_ADD_LAYER_PATH={path}");
    println!("cargo:rustc-env=VK_ADD_LAYER_PATH={path}");
}
