use crate::layer::private::{InstanceData, InstanceDispatchTable};
use crate::{get_instance_proc_addr, log};
use ash::vk;
use ash_layer as lvk;
use std::mem;

unsafe fn get_chain_info(
    p_instance_create_info: *const vk::InstanceCreateInfo,
    layer_function: lvk::LayerFunction,
) -> *mut lvk::LayerInstanceCreateInfo {
    let mut p_layer_instance_create_info: *mut lvk::LayerInstanceCreateInfo =
        mem::transmute((*p_instance_create_info).p_next);
    while !p_layer_instance_create_info.is_null()
        && !((*p_layer_instance_create_info).s_type
            == vk::StructureType::LOADER_INSTANCE_CREATE_INFO
            && (*p_layer_instance_create_info).function == layer_function)
    {
        p_layer_instance_create_info = mem::transmute((*p_layer_instance_create_info).p_next);
    }
    p_layer_instance_create_info
}

/**
 * @brief Implements vkCreateInstance Vulkan entrypoint.
 */
#[no_mangle]
pub extern "system" fn wsi_layer_vkCreateInstance(
    p_instance_create_info: *const vk::InstanceCreateInfo,
    p_allocation_callbacks: *const vk::AllocationCallbacks,
    p_instance: *mut vk::Instance,
) -> vk::Result {
    log!("Creating instance!");
    let p_layer_instance_create_info =
        unsafe { get_chain_info(p_instance_create_info, lvk::LayerFunction::LAYER_LINK_INFO) };
    let Some(layer_instance_create_info) = (unsafe { p_layer_instance_create_info.as_mut() }) else {
        return vk::Result::ERROR_INITIALIZATION_FAILED;
    };
    log!("Got chain info!");
    let gpa: vk::PFN_vkGetInstanceProcAddr = unsafe { *layer_instance_create_info.u.p_layer_info }
        .pfn_next_get_instance_proc_addr
        .unwrap();
    // Move chain for next layer.
    layer_instance_create_info.u.p_layer_info =
        unsafe { (*layer_instance_create_info.u.p_layer_info).p_next };
    let create_instance: vk::PFN_vkCreateInstance =
        unsafe { get_instance_proc_addr(gpa, vk::Instance::null(), "vkCreateInstance") };

    // Call create_instance on the next layer.
    let ret =
        unsafe { create_instance(p_instance_create_info, p_allocation_callbacks, p_instance) };
    if ret != vk::Result::SUCCESS {
        return ret;
    }
    let instance = unsafe { *p_instance };

    // Fetch all function pointers we need to invoke on the next layer
    let dispatch_table = InstanceDispatchTable {
        get_instance_proc_addr: unsafe {
            get_instance_proc_addr(gpa, instance, "vkGetInstanceProcAddr")
        },
        destroy_instance: unsafe { get_instance_proc_addr(gpa, instance, "vkDestroyInstance") },
    };
    // Allocate private instance data for this vulkan instance statically
    InstanceData::insert(&instance, dispatch_table);

    log!("Created instance!");

    vk::Result::SUCCESS
}

/**
 * @brief Implements vkDestroyInstance Vulkan entrypoint.
 */
#[no_mangle]
pub extern "system" fn wsi_layer_vkDestroyInstance(
    instance: vk::Instance,
    p_allocation_callbacks: *const vk::AllocationCallbacks,
) {
    let instance_data_guard = InstanceData::delete(&instance);
    unsafe {
        (instance_data_guard.dispatch_table().destroy_instance)(instance, p_allocation_callbacks)
    };
    // When guard is dropped here the instance data is automatically deleted for us.
}
