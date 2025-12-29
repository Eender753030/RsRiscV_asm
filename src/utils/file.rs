use std::fs;
use std::path::Path;
use std::io::{self, Write};

pub fn read_asm(filename: &str) -> io::Result<String> {
    let filepath = Path::new(filename);
    
    if !filepath.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, ""));
    }

    match filepath.extension() {
        Some(ext) => {
            if ext == "asm" || ext == "s" {
                fs::read_to_string(filepath)
            } else {
                return Err(io::Error::new(io::ErrorKind::NotFound, ""))
            }
        },
        
        None => {
            fs::read_to_string(filepath)
        }
    } 
}

pub fn write_binary(filename: &str, binary_contents: &Vec<u8>) -> io::Result<()> {
    let file = fs::File::create(filename)?;

    let mut writer = io::BufWriter::new(file);

    writer.write_all(binary_contents)?;

    Ok(())
}