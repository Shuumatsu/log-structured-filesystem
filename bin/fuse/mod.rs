#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate anyhow;

use std::borrow::BorrowMut;
use std::error::Error;
use std::io::{Read, Write};

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use tracing::{event, info, instrument, trace, Level};
use tracing_subscriber::prelude::*;

mod mkfs;
mod mount;

#[derive(Clap, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = crate_version!(), setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(version = crate_version!())]
    Mkfs(mkfs::Mkfs),
    #[clap(version = crate_version!())]
    Mount(mount::Mount),
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt()
        .pretty() // enable everything
        .with_max_level(tracing::Level::TRACE)
        .init();

    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Mkfs(config) => mkfs::mkfs(config),
        SubCommand::Mount(config) => mount::mount(config),
    }
}
