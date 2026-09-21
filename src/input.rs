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
}

pub fn print_devices(inputmngr: InputMngr){
   for device in inputmngr.devices{
       println!("{:?}", device.name().unwrap_or("Unknown"));
   } 
}
