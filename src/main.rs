use std::io::stdin;
use std::path::Path;
use walkdir::WalkDir;

fn read_files_recursively<P : AsRef<Path>>(folder_path : P) -> Result<(), String> {
    for entry in WalkDir::new(folder_path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                println!("The path is = {:?}", path);
            }
            Err(err) => {
                println!("{}", err);
                return Err(err.to_string())
            }
        }
    }
    return Ok(())
}

#[tokio::main]
async fn main() -> () {
    let mut path_to_project : String = String::new();
    println!("Pass the path to your project");
    match stdin().read_line(&mut path_to_project) {
        Ok(_) => {
            match read_files_recursively(path_to_project) {
                Ok(()) => {

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
