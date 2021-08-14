use crate::constants::MAGIC_NUMBER;

pub struct Addr {
    inode_num: usize,
    offset: usize,
}

#[derive(Debug)]
pub struct SuperBlock {
    pub magic_number: u32,
    pub blocks: u32,
    pub unused_blocks: u32,
    pub current_seg_id: u32,
    pub next_ino_number: u32,
    pub segment_size: usize,
    pub segments_cnt: usize,
}

impl SuperBlock {
    pub fn is_valid(&self) -> bool {
        self.magic_number == MAGIC_NUMBER
    }
}
