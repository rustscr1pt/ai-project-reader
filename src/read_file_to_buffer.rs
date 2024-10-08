use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::{Error, Read};
use tokio::task::JoinHandle;

pub fn read_file_to_buffer(file_path : impl AsRef<Path>, code_buffer : Arc<Mutex<String>>) -> JoinHandle<Result<(), Error>> {
    tokio::spawn(async move {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut buffer : String = String::new();
                match file.read_to_string(&mut buffer) {
                    Ok(_) => {
                        let mut unlocked = code_buffer.lock().await;
                        unlocked.push_str(&buffer);
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
    })
}