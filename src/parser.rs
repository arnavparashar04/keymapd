use std::fs::{self, OpenOptions};
use std::io::Write;
use evdev::KeyCode;
use uinput::event::keyboard::{Key, Misc};
use crate::map::{mapInfo, uInputKey};

pub struct Config {
    pub mappings: Vec<mapInfo>,
    pub hotkey: Vec<KeyCode>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Could not find config directory")?.join("keymapd");
    let path = config_dir.join("keymapd.conf");
    println!("Config path: {:?}", path);
    if !path.exists() {
        fs::create_dir_all(&config_dir)?;
        fs::File::create(&path)?;
    }
    let contents = fs::read_to_string(&path)?;
    let mut mappings = Vec::new();
    let mut hotkey = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
       let Some((lhs, rhs)) = line.split_once('=') else {
            return Err(format!("Invalid config line: {line}").into());
        };
        let lhs = lhs.trim();
        let rhs = rhs.trim();
        if lhs == "HOTKEY" {
            for key in rhs.split('+') {
                hotkey.push(parse_evdev_key(key.trim())?);
            }
        } else {
            mappings.push(mapInfo {
                fromKey: parse_uinput_key(lhs)?,
                toKey: parse_uinput_key(rhs)?,
            });
        }
    }
    if hotkey.is_empty() {
        let mut f = OpenOptions::new().append(true).open(&path)?;
        writeln!(f, "HOTKEY = KEY_LEFTMETA + KEY_LEFTALT + KEY_K")?;
        hotkey.extend_from_slice(&[KeyCode::KEY_LEFTMETA, KeyCode::KEY_LEFTALT, KeyCode::KEY_K]);
    }
    Ok(Config {
        mappings,
        hotkey,
    })
}
fn parse_evdev_key(key: &str) -> Result<KeyCode, Box<dyn std::error::Error>> {
    match key {
        "KEY_A" => Ok(KeyCode::KEY_A),
        "KEY_B" => Ok(KeyCode::KEY_B),
        "KEY_C" => Ok(KeyCode::KEY_C),
        "KEY_K" => Ok(KeyCode::KEY_K),
        "KEY_ESC" => Ok(KeyCode::KEY_ESC),
        "KEY_ENTER" => Ok(KeyCode::KEY_ENTER),
        "KEY_SPACE" => Ok(KeyCode::KEY_SPACE),
        "KEY_LEFTCTRL" => Ok(KeyCode::KEY_LEFTCTRL),
        "KEY_RIGHTCTRL" => Ok(KeyCode::KEY_RIGHTCTRL),
        "KEY_LEFTSHIFT" => Ok(KeyCode::KEY_LEFTSHIFT),
        "KEY_RIGHTSHIFT" => Ok(KeyCode::KEY_RIGHTSHIFT),
        "KEY_LEFTALT" => Ok(KeyCode::KEY_LEFTALT),
        "KEY_RIGHTALT" => Ok(KeyCode::KEY_RIGHTALT),
        "KEY_LEFTMETA" => Ok(KeyCode::KEY_LEFTMETA),
        "KEY_RIGHTMETA" => Ok(KeyCode::KEY_RIGHTMETA),
        "KEY_F1" => Ok(KeyCode::KEY_F1),
        "KEY_F2" => Ok(KeyCode::KEY_F2),
        "KEY_F3" => Ok(KeyCode::KEY_F3),
        "KEY_F4" => Ok(KeyCode::KEY_F4),
        "KEY_F5" => Ok(KeyCode::KEY_F5),
        "KEY_F6" => Ok(KeyCode::KEY_F6),
        "KEY_F7" => Ok(KeyCode::KEY_F7),
        "KEY_F8" => Ok(KeyCode::KEY_F8),
        "KEY_F9" => Ok(KeyCode::KEY_F9),
        "KEY_F10" => Ok(KeyCode::KEY_F10),
        "KEY_F11" => Ok(KeyCode::KEY_F11),
        "KEY_F12" => Ok(KeyCode::KEY_F12),
        "KEY_MUTE" => Ok(KeyCode::KEY_MUTE),
        "KEY_VOLUMEUP" => Ok(KeyCode::KEY_VOLUMEUP),
        "KEY_VOLUMEDOWN" => Ok(KeyCode::KEY_VOLUMEDOWN),
        "KEY_PLAYPAUSE" => Ok(KeyCode::KEY_PLAYPAUSE),
        "KEY_NEXTSONG" => Ok(KeyCode::KEY_NEXTSONG),
        "KEY_PREVIOUSSONG" => Ok(KeyCode::KEY_PREVIOUSSONG),
        _ => Err(format!("Unknown evdev key: {key}").into()),
    }
}

