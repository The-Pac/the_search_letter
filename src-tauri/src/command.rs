use std::fs::read;
use std::path::Path;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use tauri::api::dir::{DiskEntry, read_dir};
use tauri::command;

use crate::model;
use crate::model::letter::Letter;

#[command]
pub fn search_letters(location: String) -> Result<model::result::Result, String> {
    let path = Path::new(&location);
    println!("Checking path {}", location);
    match path.exists() {
        true => {
            println!("Path exist!");
            let mut chars = vec![];
            let test_duration = Instant::now();

            recursive_read_dir(path, &mut chars);

            let result = model::result::Result::new(chars, test_duration.elapsed().as_secs());
            Ok(result)
        }
        false => {
            Err("Path does not exist".to_string())
        }
    }
}

fn recursive_read_dir(path: &Path, chars: &mut Vec<Letter>) {
    match read_dir(path, true) {
        Ok(directory_content) => {
            directory_content.iter()
                .for_each(|file| {
                    match &file.children {
                        Some(_) => {
                            recursive_read_dir(&file.path, chars);
                        }
                        None => {
                            match read(file.path.clone()) {
                                Ok(file_content) => {
                                    file_content.iter()
                                        .for_each(|byte| {
                                            let mut found: bool = false;
                                            let byte_as_char = byte.clone() as char;
                                            if byte_as_char != ' ' {
                                                match chars.is_empty() {
                                                    true => {
                                                        chars.push(Letter::new(1, byte_as_char));
                                                    }
                                                    false => {
                                                        for char in chars.iter_mut() {
                                                            if char.char == byte_as_char {
                                                                char.number += 1;
                                                                found = true;
                                                                break;
                                                            }
                                                        }
                                                        if !found {
                                                            chars.push(Letter::new(1, byte_as_char));
                                                        }
                                                    }
                                                };
                                            }
                                        });
                                }
                                Err(_) => {
                                    eprintln!("Error reading directory content");
                                }
                            }
                        }
                    }
                });
        }
        Err(_) => {
            eprintln!("Error reading directory content");
        }
    }
}
