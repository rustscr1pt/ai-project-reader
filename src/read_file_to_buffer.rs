use std::fs::File;
use std::path::Path;
use std::io::{Error, Read};

pub fn read_file_to_buffer(file_path : impl AsRef<Path>, code_buffer : &mut String) -> Result<(), Error> {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut buffer : String = String::new();
            match file.read_to_string(&mut buffer) {
                Ok(_) => {
                    code_buffer.push_str(&buffer);
                    return Ok(())
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Err(err) => {
            return Err(err)
        }
    }
}