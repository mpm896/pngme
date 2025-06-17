use std::fs;
use std::str::FromStr;

use clap::Parser;

use crate::{Cli, Commands};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::utils::{read_png, write_png};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut png: Png = read_png(&cli.filename)?;
    
    // Collect passed args
    match &cli.command {
        Commands::encode { chunk_type, message, output } => {
            encode_png(&mut png, chunk_type, message)?;
            if let Some(out_file) = output {
                write_png(out_file, &png)?;
            } else {
                write_png(&cli.filename, &png)?;
            }
        },
        Commands::decode { chunk_type } => {
            if let Ok(msg) = decode_msg(&png, chunk_type) {
                println!("[{}], {}", chunk_type, msg)
            }
        },
        Commands::remove { chunk_type } => {
            if let Ok(msg) = remove_msg(&mut png, chunk_type) {
                // Remove file and re-write the data
                fs::remove_file(&cli.filename)?;
                write_png(&cli.filename, &png)?;
                println!("Removed message: [{}] {}", chunk_type, msg)
            }
        },
        Commands::print => print_chunks(&png)
    }

    Ok(())
}

fn encode_png<'a>(
    png: &'a mut Png, 
    chunk_type: &String, 
    msg: &String
) -> Result<&'a mut Png> {
    // Get ChunkType and data as Vec<u8> to construct a Chunk
    let chunktype: ChunkType = ChunkType::from_str(chunk_type.as_str())?;
    let msg_bytes: Vec<u8> = msg.clone().into_bytes();
    let data_chunk = Chunk::new(chunktype, msg_bytes);

    // Append the chunk to the png data and return
    png.append_chunk(data_chunk);
    Ok(png)
}

fn decode_msg(png: &Png, chunk_type: &String) -> Result<String> {
    // Get Option<&Chunk> if this chunk type is found in the png
    if let Some(chunk) = png.chunk_by_type(chunk_type.as_str()) {
        let msg: String = chunk.data_as_string()?;
        Ok(msg)
    } else {
        Err("No message found.".into())
    } 
}

fn remove_msg(png: &mut Png, chunk_type: &String) -> Result<String> {
    // Search for chunktype in png file, remove it if present
    if let Ok(chunk) = png.remove_first_chunk(chunk_type.as_str()) {
        let msg: String = chunk.data_as_string()?;
        Ok(msg)
    } else {
        Err("No message found.".into())
    }
}

fn print_chunks(png: &Png) {
    println!("{}", png);
}



