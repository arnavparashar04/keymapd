use uinput::device::Device;
pub struct OutputMngr{
    device: uinput::Device
}

impl OutputMngr{
    pub fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let device = uinput::default()?.name("keymapd virtual keyboard")?.event(uinput::event::Keyboard::All)?.create()?;
        Ok(Self{device})
    }
}
