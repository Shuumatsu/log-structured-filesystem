use std::borrow::BorrowMut;
use std::error::Error;
use std::io::{Read, Write};

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use tracing::{event, info, instrument, trace, Level};

#[derive(Clap, Debug)]
#[clap(about = "Mount a new file system")]
pub struct Mount {
    #[clap(short, long, about = "Location of the file system image")]
    image: String,
    #[clap(short, long, about = "Mountpoint")]
    mount_point: String,
}

pub fn mount(config: Mount) -> Result<()> {
    unimplemented!()
}
