mod device;
mod instance;
mod private;
mod surface;
mod swapchain;

pub use device::*;
pub use instance::*;

use crate::layer::device::{wsi_layer_vkCreateDevice, wsi_layer_vkDestroyDevice};
use crate::layer::instance::{wsi_layer_vkCreateInstance, wsi_layer_vkDestroyInstance};
use crate::layer::private::{DeviceData, InstanceData};
use crate::log;
use ash::vk;
use std::ffi::{CStr, CString};
use std::mem;

pub unsafe fn get_instance_proc_addr<T>(
    gpa: vk::PFN_vkGetInstanceProcAddr,
    instance: vk::Instance,
    proc_name: &str,
) -> T {
    let pn = CString::new(proc_name).expect("Failed to convert proc_name!");
    let p_fn = gpa(instance, pn.as_ptr()).unwrap();
    mem::transmute_copy::<_, T>(&p_fn)
}

pub unsafe fn get_device_proc_addr<T>(
    gpa: vk::PFN_vkGetDeviceProcAddr,
    device: vk::Device,
    proc_name: &str,
) -> T {
    let pn = CString::new(proc_name).expect("Failed to convert proc_name!");
    let p_fn = gpa(device, pn.as_ptr()).unwrap();
    mem::transmute_copy::<_, T>(&p_fn)
}

/**
 * @brief Implements vkGetInstanceProcAddr Vulkan entrypoint.
 * returns the entry-points of instance functions of our layer.
 */
#[no_mangle]
pub extern "system" fn wsi_layer_GetInstanceProcAddr(
    instance: vk::Instance,
    p_name: *const libc::c_char,
) -> vk::PFN_vkVoidFunction {
    let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();

    log!("Get instance proc: {}", name);
    // match strings against function names that this layer should export
    match name {
        "vkGetInstanceProcAddr" => unsafe {
            mem::transmute(wsi_layer_GetInstanceProcAddr as *const ())
        },
        "vkCreateInstance" => unsafe { mem::transmute(wsi_layer_vkCreateInstance as *const ()) },
        "vkDestroyInstance" => unsafe { mem::transmute(wsi_layer_vkDestroyInstance as *const ()) },
        _ => {
            let instance_data_guard = InstanceData::read(&instance);
            // If the function is not intercepted by our layer we simply forward the call to the next
            // layer
            unsafe {
                (instance_data_guard.dispatch_table().get_instance_proc_addr)(instance, p_name)
            }
        }
    }
}

/**
 * @brief Implements vkGetDeviceProcAddr Vulkan entrypoint.
 * returns the entry-points of device functions of our layer.
 */
#[no_mangle]
pub extern "system" fn wsi_layer_GetDeviceProcAddr(
    device: vk::Device,
    p_name: *const libc::c_char,
) -> vk::PFN_vkVoidFunction {
    let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
    match name {
        "vkGetDeviceProcAddr" => unsafe {
            mem::transmute(wsi_layer_GetDeviceProcAddr as *const ())
        },
        "vkCreateDevice" => unsafe { mem::transmute(wsi_layer_vkCreateDevice as *const ()) },
        "vkDestroyDevice" => unsafe { mem::transmute(wsi_layer_vkDestroyDevice as *const ()) },
        _ => {
            let device_data_guard = DeviceData::read(&device);
            // If the device function is not intercepted by our layer we just forward it
            // to the next layer.
            unsafe { (device_data_guard.dispatch_table().get_device_proc_addr)(device, p_name) }
        }
    }
}
