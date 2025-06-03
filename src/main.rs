use core::error;
use clap::Parser;

use crate::args::Cli;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;


mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    Ok(())
}