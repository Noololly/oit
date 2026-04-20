use std::env;
use std::path::{PathBuf};
use std::fs::DirBuilder;
pub fn init(path_buf: &Option<PathBuf>){
    let mut path = match path_buf {
        Some(path) => path.clone(),
        None => env::current_dir().unwrap(),
    };

    path.push(".oit");

    DirBuilder::new()
        .recursive(true)
        .create(&path)
        .unwrap();
}