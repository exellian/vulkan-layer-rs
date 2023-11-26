use std::sync::{Arc, RwLock};
use ash::vk;
use ash::vk::Handle;
use crate::storage::{Storage, StorageDeleteGuard, StorageReadGuard, StorageWriteGuard};

pub struct InstanceDispatchTable {
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    pub destroy_instance: vk::PFN_vkDestroyInstance,
}

pub struct DeviceDispatchTable {
    pub get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    pub destroy_device: vk::PFN_vkDestroyDevice
}

static INSTANCE_DATA: Storage<u64, InstanceData> = Storage::new();
static DEVICE_DATA: Storage<u64, DeviceData> = Storage::new();

pub struct InstanceData {
    dispatch_table: InstanceDispatchTable
}

impl InstanceData {

    pub fn insert(instance: &vk::Instance, dispatch_table: InstanceDispatchTable) {
        INSTANCE_DATA.insert(instance.as_raw(), InstanceData {
            dispatch_table
        });
    }

    /// Gets instance data for a specific vulkan instance.
    /// This prevents wrapping the vulkan-instance.
    pub fn read(instance: &vk::Instance) -> StorageReadGuard<'static, InstanceData> {
        INSTANCE_DATA.read(&instance.as_raw())
    }

    pub fn write(instance: &vk::Instance) -> StorageWriteGuard<'static, InstanceData> {
        INSTANCE_DATA.write(&instance.as_raw())
    }

    pub fn dispatch_table(&self) -> &InstanceDispatchTable {
        &self.dispatch_table
    }

    pub fn delete(instance: &vk::Instance) -> StorageDeleteGuard<'static, u64, InstanceData> {
        INSTANCE_DATA.delete(instance.as_raw())
    }
}

pub struct DeviceData {
    dispatch_table: DeviceDispatchTable
}

impl DeviceData {

    pub fn insert(device: &vk::Device, dispatch_table: DeviceDispatchTable) {
        DEVICE_DATA.insert(device.as_raw(), DeviceData {
            dispatch_table
        })
    }

    pub fn read(device: &vk::Device) -> StorageReadGuard<'static, DeviceData> {
        DEVICE_DATA.read(&device.as_raw())
    }

    pub fn write(device: &vk::Device) -> StorageWriteGuard<'static, DeviceData> {
        DEVICE_DATA.write(&device.as_raw())
    }

    pub fn dispatch_table(&self) -> &DeviceDispatchTable {
        &self.dispatch_table
    }

    pub fn delete(device: &vk::Device) -> StorageDeleteGuard<'static, u64, DeviceData> {
        DEVICE_DATA.delete(device.as_raw())
    }
}