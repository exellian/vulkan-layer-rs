use crate::layer::private::{DeviceData, DeviceDispatchTable};
use crate::{get_device_proc_addr, get_instance_proc_addr, log};
use ash::vk;
use ash_layer as lvk;
use std::mem;

unsafe fn get_chain_info(
    p_device_create_info: *const vk::DeviceCreateInfo,
    layer_function: lvk::LayerFunction,
) -> *mut lvk::LayerDeviceCreateInfo {
    let mut p_layer_device_create_info: *mut lvk::LayerDeviceCreateInfo =
        mem::transmute((*p_device_create_info).p_next);
    while !p_layer_device_create_info.is_null()
        && !((*p_layer_device_create_info).s_type == vk::StructureType::LOADER_DEVICE_CREATE_INFO
            && (*p_layer_device_create_info).function == layer_function)
    {
        p_layer_device_create_info = mem::transmute((*p_layer_device_create_info).p_next);
    }
    p_layer_device_create_info
}

/**
 * @brief Implements vkCreateInstance Vulkan entrypoint.
 */
#[no_mangle]
pub extern "system" fn wsi_layer_vkCreateDevice(
    physical_device: vk::PhysicalDevice,
    p_device_create_info: *const vk::DeviceCreateInfo,
    p_allocation_callbacks: *const vk::AllocationCallbacks,
    p_device: *mut vk::Device,
) -> vk::Result {
    log!("Creating device!");
    let p_layer_device_create_info =
        unsafe { get_chain_info(p_device_create_info, lvk::LayerFunction::LAYER_LINK_INFO) };
    let Some(layer_device_create_info) = (unsafe { p_layer_device_create_info.as_mut() }) else {
        return vk::Result::ERROR_INITIALIZATION_FAILED;
    };
    let gipa: vk::PFN_vkGetInstanceProcAddr = unsafe { *layer_device_create_info.u.p_layer_info }
        .pfn_next_get_instance_proc_addr
        .unwrap();
    let gdpa: vk::PFN_vkGetDeviceProcAddr = unsafe { *layer_device_create_info.u.p_layer_info }
        .pfn_next_get_device_proc_addr
        .unwrap();

    // Move chain info for next layer. This needs to be done to prevent stack overflow.
    layer_device_create_info.u.p_layer_info =
        unsafe { (*layer_device_create_info.u.p_layer_info).p_next };

    // Get the create device function of the next layer
    let create_device: vk::PFN_vkCreateDevice =
        unsafe { get_instance_proc_addr(gipa, vk::Instance::null(), "vkCreateDevice") };

    // Call create_device on the next layer.
    let ret = unsafe {
        create_device(
            physical_device,
            p_device_create_info,
            p_allocation_callbacks,
            p_device,
        )
    };
    if ret != vk::Result::SUCCESS {
        return ret;
    }
    let device = unsafe { *p_device };

    // Fetch all function pointers we need to invoke on the next layer
    let dispatch_table = DeviceDispatchTable {
        get_device_proc_addr: unsafe { get_device_proc_addr(gdpa, device, "vkGetDeviceProcAddr") },
        destroy_device: unsafe { get_device_proc_addr(gdpa, device, "vkDestroyDevice") },
    };
    // Allocate private device data for this vulkan device statically
    // Can be retrieved in any other function call later.
    DeviceData::insert(&device, dispatch_table);

    vk::Result::SUCCESS
}

/**
 * @brief Implements vkDestroyDevice Vulkan entrypoint.
 */
#[no_mangle]
pub extern "system" fn wsi_layer_vkDestroyDevice(
    device: vk::Device,
    p_allocation_callbacks: *const vk::AllocationCallbacks,
) {
    let instance_data_guard = DeviceData::delete(&device);
    unsafe {
        (instance_data_guard.dispatch_table().destroy_device)(device, p_allocation_callbacks)
    };
    // When guard is dropped here the device data is automatically deleted for us.
}
