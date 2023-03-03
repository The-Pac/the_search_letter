use std::fs::{metadata, read, read_to_string};
use std::ops::{Add, AddAssign, Div, DivAssign};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use log::{error, info};
use serde::de::Unexpected::Str;
use serde_json::from_str;
use tauri::api::dir::{DiskEntry, read_dir};
use tauri::command;

use crate::model;
use crate::model::file::File;
use crate::model::letter::Letter;

#[command]
pub fn search_letters(location: String, selected_extensions: Vec<String>) -> Result<model::result::Result, String> {
    let path = Path::new(&location);
    info!("Research on path [{}] with selected extensions [{}] ",location,selected_extensions.join(","));
    match path.exists() {
        true => {
            info!("Path exist!");
            let mut chars = vec![];
            let mut files = vec![];
            let test_duration = Instant::now();
            let mut total_chars: u128 = 0;
            let mut total_sizes: u64 = 0;

            recursive_read_dir(path, &mut chars, &selected_extensions, &mut total_chars, &mut files, &mut total_sizes);

            chars.sort_by(|a, b| a.number.cmp(&b.number).reverse());
            for char in chars.iter_mut() {
                char.percents = format!("{:.2}%", (char.number as f64 * 100.0) / total_chars as f64);
            }

            files.sort_by(|a, b| a.size.cmp(&b.size).reverse());
            for file in files.iter_mut() {
                file.percents = format!("{:.2}%", (file.size as f64 * 100.0) / total_sizes as f64);
            }

            let result = model::result::Result::new(chars, files, test_duration.elapsed().as_secs());
            Ok(result)
        }
        false => {
            error!("Path does not exist");
            Err("Path does not exist".to_string())
        }
    }
}

fn recursive_read_dir(path: &Path, chars: &mut Vec<Letter>, selected_extensions: &Vec<String>, total_chars: &mut u128, files: &mut Vec<File>, total_sizes: &mut u64) {
    match read_dir(path, true) {
        Ok(directory_content) => {
            directory_content.iter()
                .for_each(|file| {
                    info!("File name : {}", file.name.clone().unwrap());
                    match &file.children {
                        Some(_) => {
                            info!("This file is a directory : {}",file.path.clone().to_str().unwrap());
                            recursive_read_dir(&file.path, chars, &selected_extensions, total_chars, files, total_sizes);
                        }
                        None => {
                            info!("Reading the file : {}",file.name.clone().unwrap());
                            for extension in selected_extensions.iter() {
                                match file.name.clone().unwrap().ends_with(extension) {
                                    true => {
                                        match read(file.path.clone()) {
                                            Ok(file_content) => {
                                                let file_metadata = file.path.clone().metadata().expect("Cannot get metadata");
                                                files.push(File::new(file_metadata.file_size(), file.name.clone().unwrap(), String::new()));
                                                total_sizes.add_assign(file_metadata.file_size());
                                                file_content.iter().for_each(|byte| {
                                                    let mut found: bool = false;
                                                    let byte_as_char = byte.clone() as char;
                                                    if byte_as_char != ' ' {
                                                        match chars.is_empty() {
                                                            true => {
                                                                chars.push(Letter::new(1, byte_as_char, String::new()));
                                                                &total_chars.add_assign(1);
                                                            }
                                                            false => {
                                                                for char in chars.iter_mut() {
                                                                    if char.char == byte_as_char {
                                                                        char.number += 1;
                                                                        &total_chars.add_assign(1);
                                                                        found = true;
                                                                        break;
                                                                    }
                                                                }
                                                                if !found {
                                                                    chars.push(Letter::new(1, byte_as_char, String::new()));
                                                                    &total_chars.add_assign(1);
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
    match read_to_string("config/extensions.json") {
        Ok(extensions) => {
            Ok(from_str::<Vec<String>>(extensions.as_str()).unwrap())
        }
        Err(_) => {
            error!("Cannot get the extensions");
            Err("Cannot get the extensions".to_string())
        }
    }
}