use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let api_version = env::var("CARGO_PKG_VERSION")
        .expect("Unable to get CARGO_PKG_VERSION from the environment");
    let name = env::var("CARGO_PKG_NAME")
        .expect("Unable to get CARGO_PKG_NAME from the environment")
        .replace("-", "_");
    // Get the target directory where the executable will be placed
    let out_dir = env::var("OUT_DIR").expect("Unable to get OUT_DIR from the environment");

    let link_name = if cfg!(target_os = "windows") {
        format!("{}.dll", name)
    } else if cfg!(target_os = "linux") {
        format!("lib{}.so", name)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", name)
    } else {
        panic!("Other os than .so/.dylib/.dll are currently not supported!");
    };

    let json = format!(
        "{{ \
  \"file_format_version\" : \"1.0.0\", \
  \"layer\" : {{ \
    \"name\": \"VK_LAYER_{}\", \
    \"type\": \"GLOBAL\", \
    \"library_path\": \"{}\", \
    \"api_version\": \"{}\", \
    \"implementation_version\": \"1\", \
    \"description\": \"Sample layer - https://renderdoc.org/vulkan-layer-guide.html\", \
    \"functions\": {{ \
      \"vkGetInstanceProcAddr\": \"wsi_layer_GetInstanceProcAddr\", \
      \"vkGetDeviceProcAddr\": \"wsi_layer_GetDeviceProcAddr\" \
    }} \
  }} \
}}",
        name, link_name, api_version
    );

    // Construct the destination path in the target directory
    let dest_file = Path::new(&out_dir).join(format!("../../../{}.json", name));

    // Copy the JSON file to the target directory
    fs::write(dest_file, json).expect("Failed to write json manifest!");
}
