use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};

use crate::block_cache::get_block_cache;
use crate::block_dev::BlockDevice;
use crate::inode::DiskInode;
use crate::lfs::LogStructuredFileSystem;

pub struct Inode {
    block_id: usize,
    block_offset: usize,
    fs: Arc<Mutex<LogStructuredFileSystem>>,
    block_device: Arc<dyn BlockDevice>,
}

impl Inode {
    pub fn new(
        block_id: usize,
        block_offset: usize,
        fs: Arc<Mutex<LogStructuredFileSystem>>,
        block_device: Arc<dyn BlockDevice>,
    ) -> Inode {
        Inode {
            block_id,
            block_offset,
            fs,
            block_device,
        }
    }

    fn read_disk_inode<V>(&self, f: impl FnOnce(&DiskInode) -> V) -> V {
        get_block_cache(self.block_device.clone(), self.block_id)
            .lock()
            .read(self.block_offset, f)
    }

    fn modify_disk_inode<V>(&self, f: impl FnOnce(&mut DiskInode) -> V) -> V {
        get_block_cache(self.block_device.clone(), self.block_id)
            .lock()
            .modify(self.block_offset, f)
    }
}
