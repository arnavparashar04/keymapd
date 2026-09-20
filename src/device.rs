use evdev::{Device, EventType, KeyCode, enumerate};
use std::{fs, path::{self, PathBuf}};

use crate::device;

pub fn list_keyboards(){
    match detect_keyboards(){
        Some(keyboards) => {
            for keyboard in keyboards  {
                println!("Keyboard: {}",keyboard.display());
            }
        },
        None => {println!("No Keyboards Found");} 
    }
}

pub fn detect_keyboards() -> Option<Vec<PathBuf>>{
    let mut keyboards : Vec<PathBuf> = Vec::new();
    for (path, device) in enumerate(){
        if let Some(keys) = device.supported_keys(){
            if keys.contains(KeyCode::KEY_Q) && keys.contains(KeyCode::KEY_W) && keys.contains(KeyCode::KEY_ENTER) && keys.contains(KeyCode::KEY_SPACE){
                keyboards.push(path);
            }
        }
    }
    if keyboards.is_empty(){
        None
    }else {
        Some(keyboards)        
    }
}

