use std::{os::fd::AsRawFd, path::PathBuf};
use crate::device;
use evdev::Device;
use crate::map::evdev_to_uinput;
use crate::parser::{load_config, Config};
use crate::output;
pub struct InputMngr{
    devices: Vec<evdev::Device>,
    epollfd :i32,
    output: output::OutputMngr,
    config: Config,
    mappings_enabled: bool,
    hotkey_pressed: Vec<evdev::KeyCode>,
    hotkey_triggered: bool,
}
impl InputMngr{
    pub fn new(paths: Vec<PathBuf>) -> Result<Self, Box<dyn std::error::Error>>{
       let mut devices = Vec::new();
       let epollfd = epoll::create(false)?;
       let output = output::OutputMngr::new()?;
       let config = load_config()?;
       for (index, path) in paths.iter().enumerate(){
           let mut device = evdev::Device::open(path)?;
           device.grab()?;
           epoll::ctl(epollfd, epoll::ControlOptions::EPOLL_CTL_ADD, device.as_raw_fd(), epoll::Event::new(epoll::Events::EPOLLIN, index as u64))?;
           devices.push(device);

       }
       Ok(Self{devices, epollfd, output, config, mappings_enabled: true, hotkey_pressed: Vec::new(), hotkey_triggered: false})
    }
    pub fn print_devices(&self){
        for device in &self.devices{
            println!("{:?}", device.name().unwrap_or("Unknown"));
        } 
    }

    fn handle_hotkey(&mut self, key: evdev::KeyCode, value: i32) {
        if self.config.hotkey.is_empty() || !self.config.hotkey.contains(&key) {
            return;
        }
        if value == 1 {
            if !self.hotkey_pressed.contains(&key) {
                self.hotkey_pressed.push(key);
            }
            if self.hotkey_pressed.len() == self.config.hotkey.len() && !self.hotkey_triggered {
                self.mappings_enabled = !self.mappings_enabled;
                self.hotkey_triggered = true;
            }
        } else if value == 0 {
            self.hotkey_pressed.retain(|k| *k != key);
            self.hotkey_triggered = false;
        }
    }

    pub fn scanevents(&mut self) ->Result<(), Box<dyn std::error::Error>>{
        let mut events = vec![epoll::Event::new(epoll::Events::empty(), 0); self.devices.len()];
        let waitevents = epoll::wait(self.epollfd, -1, &mut events)?;
        for event in &events[..waitevents]{
            let di = event.data as usize;
            let keys: Vec<evdev::InputEvent> = self.devices[di].fetch_events()?.collect();
            for inputevents in keys{
              match inputevents.destructure() {
                    evdev::EventSummary::Key(_, key, 1) |
                    evdev::EventSummary::Key(_, key, 2) => {
					  self.handle_hotkey(key, 1);
					  if let Some(uinput_key) = evdev_to_uinput(key) {
						  if self.mappings_enabled {
							  if let Some(mapping) = self.config.mappings.iter().find(|mapping| mapping.fromKey == uinput_key)
							  {
								  self.output.press_uinput(&mapping.toKey);
							  } else {
								  self.output.press_uinput(&uinput_key);
							  }
						  } else {
							  self.output.press_raw(key);
						  }
					  }
				  }
			  evdev::EventSummary::Key(_, key, 0) => {
				  self.handle_hotkey(key, 0);
				  if let Some(uinput_key) = evdev_to_uinput(key) {
					  if self.mappings_enabled {
						  if let Some(mapping) = self.config.mappings.iter().find(|mapping| mapping.fromKey == uinput_key)
						  {
							  self.output.release_uinput(&mapping.toKey);
						  } else {
							  self.output.release_uinput(&uinput_key);
						  }
					  } else {
						  self.output.release_raw(key);
					  }
				  }
			  }

			  _ => {}
		  }  
            }
        }

        Ok(())
    }

}




