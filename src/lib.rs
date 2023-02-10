
use std::error::Error;
use std::fs;

pub struct Config{
    pub query     : String,
    pub file_path : String,
}

impl Config{
    pub fn build(args : &[String]) -> Result<Config , &'static str>{
        if args.len() < 3{ 
            return Err("Not Enough arguments");
        }

        let query     = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query: query.to_string(),file_path : file_path.to_string()})
    }
} 

pub fn run(config : Config) -> Result<() , Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    

    for line in search(&config.query, &contents){
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &'a str ,
    contents: &'a str)-> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(& query){
            results.push(line);
        }
    }

    results
}

//learning how to write a test here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents="
\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query,contents)
        );
    }
}
