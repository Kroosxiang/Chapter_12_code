use std::{env, fs};

fn main() {
    let args : Vec<String> = env::args().collect();

    //V1: let (query , file_path) = parse_config(&args);
    //V2: let config = parse_config(&args);
    let config = Config::new(&args);

    // println!("We are reading {query}");
    println!("We are reading {}", config.query);
    // println!("From this file path {file_path}");
    println!("File path {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config{
    query : String,
    file_path :String,
}

impl Config{
    fn new(args : &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {query: query.to_string(),file_path : file_path.to_string()}
    }
}

/* fn parse_config(args : &[String]) -> Config{  //(&str , &str)
    // let query = &args[1];
    let query = &args[1].clone(); //hard copy
    // let file_path = &args[2];
    let file_path = &args[2].clone(); //hard copy

    // (query, file_path)
    Config {query: query.to_string(),file_path : file_path.to_string()}
} */
