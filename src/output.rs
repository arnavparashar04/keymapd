use std::fs::OpenOptions;
use std::os::fd::{AsRawFd, RawFd};
use std::io::Write;
use std::mem;
use uinput::event::Code;

use crate::map::uInputKey;

pub struct OutputMngr {
    file: std::fs::File,
    fd: RawFd,
}

impl OutputMngr {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().read(true).write(true).open("/dev/uinput")?;
        let fd = file.as_raw_fd();

        unsafe {
            uinput_sys::ui_set_evbit(fd, uinput_sys::EV_KEY as i32);
            uinput_sys::ui_set_evbit(fd, uinput_sys::EV_SYN as i32);

            for key in 0..uinput_sys::KEY_MAX {
                uinput_sys::ui_set_keybit(fd, key as i32);
            }

            let mut device: uinput_sys::uinput_user_dev = mem::zeroed();

            let name = b"keymapd Virtual Keyboard\0";
            for (i, byte) in name.iter().enumerate() {
                device.name[i] = *byte as i8;
            }

            device.id.bustype = 0x03;
            device.id.vendor = 0x1234;
            device.id.product = 0x5678;
            device.id.version = 1;

            file.write_all(std::slice::from_raw_parts(
                &device as *const _ as *const u8,
                mem::size_of::<uinput_sys::uinput_user_dev>(),
            ))?;

            uinput_sys::ui_dev_create(fd);
        }

        Ok(Self { file, fd })
    }

    pub fn press_uinput(&mut self, key: &uInputKey) {
        match key {
            uInputKey::Key(key) => {
                self.write_key(key.code() as u16, 1);
            }
            uInputKey::Misc(key) => {
                self.write_key(key.code() as u16, 1);
            }
        }
    }

    pub fn release_uinput(&mut self, key: &uInputKey) {
        match key {
            uInputKey::Key(key) => {
                self.write_key(key.code() as u16, 0);
            }
            uInputKey::Misc(key) => {
                self.write_key(key.code() as u16, 0);
            }
        }
    }

    pub fn press_raw(&mut self, key: evdev::KeyCode) {
        self.write_key(key.code(), 1);
    }

    pub fn release_raw(&mut self, key: evdev::KeyCode) {
        self.write_key(key.code(), 0);
    }

    fn write_key(&mut self, code: u16, value: i32) {
        let event = uinput_sys::input_event {
            time: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            kind: uinput_sys::EV_KEY as u16,
            code,
            value,
        };

        let sync = uinput_sys::input_event {
            time: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            kind: uinput_sys::EV_SYN as u16,
            code: uinput_sys::SYN_REPORT as u16,
            value: 0,
        };

        unsafe {
            libc::write(self.fd, &event as *const _ as *const libc::c_void, mem::size_of::<uinput_sys::input_event>());
            libc::write(self.fd, &sync as *const _ as *const libc::c_void, mem::size_of::<uinput_sys::input_event>());
        }
    }
}

impl Drop for OutputMngr {
    fn drop(&mut self) {
        unsafe {
            uinput_sys::ui_dev_destroy(self.fd);
        }
    }
}
