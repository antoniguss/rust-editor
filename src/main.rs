use std::env;
use std::error::Error;
use std::process;

use editor::append;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Performing {}", config.command);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // if config.command.to_lowercase() == "append" {
    //     let content = &args[3];
    //     println!("Appending {content}");
    // }

    append(&config.file_path, &config.contents);

    Ok(())
}

struct Config {
    command: String,
    file_path: String,
    contents: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        let file_path = args[2].clone();
        let mut contents = "";

        if command.as_str() == "append" {
            if args.len() < 4 {
                return Err("not enough arguments");
            }
            contents = &args[3];
        }

        Ok(Config {
            command,
            file_path,
            contents: contents.to_string(),
        })
    }
}