fn parse_uinput_key(key: &str) -> Result<uInputKey, Box<dyn std::error::Error>> {
    match key {
        "KEY_A" => Ok(uInputKey::Key(Key::A)),
        "KEY_B" => Ok(uInputKey::Key(Key::B)),
        "KEY_C" => Ok(uInputKey::Key(Key::C)),
        "KEY_ESC" => Ok(uInputKey::Key(Key::Esc)),
        "KEY_ENTER" => Ok(uInputKey::Key(Key::Enter)),
        "KEY_SPACE" => Ok(uInputKey::Key(Key::Space)),
        "KEY_LEFTCTRL" => Ok(uInputKey::Key(Key::LeftControl)),
        "KEY_RIGHTCTRL" => Ok(uInputKey::Key(Key::RightControl)),
        "KEY_LEFTSHIFT" => Ok(uInputKey::Key(Key::LeftShift)),
        "KEY_RIGHTSHIFT" => Ok(uInputKey::Key(Key::RightShift)),
        "KEY_LEFTALT" => Ok(uInputKey::Key(Key::LeftAlt)),
        "KEY_RIGHTALT" => Ok(uInputKey::Key(Key::RightAlt)),
        "KEY_LEFTMETA" => Ok(uInputKey::Key(Key::LeftMeta)),
        "KEY_RIGHTMETA" => Ok(uInputKey::Key(Key::RightMeta)),
        "KEY_F1" => Ok(uInputKey::Key(Key::F1)),
        "KEY_F2" => Ok(uInputKey::Key(Key::F2)),
        "KEY_F3" => Ok(uInputKey::Key(Key::F3)),
        "KEY_F4" => Ok(uInputKey::Key(Key::F4)),
        "KEY_F5" => Ok(uInputKey::Key(Key::F5)),
        "KEY_F6" => Ok(uInputKey::Key(Key::F6)),
        "KEY_F7" => Ok(uInputKey::Key(Key::F7)),
        "KEY_F8" => Ok(uInputKey::Key(Key::F8)),
        "KEY_F9" => Ok(uInputKey::Key(Key::F9)),
        "KEY_F10" => Ok(uInputKey::Key(Key::F10)),
        "KEY_F11" => Ok(uInputKey::Key(Key::F11)),
        "KEY_F12" => Ok(uInputKey::Key(Key::F12)),
        "KEY_MUTE" => Ok(uInputKey::Misc(Misc::Mute)),
        "KEY_VOLUMEUP" => Ok(uInputKey::Misc(Misc::VolumeUp)),
        "KEY_VOLUMEDOWN" => Ok(uInputKey::Misc(Misc::VolumeDown)),
        "KEY_PLAYPAUSE" => Ok(uInputKey::Misc(Misc::PlayPause)),
        "KEY_NEXTSONG" => Ok(uInputKey::Misc(Misc::NextSong)),
        "KEY_PREVIOUSSONG" => Ok(uInputKey::Misc(Misc::PreviousSong)),
        _ => Err(format!("Unknown uinput key: {key}").into()),
    }
}
