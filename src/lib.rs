use std::error::Error;
use std::{fs,env};

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(args : &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }
        let mut query = String::new();
        let mut file_path = String::new();
        let mut found_query = false;
        let mut ignore_case = if env::var("CASE_SENSITIVE").is_ok(){
            false
        }else {
            env::var("IGNORE_CASE").is_ok()
        };

        for arg in &args[1..]{
            if !arg.starts_with("--"){
                if !found_query{
                    query = arg.clone();
                    found_query = true;
                }else if file_path.is_empty(){
                    file_path = arg.clone();
                }
            }else if arg =="--case-sensitive" {
                ignore_case = false;
            }else if arg == "--ignore-case" {
                ignore_case = true;
            }else {
                return Err("Invalid commandline argument provided");
            }
        }

        if query.is_empty() || file_path.is_empty(){
            return Err("Query and the file path must be provided");
        }
        Ok (Config{query,file_path,ignore_case})
    }
}

pub fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case{
        search_case_insensitive(&config.query,&contents)
    }else {
        search(&config.query,&contents)
    };
    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str,contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents));
    }
}
