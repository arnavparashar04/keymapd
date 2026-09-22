use crate::device::detect_keyboards;
mod device;
mod input;
mod output;
mod map;
use input::InputMngr;
fn main() {
    let mut devicePaths = device::detect_keyboards();
    if let Some(paths) = devicePaths{
       let mut input = InputMngr::new(paths).unwrap();
       input::InputMngr::print_devices(&input);
       let mut output = output::OutputMngr::new();
       while(true){input::InputMngr::scanevents(&mut input);}
    }
}
