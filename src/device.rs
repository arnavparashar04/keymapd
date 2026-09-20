use evdev::enumerate;

use crate::device;

pub fn list_devices(){
    for device in enumerate(){
        if let Some(name) = device.1.name(){
            println!("Found devices: {}", name );
        } 
    }
}
