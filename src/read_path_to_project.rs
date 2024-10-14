use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_path_to_project() -> Result<String, Error> {
    let mut buffer : String = String::new();
    match File::open(r#".readerpath"#) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for lines in reader.lines() {
                match lines {
                    Ok(line) => {
                        buffer = line
                    }
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
            return Ok(buffer)
        }
        Err(err) => {
            return Err(err)
        }
    }
}