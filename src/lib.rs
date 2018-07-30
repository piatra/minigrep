use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn few_args_fail() {
        let few_args: Vec<String> = vec!["minigrep".to_owned()];
        if let Err(_) = Config::new(&few_args) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn open_file() {
        let args: Vec<String> = vec!["minigrep", "foo", "poem.txt"].into_iter().map(|s| {
            s.into()
        }).collect();
        println!("debug {:?}", args);
        let config = Config::new(&args).unwrap();
        if let Err(_) = run(config) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
            );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
            );
    }
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").unwrap();

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: case_sensitive == "1"
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.split("\n").filter(|line| {
        line.to_lowercase().contains(&query)
    }).collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.split("\n").filter(|line| line.contains(query)).collect()
}
