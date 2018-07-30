use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

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
}

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(())
}
