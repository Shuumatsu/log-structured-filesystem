#![no_std]
extern crate alloc;

const MAGIC: u32 = 0x6d414653;
const ROOT_INODE: u32 = 1;
const INODE_SIZE: u64 = 128;
pub const SUPERBLOCK_SIZE: u64 = 1024;
pub const DIRECT_POINTERS: u64 = 12;

mod block_cache;
mod block_dev;
mod constants;
mod inode;
mod lfs;
mod segment;
mod super_block;
mod vfs;

pub use block_dev::BlockDevice;
pub use lfs::LogStructuredFileSystem;
pub use vfs::Inode;
