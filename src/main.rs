mod fill_ignored_directories;
mod read_files_recursively;
mod is_ignored;
mod read_file_to_buffer;
mod copy_to_clipboard;
mod read_path_to_project;

use std::path::PathBuf;
use crate::copy_to_clipboard::copy_to_clipboard;
use crate::fill_ignored_directories::fill_ignored_directories;
use crate::read_file_to_buffer::read_file_to_buffer;
use crate::read_files_recursively::read_files_recursively;
use crate::read_path_to_project::read_path_to_project;

#[tokio::main]
async fn main() -> () {
    match read_path_to_project() {
        Ok(path_to_project) => {
            let mut ignored_directories_vec : Vec<String> = vec![String::from(".readerignore"), String::from(".readerpath")];
            let mut paths_buffer : Vec<PathBuf> = Vec::new();
            let mut code_buffer : String = String::new();
            match fill_ignored_directories(&mut ignored_directories_vec) {
                Ok(()) => {
                    println!("Successfully read .readignore file : {:?}", ignored_directories_vec);
                    match read_files_recursively(path_to_project.trim(), &ignored_directories_vec, &mut paths_buffer) {
                        Ok(()) => {
                            println!("Success on filling the buffer with paths.\n{:#?}", paths_buffer);
                            for file_path in paths_buffer {
                                match read_file_to_buffer(file_path, &mut code_buffer) {
                                    Ok(()) => {}
                                    Err(err) => {
                                        println!("Error when trying to fill the buffer \n{}", err)
                                    }
                                }
                            };
                            match copy_to_clipboard(code_buffer) {
                                Ok(()) => {
                                    println!("Clipboard is filled with all files")
                                }
                                Err(err) => {
                                    println!("{}", err)
                                }
                            }
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }
                Err(err) => {
                    println!("Error when trying to read .readerignore\n{}", err);
                    match read_files_recursively(path_to_project.trim(), &ignored_directories_vec, &mut paths_buffer) {
                        Ok(()) => {
                            println!("Success on filling the buffer with paths.\n{:#?}", paths_buffer)
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
