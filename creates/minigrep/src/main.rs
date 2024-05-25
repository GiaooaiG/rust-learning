use std::env;
use std::fs;
mod tests;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);
    println!("{}", fs::read_to_string(config.filename).expect(""));
}

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments")
        };
        let filename = args[1].clone();
        let query = args[2].clone();
        Config { filename, query }
    }
}
