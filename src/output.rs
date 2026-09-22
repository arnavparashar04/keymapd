use uinput::device::Device;
use uinput::event::keyboard::{Key, KeyVariants};
use uinput::event::Code;
use crate::map::evdev_to_uinput;
pub struct OutputMngr{
    device: uinput::Device
}

impl OutputMngr{
    pub fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let device = uinput::default()?.name("keymapd virtual keyboard")?.event(uinput::event::Keyboard::All)?.create()?;
        Ok(Self{device})
    }
    pub fn press(&mut self, key: evdev::KeyCode){
        if let Some(key) = evdev_to_uinput(key){
            self.device.press(&key);
            self.device.synchronize();
        }
    }
    pub fn release(&mut self, key: evdev::KeyCode){
        if let Some(key) = evdev_to_uinput(key){
            self.device.release(&key);
            self.device.synchronize();
        }

    }

}


