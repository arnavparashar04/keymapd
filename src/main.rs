use crate::device::detect_keyboards;
mod device;
mod input;
mod output;
mod map;
mod parser;
use input::InputMngr;
fn main() {
    let mut devicePaths = device::detect_keyboards();
    if let Some(paths) = devicePaths{
       let mut input = InputMngr::new(paths).unwrap();
       input::InputMngr::print_devices(&input);
       println!("Event scan started");
       loop{
           input::InputMngr::scanevents(&mut input);
       }
    }
}
