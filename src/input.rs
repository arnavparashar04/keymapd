use std::path::PathBuf;
use crate::device;
use evdev::Device;
pub struct InputMngr{
    devices: Vec<evdev::Device>
}

impl InputMngr{
    pub fn new(paths: Vec<PathBuf>) -> Result<Self, Box<dyn std::error::Error>>{
       let mut devices = Vec::new();
       for path in paths{
           let device = evdev::Device::open(path)?;
           devices.push(device);
       }
       Ok(Self{devices})
    }
}

pub fn print_devices(inputmngr: InputMngr){
   for device in inputmngr.devices{
       println!("{:?}", device);
   } 
}
