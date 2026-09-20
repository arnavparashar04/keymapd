use crate::device::detect_keyboards;
mod device;
mod input;
mod output;
mod map;
use input::InputMngr;
fn main() {
    let mut devicePaths = device::detect_keyboards();
    if let Some(paths) = devicePaths{
       let input = InputMngr::new(paths).unwrap();
       input::print_devices(input);
    }
}
