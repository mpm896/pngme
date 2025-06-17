# PNGme: An intermediate rust project to encode messages in png files

This project assumes the the rust toolchain is already installed. If not, follow [these instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html).

This project follows the [PNGme project](https://jrdngr.github.io/pngme_book/) as a way to advance my Rust programming skills.

## Usage
1. Clone the repo: `git clone https://github.com/mpm896/pngme.git`

2. `cd` into the directory and build the binary:  
`cargo build --release`

3. `cd` into `target/release` and run `./pngme`

### Commands
For all commands, must provide in input png file.


`encode`: Encode a message within a png file. Must provide a png file, a chunk type (4 letter code), and a message. Optionally provide an output file. Example:

`./pngme file.png encode rUSt "This is a test message"` 

`decode`: Decode a message in a png file with a given chunk type. Must provide a chunk type to decode. Example:

`./pngme file.png decode rUST`

`remove`: Remove a message from a png file based on a given chunk type. Must prove a chunk type to find and remove. Example:

`./pngme file.png remove rUST`

`print`: Print the hidden messages in the png file. Example:

`./pngme file.png print`