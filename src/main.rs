mod fill_ignored_directories;
mod read_files_recursively;
mod is_ignored;

use crate::fill_ignored_directories::fill_ignored_directories;
use crate::read_files_recursively::read_files_recursively;

#[tokio::main]
async fn main() -> () {
    let path_to_project : String = String::from(r#"C:\Users\User\RustroverProjects\ai-project-reader"#);
    let mut ignored_directories_vec : Vec<String> = Vec::new();
    let mut paths_buffer : Vec<String> = Vec::new();
    match fill_ignored_directories(&mut ignored_directories_vec) {
        Ok(()) => {
            println!("Successfully read .readignore file : {:?}", ignored_directories_vec);
            match read_files_recursively(path_to_project.trim(), &ignored_directories_vec, &mut paths_buffer) {
                Ok(()) => {

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

                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }
}
