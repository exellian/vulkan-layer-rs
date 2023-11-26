use ash::extensions::ext::DebugUtils;
use ash::{vk, Entry};
use log::debug;
use std::ffi::CStr;

fn main() -> Result<(), ()> {
    let entry = Entry::linked();

    let layers = entry.enumerate_instance_layer_properties().unwrap();

    for layer in layers {
        let name = unsafe {
            std::slice::from_raw_parts(
                layer.layer_name.as_ptr() as *const u8,
                layer.layer_name.len(),
            )
        };
        let name = CStr::from_bytes_until_nul(name).unwrap();
        println!("{:?}", name);
    }

    let layer_names = [
        unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_lui_wsi\0") },
        unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") },
    ];
    let layers_names_raw: Vec<*const libc::c_char> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

    let mut extension_names = Vec::new();
    extension_names.push(DebugUtils::name().as_ptr());

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        extension_names.push(vk::KhrPortabilityEnumerationFn::name().as_ptr());
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        extension_names.push(vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
    }

    let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"VulkanTest\0") };

    let appinfo = vk::ApplicationInfo::builder()
        .application_name(app_name)
        .application_version(0)
        .engine_name(app_name)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0))
        .build();

    let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::default()
    };

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo)
        .enabled_layer_names(&layers_names_raw)
        .enabled_extension_names(&extension_names)
        .flags(create_flags)
        .build();

    let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };

    println!("Successfully created vulkan instance!");

    Ok(())
}
