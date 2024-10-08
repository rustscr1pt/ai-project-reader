use walkdir::DirEntry;

pub fn is_ignored(path : &DirEntry, ignored_directories_vec : &Vec<String>) -> bool {
    let path_str = path.path().to_string_lossy();
    for ignored in ignored_directories_vec {
        if path_str.contains(ignored) {
            return true
        }
    }
    return false
}