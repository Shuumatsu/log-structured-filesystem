pub const NDIRECT: usize = 12;

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FileType {
    Invalid = 0,
    File = 1,
    Dir = 2,
    SymLink = 3,
    CharDevice = 4,
    BlockDevice = 5,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DiskInode {
    pub size: u32,
    pub type_: FileType,
    pub nlinks: u16,
    pub blocks: u32,
    pub direct: [u32; NDIRECT],
    pub indirect: u32,
    pub db_indirect: u32,

    pub device_inode_id: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct DiskEntry {
    /// inode number
    pub id: u32,
    /// file name
    pub name: [u8; 256],
}

#[repr(C)]
pub struct IndirectBlock {
    // pub entries: [u32; BLK_NENTRY],
}

pub struct DiskInodeMap {}
