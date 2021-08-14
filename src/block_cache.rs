use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use spin::Mutex;

use crate::block_dev::BlockDevice;
use crate::constants::BLOCK_SIZE;

pub struct BlockCache {
    cache: [u8; BLOCK_SIZE],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache {
    pub fn new(block_device: Arc<dyn BlockDevice>, block_id: usize) -> BlockCache {
        let mut cache = [0u8; BLOCK_SIZE];
        block_device.read_block(block_id, &mut cache);

        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }

    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }

    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SIZE);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SIZE);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

const BLOCK_CACHE_SIZE: usize = 16;
pub struct BlockCacheManager {
    queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        BlockCacheManager {
            queue: VecDeque::new(),
        }
    }

    pub fn get_block_cache(
        &mut self,
        block_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>> {
        let cache = self.queue.iter().find(|(id, _)| *id == block_id);
        if let Some(block_cache) = cache {
            return block_cache.1.clone();
        }

        if self.queue.len() == BLOCK_CACHE_SIZE {
            self.evict();
        }

        let block_cache = Arc::new(Mutex::new(BlockCache::new(
            Arc::clone(&block_device),
            block_id,
        )));
        self.queue.push_back((block_id, Arc::clone(&block_cache)));
        block_cache
    }

    fn evict(&mut self) {
        if let Some((idx, _)) = self
            .queue
            .iter()
            .enumerate()
            .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
        {
            self.queue.drain(idx..=idx);
        } else {
            panic!("All caches are in use!");
        }
    }
}

lazy_static! {
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> =
        Mutex::new(BlockCacheManager::new());
}

pub fn get_block_cache(
    block_device: Arc<dyn BlockDevice>,
    block_id: usize,
) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHE_MANAGER
        .lock()
        .get_block_cache(block_id, block_device)
}
