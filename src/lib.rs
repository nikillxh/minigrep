use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    pub init: String,
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() == 1 {
            return Err("Use -h for help")
        } else if args.len() == 2 && args[1] == "-h" {
            return Err("Tool for searching line of the word in a file.
- Search Case sensitive: -s <search_word> <file_location>
- Search Case insensitive: -sci <search_word> <file_location>
- Version: -v
- Help: -h
Excess arguments will be ignored")
        } else if args.len() == 2 && args[1] == "-v"{
            return Err("minigrep 1.0")
        } else if args[1] != "-s"&& args[1] != "-sci"&& args[1] != "-v"&& args[1] != "-h" {
            return Err("Invalid flag, use -h for help.");
        } else if args.len() < 4 && (args[1] == "-s" || args[1] == "-sci"){
            return Err("Not enough arguments, use -h for help.");
        } else if args.len() > 2 && (args[1] != "-s" && args[1] != "-sci"){
            return Err("Invalid arguments, use -h for help.");
        } else if args[1] == "-sci" {
            env::set_var("CASE_INSENSITIVE", "1")
        }
        let init: String = args[1].clone();
        let query: String = args[2].clone();
        let filename: String = args[3].clone();

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {init, query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f: File = File::open(config.filename)?;

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let mut line_number: u128 = 1;
    for line in contents.lines() {
        if line.contains(query) {
            let result_line =format!("line {}: {}",line_number.to_string(), line);
            results.push(Box::leak(result_line.into_boxed_str()) as &'a str);
        }
        line_number += 1;
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}