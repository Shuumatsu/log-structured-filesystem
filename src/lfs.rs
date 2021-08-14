use alloc::sync::Arc;
use spin::Mutex;

use crate::block_cache::get_block_cache;
use crate::block_dev::BlockDevice;
use crate::constants::MAX_NAME_LEN;
use crate::super_block::SuperBlock;
use crate::vfs::Inode;

pub struct InodeMapEntry {
    seg_num: usize,
    blk_num: usize,
}

pub struct LogStructuredFileSystem {
    pub block_device: Arc<dyn BlockDevice>,
    pub segment_size: usize,
}

impl LogStructuredFileSystem {
    pub fn init(block_device: Arc<dyn BlockDevice>, segment_size: usize) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(LogStructuredFileSystem {
            block_device,
            segment_size,
        }))
    }

    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
        get_block_cache(block_device.clone(), 0)
            .lock()
            .read(0, |super_block: &SuperBlock| {
                assert!(super_block.is_valid(), "Error loading LFS!");

                Arc::new(Mutex::new(LogStructuredFileSystem {
                    block_device,
                    segment_size: super_block.segment_size,
                }))
            })
    }

    pub fn root_inode(lfs: &Arc<Mutex<LogStructuredFileSystem>>) -> Inode {
        unimplemented!()
    }
}
