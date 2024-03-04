use colored::*;
use std::{env, error::Error, fs, io, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let command = args[1].clone();
        let query = args[2].clone();
        let value = if args.len() > 3 {
            args[3].clone()
        } else {
            String::new()
        };

        Ok(Config {
            command,
            query,
            value,
        })
    }
}

struct Config {
    command: String,
    query: String,
    value: String,
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command.as_str() {
        "echo" => {
            println!("{}", config.query.blink());
            Ok(())
        }
        "cat" => read_lines(&config.query),
        "ls" => read_directories(&config.query),
        "find" => find_file(&config.query, &config.value),
        "grep" => search(config),
        _ => {
            println!("something else!");
            Ok(())
        }
    }
}

fn search(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.value)?;

    let results: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(&config.query))
        .collect();

    for line in results {
        println!("{line}")
    }

    Ok(())
}

fn read_lines(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn read_directories(file_path: &str) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(file_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for line in dir {
        if let Some(line_str) = line.to_str() {
            println!(
                "{}",
                if line.is_dir() {
                    line_str.blue()
                } else {
                    line_str.white()
                }
            );
        } else {
            println!("{:?}", line);
        }
    }

    Ok(())
}

fn find_file(directory: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_file(&path.to_string_lossy(), filename)?;
        } else if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if file_name_str.contains(filename) {
                    println!("{}", path.to_string_lossy())
                }
            }
        }
    }

    Ok(())
}
