use std::fs::read_to_string;
use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].to_owned();
    
    let mut token_count = 0;
    let data = read_to_string(filename).unwrap();

    let mut tokenizer = Tokenizer::from(&*data);

    while let Some(token) = tokenizer.next() {
        println!("{:?}", token);
        token_count += 1;
    };
    
    println!("{}", token_count);
}
