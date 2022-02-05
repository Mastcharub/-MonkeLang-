use std::env;
use std::fs;

// #TOKENS

pub enum Token{
    ADD,
    SUB,
    MOL,
    DIV,
    NUM
}

impl Token {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::ADD,
            "-" => Self::SUB,
            "*" => Self::MOL,
            "/" => Self::DIV,
            _ => panic!("bad operator"),
        }
    }
}


fn main(){

    // #SELCTING THE FILE
    let args: Vec<String> = env::args().collect();
    let selected_file = args[1].clone();

    // #READING FILE CONTENT
    let contents = fs::read_to_string(selected_file).expect("Something went wrong reading the file");
    println!("Text:\n{}", contents);

    
}