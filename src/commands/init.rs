use std::path::PathBuf;
pub fn init(path_buf: &Option<PathBuf>) { 
    println!("init called {:?}", path_buf);
}