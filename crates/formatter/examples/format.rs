use std::fs::read_to_string;
use formatter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].to_owned();
    
    let data = read_to_string(filename).unwrap();
    
    let formatted = formatter::xformat(&data);
    
    print!("{}", formatted);
}
