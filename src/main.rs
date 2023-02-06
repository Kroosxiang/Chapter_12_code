use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args : Vec<String> = env::args().collect();

    //V1: let (query , file_path) = parse_config(&args);
    //V2: let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(
        |err|{
            println!("Problem parsing argument: {err}");
            process::exit(1);
    });

    // println!("We are reading {query}");
    // println!("From this file path {file_path}");
    println!("We are reading {}", config.query);
    println!("File path {}", config.file_path);

    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");
    if let Err(e) = minigrep::run(config){
        println!("Application  error : {e}");
        process::exit(1);
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
