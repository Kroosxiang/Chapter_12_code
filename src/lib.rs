
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
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text : {contents}");

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

//learning how to write a test here
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "
            \
            rust:
            safe, fast, productive.
            Pick three.
        ";
    }

    assert_eq!(vec!["safe, fast, productive."], search(querycontents));
}
