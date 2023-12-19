use std::{fs::read_to_string, path::Path};
use walkdir::WalkDir;
use std::collections::HashMap;
use tokenizer::Tokenizer;
use tokenizer::token::{Token, TokenType};

#[derive(PartialEq, Eq, Hash, Debug)]
struct OwnedToken {
    data: String,
    kind: TokenType
}

impl<'a> From<Token<'a>> for OwnedToken {
    fn from(value: Token) -> Self {
        OwnedToken {
            data: value.data.to_owned(),
            kind: value.kind
        }
    }
}

fn compute_cnt(toks: &mut HashMap<OwnedToken, usize>, file: &Path) -> usize {
    let mut cnt = 0;
    let data = read_to_string(file).unwrap();
    let mut tokenizer = Tokenizer::from(&*data);
    while let Some(token) = tokenizer.next() {
        toks.entry(OwnedToken::from(token))
            .and_modify(|v| *v += 1)
            .or_insert(1);
        cnt += 1;
    };
    cnt
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args[1].to_owned();
    
    let mut toks: HashMap<OwnedToken, usize> = HashMap::default();

    let cnt: usize = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".clj"))
        .map(|x| compute_cnt(&mut toks, x.path()))
        .sum();
    
    let mut kvs: Vec<(&OwnedToken, &usize)> = toks.iter().collect();
    kvs.sort_unstable_by_key(|(_, v)| **v);
    let kvs: Vec<(&OwnedToken, &usize)> = kvs.iter()
        .rev()
        .take(40)
        .cloned()
        .collect();
    
    for (tok, &cnt) in &kvs {
        println!("{}\t\t{:?}", cnt, **tok);
    }
    
    println!("{}", cnt);
}
