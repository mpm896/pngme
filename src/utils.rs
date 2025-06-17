// Util functions
use std::fs::{self, File, OpenOptions};
use std::io::Write;

use crate::Result;
use crate::png::Png;

pub fn read_png(filename: &String) -> Result<Png> {
    // Read a png from a file -> &[u8] -> Png
    let data: &[u8] = &fs::read(filename)?[..];
    let png: Png = Png::try_from(data)?;
    Ok(png)
}

pub fn write_png(filename: &String, data: &Png) -> Result<()> {
    let mut file: File = OpenOptions::new()
                        .read(true)
                        .create(true)
                        .write(true)
                        .open(filename.as_str())?;

    let bytes: Vec<u8> = data.as_bytes();

    // Write all bytes to the file: Overwrite if file already exists, create new file if not
    file.write(&bytes)?;
    Ok(())               
}