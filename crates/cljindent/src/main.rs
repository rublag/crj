use std::{fs::{read_to_string, OpenOptions}, path::Path, io::Write};
use walkdir::WalkDir;
use formatter;
use similar::TextDiff;

fn format(file: &Path) {
    let data = read_to_string(file).unwrap();
    let formatted = formatter::xformat(&data);
    
    if data == formatted {
        return;
    }
    
    let mut out = OpenOptions::new().write(true).truncate(true).open(file).unwrap();
    out.write(formatted.as_bytes()).unwrap();

    println!("Format {}", file.display());
}

fn diff(file: &Path) {
    let data = read_to_string(file).unwrap();
    let formatted = formatter::xformat(&data);
    let diff = TextDiff::from_lines(&data, &formatted);
    print!(
        "{}",
        diff
            .unified_diff()
            .context_radius(3)
            .header(&file.display().to_string(), &file.display().to_string())
    );
}

fn run(dir: &str, action: fn(&Path)) {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".clj"))
        .for_each(|x| action(x.path()));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args[1].to_owned();
    let dir = args[2].to_owned();
    
    if mode == "fix" {
        run(&dir, format);
    }
    else if mode == "check" {
        run(&dir, diff);
    }
    else {
        println!("Use fix or check");
    }
}
