use colored::{self, Colorize};
use std::{fs, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: &Config) -> String {
    let contents = match fs::read_to_string(&config.file_path) {
        Ok(data) => data,
        Err(_) => {
            println!("{}", "Should've Been Able To Read The File!".red().bold());
            process::exit(1);
        }
    };

    contents
}

pub fn search<'a>(query: &str, contents: &'a str) -> (Vec<(usize, &'a str)>, usize) {
    let mut results = Vec::new();
    let mut linecount = 0;

    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            // Line numbers are generally 1-indexed, so add +1
            results.push((i + 1, line));
        }
        linecount = i + 1;
    }

    (results, linecount)
}
