use colored::{self, Colorize};
use minigrep::{Config, run, search};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!(
            "{}",
            format!("Problem Parsing Arguments, {}!", err).red().bold()
        );
        process::exit(1);
    });

    println!(
        "{} {}",
        "Searching for =>".cyan().bold(),
        config.query.yellow().bold()
    );
    println!(
        "{} {}",
        "In The File =>".cyan().bold(),
        config.file_path.yellow().bold()
    );

    let contents = run(&config);
    let (results, total_lines) = search(&config.query, &contents);

    if results.is_empty() {
        println!("{}", "No matches found.".red().bold());
    } else {
        println!("{}", "Matches Found:".green().bold());
        for (line_number, line) in results {
            println!(
                "{} {} {} \"{}\"",
                "Line:".cyan().bold(),
                line_number.to_string().cyan().bold(),
                "=>".cyan().bold(),
                line.to_string().yellow().bold()
            );
        }
    }

    println!(
        "{} {}",
        "Total Lines: =>".cyan().bold(),
        total_lines.to_string().yellow().bold()
    );
}
