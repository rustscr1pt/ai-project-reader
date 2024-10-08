use walkdir::Error;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::is_ignored::is_ignored;

pub fn read_files_recursively(
    folder_path : impl AsRef<Path>,
    ignored_directories_vec : &Vec<String>,
    paths_buffer : &mut Vec<PathBuf>
) -> Result<(), Error> {
    for entry in WalkDir::new(folder_path)
        .into_iter()
        .filter_entry(|path| !is_ignored(path, ignored_directories_vec)) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                println!("Allowed path is : {:?}", path);
                if path.is_file() {
                    println!("Found file : {:?}", path);
                    paths_buffer.push(path.to_path_buf())
                }
            }
            Err(err) => {
                return Err(err)
            }
        }
    }
    return Ok(())
}