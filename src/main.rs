use std::fs::{self, File, OpenOptions};
use std::str::FromStr;

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
    let mut png: Png = read_png(&cli.filename)?;
    
    // Collect passed args
    match &cli.command {
        Commands::encode { chunk_type, message, output } => {
            encode_png(&mut png, chunk_type, message);
            if let Some(out_file) = output {
                write_png(out_file, &png);
            } else {
                write_png(&cli.filename, &png);
            }
        },
        Commands::decode { chunk_type } => todo!(),
        Commands::remove { chunk_type } => todo!(),
        Commands::print => print_chunks(&png)
    }

    Ok(())
}

fn read_png(filename: &String) -> Result<Png> {
    // Read a png from a file -> &[u8] -> Png
    let data: &[u8] = &fs::read(filename)?[..];
    let png: Png = Png::try_from(data)?;
    Ok(png)
}

fn write_png(filename: &String, data: &Png) -> Result<()> {
    let file: File = OpenOptions::new()
                        .read(true)
                        .create(true)
                        .append(true)
                        .open(filename.as_str())?;

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

fn decode_msg(png: &Png, chunk_type: &String) -> String {
    todo!()
}

fn remove_msg(png: &Png, chunk_type: &String) -> String {
    todo!()
}

fn print_chunks(png: &Png) {
    println!("{}", png);
}



