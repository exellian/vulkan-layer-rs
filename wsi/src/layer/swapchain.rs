use ash::vk;

#[no_mangle]
pub extern "system" fn wsi_layer_vkCreateSwapchainKHR(
    device: vk::Device,
    pSwapchainCreateInfo: *const vk::SwapchainCreateInfoKHR,
    pAllocator: *const vk::AllocationCallbacks,
    pSwapchain: *mut vk::SwapchainKHR,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkDestroySwapchainKHR(
    device: vk::Device,
    swapchain: vk::SwapchainKHR,
    pAllocator: *const vk::AllocationCallbacks,
) -> libc::c_void {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkGetSwapchainImagesKHR(
    device: vk::Device,
    swapchain: vk::SwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut vk::Image,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkAcquireNextImageKHR(
    device: vk::Device,
    swapchain: vk::SwapchainKHR,
    timeout: u64,
    semaphore: vk::Semaphore,
    fence: vk::Fence,
    pImageIndex: *mut u32,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkQueuePresentKHR(
    queue: vk::Queue,
    pPresentInfo: *const vk::PresentInfoKHR,
) -> vk::Result {
    todo!()
}

/* 1.1 entrypoints */
#[no_mangle]
pub extern "C" fn wsi_layer_vkGetDeviceGroupPresentCapabilitiesKHR(
    device: vk::Device,
    pDeviceGroupPresentCapabilities: *mut vk::DeviceGroupPresentCapabilitiesKHR,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkGetDeviceGroupSurfacePresentModesKHR(
    device: vk::Device,
    surface: vk::SurfaceKHR,
    pModes: *mut vk::DeviceGroupPresentModeFlagsKHR,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut vk::Rect2D,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkAcquireNextImage2KHR(
    device: vk::Device,
    pAcquireInfo: *const vk::AcquireNextImageInfoKHR,
    pImageIndex: *mut u32,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkCreateImage(
    device: vk::Device,
    pCreateInfo: *const vk::ImageCreateInfo,
    pAllocator: *const vk::AllocationCallbacks,
    pImage: *mut vk::Image,
) -> vk::Result {
    todo!()
}

#[no_mangle]
pub extern "C" fn wsi_layer_vkBindImageMemory2(
    device: vk::Device,
    bindInfoCount: u32,
    pBindInfos: *const vk::BindImageMemoryInfo,
) -> vk::Result {
    todo!()
}
