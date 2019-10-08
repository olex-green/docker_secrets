use std::fs::{File, read_dir};
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

const SECRET_PATH: &'static str = "/run/secrets";

/// Get Docker secrets list
pub fn get_list() -> Result<Vec<String>, &'static str> {
    let path = Path::new(SECRET_PATH);
    match read_dir(path) {
        Ok(dir) => {
            let list: Vec<String> = dir
                .filter(|entry| match entry { Ok(_) => true, Err(_) => false })
                .map(|entry| entry.unwrap().file_name().into_string().unwrap_or_default())
                .collect();

            Ok(list)
        },
        Err(_) => {
            Err("Read secrets path is failed")
        }
    }
}

/// Get a Docker secret by name
/// If error or the secret does not exist - returns Error
pub fn get(secret_name: &str) -> Result<String, &str> {
    match File::open(format!("{}/{}", SECRET_PATH, secret_name)) {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            
            if let Err(_) = buf_reader.read_to_string(&mut contents) {
               return Err("Read secrets is failed")
            }

            Ok(contents.trim().to_string())
        },
        Err(_) => {
            Err("Read secrets path is failed")
        }
    }
}
