use core::error;
use clap::Parser;

use crate::args::{Cli, Commands};
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
    
    // Collect passed args
    match &cli.command {
        Commands::encode { chunk_type, message, output } => todo!(),
        Commands::decode { chunk_type } => todo!(),
        Commands::remove { chunk_type } => todo!(),
        Commands::print => todo!()
    }
    

    Ok(())
}

fn encode_png(chunk_type: &String, msg: &String, out: Option<String>) -> Result<()> {
    todo!()
}

fn decode_msg(chunk_type: &String) -> String {
    todo!()
}

fn remove_msg(chunk_type: &String) -> String {
    todo!()
}

fn print_chunks() -> Result<()> {
    todo!()
}



