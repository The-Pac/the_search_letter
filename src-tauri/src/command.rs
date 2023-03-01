use std::fs::{read, read_to_string};
use std::path::Path;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use log::{error, info};
use serde::de::Unexpected::Str;
use serde_json::from_str;
use tauri::api::dir::{DiskEntry, read_dir};
use tauri::command;

use crate::model;
use crate::model::letter::Letter;

#[command]
pub fn search_letters(location: String, selected_extensions: Vec<String>) -> Result<model::result::Result, String> {
    let path = Path::new(&location);
    info!("Research on path [{}] with selected extensions [{}] ",location,selected_extensions.join(","));
    match path.exists() {
        true => {
            info!("Path exist!");
            let mut chars = vec![];
            let test_duration = Instant::now();

            recursive_read_dir(path, &mut chars, &selected_extensions);

            let result = model::result::Result::new(chars, test_duration.elapsed().as_secs());
            Ok(result)
        }
        false => {
            error!("Path does not exist");
            Err("Path does not exist".to_string())
        }
    }
}

fn recursive_read_dir(path: &Path, chars: &mut Vec<Letter>, selected_extensions: &Vec<String>) {
    match read_dir(path, true) {
        Ok(directory_content) => {
            directory_content.iter()
                .for_each(|file| {
                    info!("File name : {}", file.name.clone().unwrap());
                    match &file.children {
                        Some(_) => {
                            info!("This file is a directory : {}",file.path.clone().to_str().unwrap());
                            recursive_read_dir(&file.path, chars, &selected_extensions);
                        }
                        None => {
                            info!("Reading the file : {}",file.name.clone().unwrap());
                            for extension in selected_extensions.iter() {
                                match file.name.clone().unwrap().ends_with(extension) {
                                    true => {
                                        match read(file.path.clone()) {
                                            Ok(file_content) => {
                                                file_content.iter().for_each(|byte| {
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
                                    false => {
                                        continue;
                                    }
                                }
                            };
                        }
                    }
                });
        }
        Err(_) => {
            error!("Error reading directory content");
        }
    }
}

#[command]
pub fn get_all_extensions() -> Result<Vec<String>, String> {
    match read_to_string("src/config/extensions.json") {
        Ok(extensions) => {
            Ok(from_str::<Vec<String>>(extensions.as_str()).unwrap())
        }
        Err(_) => {
            error!("Cannot get the extensions");
            Err("Cannot get the extensions".to_string())
        }
    }
}