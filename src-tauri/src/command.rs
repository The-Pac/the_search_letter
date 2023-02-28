use std::fs::read;
use std::path::Path;
use std::thread::spawn;
use std::time::Instant;

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
            let mut chars: Vec<Letter> = vec![];
            let mut found = false;
            let test_duration = Instant::now();
            recursive_read_dir()
        }
        false => {
            Err("Path does not exist".to_string())
        }
    }
}

fn recursive_read_dir(path: &Path,) {
    match read_dir(path, true) {
        Ok(directory_content) => {
            directory_content.iter()
                .for_each(|file| {
                    println!("{}", file.name.clone().expect("dd"));

                    match file.children {
                        Some(childrens) => {
                            recursive_read_dir(&path);
                        }
                        None => {
                            match read(&file.path) {
                                Ok(file_content) => {
                                    file_content.iter()
                                        .for_each(|byte| {
                                            let byte_as_char = byte.clone() as char;
                                            if *byte != 27 {
                                                match chars.is_empty() {
                                                    true => {
                                                        chars.push(Letter::new(byte.clone(), 1, byte_as_char));
                                                    }
                                                    false => {
                                                        for char in chars.iter_mut() {
                                                            if char.byte == *byte {
                                                                char.number += 1;
                                                                found = true;
                                                                break;
                                                            }
                                                        }
                                                        if !found {
                                                            chars.push(Letter::new(byte.clone(), 1, byte_as_char));
                                                        }
                                                    }
                                                };
                                            }
                                        });
                                }
                                Err(_) => {}
                            }
                        }
                    }
                });
            Ok(model::result::Result::new(chars, Instant::now().duration_since(test_duration).as_secs()))
        }
        Err(_) => {
            Err("Cannot read the directory".to_string())
        }
    }
}