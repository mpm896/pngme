use clap::{Parser, Subcommand};

use crate::commands;

/// A simple program to encode messages into PNG files and decode messages from PNG files
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    filename: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a message. 
    /// Provide a chunk type and message to encode into a PNG file. 
    /// Optionally provide an output file to prevent overwriting the original file.
    encode {
        chunk_type: String,
        message: String,

        #[arg(short, long)]
        output: Option<String>
    },

    /// Decode a message. Provide a chunk type to decode
    decode { chunk_type: String, },

    /// Remove a message. Provide a chunk type to remove. 
    remove {  chunk_type: String,  },

    /// Print the PNG file
    print,
}


#[cfg(test)]
mod test {
    use crate::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
