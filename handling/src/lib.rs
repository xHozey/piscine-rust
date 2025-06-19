use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)         
        .append(true)        
        .create(true)        
        .open(path).unwrap();
    file.write_all(content.as_bytes()).unwrap()
    
}