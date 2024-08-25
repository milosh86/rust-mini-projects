use std::{env, process};

use rust_01_hash_tool::{Config, hash_data, read_file};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    let file_content = read_file(&config.file_path).unwrap_or_else(|_e| {
        eprintln!("Failed to load file '{}'", config.file_path);
        process::exit(1);
    });

    let hash = hash_data(file_content);

    println!("\nHashing file: {}", config.file_path);
    println!("File SHA-256 hash: {}", hex::encode(hash));
}
