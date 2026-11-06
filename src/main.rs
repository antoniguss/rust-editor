use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let file_path = &args[2];

    println!("Performing {command}");
    println!("In file {file_path}");

    if command.to_lowercase() == "append" {
        let content = &args[3];
        println!("Appending {content}");
    }
}
