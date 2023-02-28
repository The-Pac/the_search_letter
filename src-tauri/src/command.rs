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
            let mut chars = vec![];
            let test_duration = Instant::now();

            match read_dir(path, true) {
                Ok(directory_content) => {
                    directory_content.iter()
                        .for_each(|file| {
                            match &file.children {
                                Some(_) => {
                                    recursive_read_dir(&file_or_directory.children, chars);
                                }
                                None => {}
                            }
                            recursive_read_dir(&file.children, &mut chars);

                        });
                }
                Err(_) => {}
            }
            let result = model::result::Result::new(chars, test_duration.elapsed().as_secs());
            Ok(result)
        }
        false => {
            Err("Path does not exist".to_string())
        }
    }
}

fn recursive_read_dir(disk_entries: &Option<Vec<DiskEntry>>, chars: &mut Vec<Letter>) {
    match disk_entries {
        Some(disk_entries) => {
            disk_entries.iter().for_each(|file_or_directory| {
                match file_or_directory.children {
                    Some(_) => {
                        disk_entries.iter()
                            .for_each(|file| {
                                match read(file.path.clone()) {
                                    Ok(file_content) => {
                                        let mut found: bool = false;
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
                            })
                    }
                    None => {
                        recursive_read_dir(&file_or_directory.children, chars);
                    }
                }

            });
        }
        None => {
        }
    }
}
