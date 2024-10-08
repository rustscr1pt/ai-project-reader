use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

pub fn fill_ignored_directories(ignored_directories_vec : &mut Vec<String>) -> Result<(), Error> {
    match File::open(r#".readerignore"#) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(content) => ignored_directories_vec.push(content),
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
            return Ok(())
        }
        Err(err) => {
            return Err(err)
        }
    }
}