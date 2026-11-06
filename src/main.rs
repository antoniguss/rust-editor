use std::process;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let first_arg = &args[1];

    println!("Performing {}", config.command);
    println!("Performing {first_arg}");
    println!("In file {}", config.file_path);

    if config.command.to_lowercase() == "append" {
        let content = &args[3];
        println!("Appending {content}");
    }

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}

struct Config {
    command: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { command, file_path })
    }
}
