use std::{os::fd::AsRawFd, path::PathBuf};
use crate::device;
use evdev::Device;
pub struct InputMngr{
    devices: Vec<evdev::Device>,
    epollfd :i32,
}
impl InputMngr{
    pub fn new(paths: Vec<PathBuf>) -> Result<Self, Box<dyn std::error::Error>>{
       let mut devices = Vec::new();
       let epollfd = epoll::create(false)?;
       for (index, path) in paths.iter().enumerate(){
           let device = evdev::Device::open(path)?;
           epoll::ctl(epollfd, epoll::ControlOptions::EPOLL_CTL_ADD, device.as_raw_fd(), epoll::Event::new(epoll::Events::EPOLLIN, index as u64))?;
           devices.push(device);
       }
       Ok(Self{devices, epollfd})
    }
    pub fn print_devices(&self){
        for device in &self.devices{
            println!("{:?}", device.name().unwrap_or("Unknown"));
        } 
    }

    pub fn scanevents(&mut self) ->Result<(), Box<dyn std::error::Error>>{
        let mut events = vec![epoll::Event::new(epoll::Events::empty(), 0); self.devices.len()];
        let mut waitevents = epoll::wait(self.epollfd, -1, &mut events)?;
        for event in &events[..waitevents]{
            let di = event.data as usize;
            for inputevents in self.devices[di].fetch_events()?{
                match inputevents.destructure() {
                    evdev::EventSummary::Key(_, key, 1) => {
                         println!("{:?} PRESSED", key);
                    }

                    evdev::EventSummary::Key(_, key, 0) => {
                         println!("{:?} RELEASED", key);
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }

}



