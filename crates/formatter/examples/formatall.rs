use std::{fs::read_to_string, path::Path};
use walkdir::WalkDir;
use formatter;
use rayon::prelude::*;

fn format(file: &Path) {
    let data = read_to_string(file).unwrap();
    let formatted = formatter::xformat(&data);
    print!("{}", formatted);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args[1].to_owned();
    
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".clj"))
        //.par_bridge()
        .for_each(|x| format(x.path()));
}
