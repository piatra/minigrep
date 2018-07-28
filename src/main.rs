use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process.exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
        
    println!("With text:\n{}", contents);
}
