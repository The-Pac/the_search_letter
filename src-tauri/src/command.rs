use std::fs::{read, read_to_string};
use tauri::command;
use std::path::Path;
use tauri::api::dir::read_dir;
use crate::model::letter::Letter;

#[command]
pub fn search_letters(location: String) -> Result<Vec<Letter>, String> {
    let path = Path::new(&location);
    println!("Checking path {}", location);
    match path.exists() {
        true => {
            println!("Path exist!");
            let directory_content = read_dir(path, true).expect("No files");
            let mut chars: Vec<Letter> = vec![];
            let mut found = false;
            directory_content.iter().for_each(|file| {
                match read(&file.path) {
                    Ok(file_content) => {
                        file_content.iter().for_each(|byte| {
                            let byte_as_char = byte.clone() as char;
                            if *byte != 27 {
                                print!("{byte_as_char}");
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
                                }
                            }
                        })
                    }
                    Err(_) => {}
                }
            });
            Ok(chars)
        }
        false => {
            Err("Path does not exist".to_string())
        }
    }
}