use ash::vk;

/**
 * @brief Implements vkGetPhysicalDeviceSurfaceCapabilitiesKHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    pSurfaceCapabilities: *mut vk::SurfaceCapabilitiesKHR,
) -> vk::Result {
    todo!()
}

/**
 * @brief Implements vkGetPhysicalDeviceSurfaceCapabilities2KHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    physicalDevice: vk::PhysicalDevice,
    pSurfaceInfo: *const vk::PhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut vk::SurfaceCapabilities2KHR,
) -> vk::Result {
    todo!()
}

/**
 * @brief Implements vkGetPhysicalDeviceSurfaceFormatsKHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut vk::SurfaceFormatKHR,
) -> vk::Result {
    todo!()
}

/**
 * @brief Implements vkGetPhysicalDeviceSurfaceFormats2KHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDeviceSurfaceFormats2KHR(
    physicalDevice: vk::PhysicalDevice,
    pSurfaceInfo: *const vk::PhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut vk::SurfaceFormat2KHR,
) -> vk::Result {
    todo!()
}

/**
 * @brief Implements vkGetPhysicalDeviceSurfacePresentModesKHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut vk::PresentModeKHR,
) -> vk::Result {
    todo!()
}

/**
 * @brief Implements vkGetPhysicalDeviceSurfaceSupportKHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: vk::PhysicalDevice,
    queueFamilyIndex: u32,
    surface: vk::SurfaceKHR,
    pSupported: *mut vk::Bool32,
) -> vk::Result {
    todo!()
}

/**
 * @brief Implements vkDestroySurfaceKHR Vulkan entrypoint.
 */
#[no_mangle]
pub extern "C" fn wsi_layer_vkDestroySurfaceKHR(
    instance: vk::Instance,
    surface: vk::SurfaceKHR,
    pAllocator: *const vk::AllocationCallbacks,
) -> libc::c_void {
    todo!()
}
