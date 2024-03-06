use colored::*;
use std::{error::Error, fs, io};

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub struct Config {
    pub command: String,
    pub query: String,
    pub value: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

pub fn search(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.value)?;

    let results = filter_contents(&config.query, &contents);

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn filter_contents<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn read_lines(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}

pub fn read_directories(file_path: &str) -> Result<(), Box<dyn Error>> {
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

pub fn find_file(directory: &str, filename: &str) -> Result<(), Box<dyn Error>> {
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

        assert_eq!(
            vec!["safe, fast, productive."],
            filter_contents(query, contents)
        );
    }
}
