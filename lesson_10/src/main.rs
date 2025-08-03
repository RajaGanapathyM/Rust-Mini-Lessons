use std::env;
use std::fs;
use std::io;

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

fn main() -> Result<(), io::Error> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Command not in format: {} <file_path>", args[0]);
    }

    let file_path = &args[1];
    match read_file_content(file_path) {
        Ok(content) => {
            println!("File Content:\n{}", content);
            Ok(())
            
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}
