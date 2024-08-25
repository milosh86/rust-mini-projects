use std::fs;

use sha2::{Digest, Sha256};
use sha2::digest::consts::U32;
use sha2::digest::generic_array::GenericArray;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip program name
        let p_name = args.next().unwrap();

        println!("Running program '{}'", p_name);

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path provided!"),
        };

        Ok(Config { file_path })
    }
}

pub fn read_file(file_path: &String) -> Result<Vec<u8>, &'static str> {
    let file = match fs::read(&file_path) {
        Ok(content) => content,
        Err(_e) => return Err("FAILED_FILE_READ"),
    };

    Ok(file)
}

pub fn hash_data(data: Vec<u8>) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result
}

#[cfg(test)]
mod tests {
    use crate::{Config, hash_data, read_file};

    #[test]
    fn parse_config() {
        // arrange
        let args = vec![String::from("program-name"), String::from("file-path")].into_iter();

        // act
        let parsed_config = Config::build(args).unwrap();

        // assert
        assert_eq!(
            parsed_config,
            Config {
                file_path: String::from("file-path")
            }
        )
    }

    #[test]
    fn read_file_failed() {
        let error: &str = match read_file(&String::from("non-existent")) {
            Ok(_f) => "can't happen",
            Err(e) => e,
        };

        assert_eq!(error, "FAILED_FILE_READ");
    }

    #[test]
    fn read_file_success() {
        let expected_content = "test-123\n".as_bytes().to_vec();
        let file_content = read_file(&String::from("test-file-1.txt")).unwrap();
        assert_eq!(file_content, expected_content);
    }

    #[test]
    fn hash_file() {
        let file_content = read_file(&String::from("test-file-1.txt")).unwrap();
        let expected_hash = "cd6c8edf218a2c38d46d5b0ecf235a71185dc11f57735f607cdcc5e5339b2c3c";

        let hash = hash_data(file_content);

        assert_eq!(hex::encode(hash), expected_hash)
    }
}
