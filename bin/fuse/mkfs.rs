use std::borrow::BorrowMut;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use anyhow::{Context, Result};
use byte_unit::{Byte, ByteUnit};
use clap::{AppSettings, Clap};
use tracing::{event, info, instrument, trace, Level};

#[derive(Clap, Debug)]
#[clap(about = "Create a new file system")]
pub struct Mkfs {
    #[clap(short, long, default_value = "4096", possible_values = &["1024", "2048", "4096"])]
    block_size: u32,
    #[clap(
        short,
        long,
        about = "Specify the total size of the file system. The final size might be bigger than the provided value in order to have space for the file system structures."
    )]
    system_size: String,
    #[clap(short, long, about = "Location of the new file system image")]
    file: String,
}

pub fn mkfs(config: Mkfs) -> Result<()> {
    let file_size = Byte::from_str(config.system_size).unwrap();
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config.file)?;
    let mut buf = BufWriter::new(&file);
    let uid = nix::unistd::geteuid().as_raw();
    let gid = nix::unistd::getegid().as_raw();

    unimplemented!()
}
