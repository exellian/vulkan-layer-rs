use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Storage<K, V> {
    storage: UnsafeCell<Option<HashMap<K, V>>>,
    lock: RwLock<()>
}

unsafe impl<K, V> Sync for Storage<K, V> {}

pub struct StorageWriteGuard<'a, V> {
    value: &'a mut V,
    guard: RwLockWriteGuard<'a, ()>
}

pub struct StorageReadGuard<'a, V> {
    value: &'a V,
    guard: RwLockReadGuard<'a, ()>
}

pub struct StorageDeleteGuard<'a, K, V> where K: Eq + Hash {
    key: K,
    value: &'a mut V,
    storage: &'a UnsafeCell<Option<HashMap<K, V>>>,
    guard: RwLockWriteGuard<'a, ()>
}

impl<'a, V> Deref for StorageReadGuard<'a, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, V> Deref for StorageWriteGuard<'a, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, V> DerefMut for StorageWriteGuard<'a, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, K, V> Deref for StorageDeleteGuard<'a, K, V> where K: Eq + Hash {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, K, V> DerefMut for StorageDeleteGuard<'a, K, V> where K: Eq + Hash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, K, V> Drop for StorageDeleteGuard<'a, K, V> where K: Eq + Hash {
    fn drop(&mut self) {
        unsafe { (*self.storage.get()).as_mut().unwrap().remove(&self.key) };
    }
}

impl<K, V> Storage<K, V> where K: Eq + Hash {

    pub const fn new() -> Self {
        Storage {
            storage: UnsafeCell::new(None),
            lock: RwLock::new(()),
        }
    }

    unsafe fn get_mut(&self) -> &mut HashMap<K, V> {
        let storage = self.storage.get().as_mut().unwrap();
        if storage.is_none() {
            *storage = Some(HashMap::new());
        }
        storage.as_mut().unwrap()
    }

    unsafe fn get(&self) -> &HashMap<K, V> {
        let storage = self.storage.get().as_ref().unwrap();
        if storage.is_none() {
            panic!("Can't retrieve value when storage was not initialized!");
        }
        storage.as_ref().unwrap()
    }

    pub fn insert(&self, key: K, value: V) {
        let _lock = self.lock.write().unwrap();
        let storage = unsafe { self.get_mut() };
        storage.insert(key, value);
    }

    /// Gets instance data for a specific vulkan instance.
    /// This prevents wrapping the vulkan-instance.
    pub fn write<'a>(&'a self, key: &K) -> StorageWriteGuard<'a, V> {
        let guard = self.lock.write().unwrap();
        let storage = unsafe { self.get_mut() };
        let value = storage.get_mut(key).expect("Failed to retrieve missing value!");
        StorageWriteGuard {
            value,
            guard
        }
    }

    pub fn read<'a>(&'a self, key: &K) -> StorageReadGuard<'a, V> {
        let guard = self.lock.read().unwrap();
        let storage = unsafe { self.get() };
        let value = storage.get(key).expect("Failed to retrieve missing value!");
        StorageReadGuard {
            value,
            guard
        }
    }

    pub fn delete<'a>(&'a self, key: K) -> StorageDeleteGuard<'a, K, V> {
        let guard = self.lock.write().unwrap();
        let storage = unsafe { self.get_mut() };
        let value = storage.get_mut(&key).expect("Failed to retrieve missing value!");
        StorageDeleteGuard {
            key,
            value,
            storage: &self.storage,
            guard,
        }
    }
}
