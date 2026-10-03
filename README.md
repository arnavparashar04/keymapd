# keymapd

A simple key remapping daemon written in Rust.

I made this because my keyboard doesn't have a built-in Function Lock (Fn Lock) key, and holding `Fn` every time to use function or media keys was annoying. `keymapd` listens for keyboard events via `evdev`, remaps them according to a configuration file, and sends out the new keypresses using `uinput`.

## Features

- Custom key remapping using `evdev` and `uinput`.
- Simple text configuration file.
- Toggle shortcut (default: `Super + Alt + K`) to enable or disable remapping on the fly.

## Configuration

The configuration file is located at:

```
~/.config/keymapd/keymapd.conf
```

If it doesn't exist when `keymapd` runs, it will be created automatically with default settings.

### Config Format

The file uses `KEY_FROM = KEY_TO` mapping format. You can also define a toggle hotkey.

Example `~/.config/keymapd/keymapd.conf`:

```ini
# Shortcut to toggle remapping on/off
HOTKEY = KEY_LEFTMETA + KEY_LEFTALT + KEY_K

# Remap Function keys to Media keys
KEY_F1 = KEY_MUTE
KEY_F2 = KEY_VOLUMEDOWN
KEY_F3 = KEY_VOLUMEUP
KEY_F5 = KEY_PREVIOUSSONG
KEY_F6 = KEY_PLAYPAUSE
KEY_F7 = KEY_NEXTSONG
```

### Supported Keys

Supported key names match Linux evdev event names:

- **Letters & Misc:** `KEY_A`, `KEY_B`, `KEY_C`, `KEY_K`, `KEY_ESC`, `KEY_ENTER`, `KEY_SPACE`
- **Modifiers:** `KEY_LEFTCTRL`, `KEY_RIGHTCTRL`, `KEY_LEFTSHIFT`, `KEY_RIGHTSHIFT`, `KEY_LEFTALT`, `KEY_RIGHTALT`, `KEY_LEFTMETA`, `KEY_RIGHTMETA`
- **Function Keys:** `KEY_F1` through `KEY_F12`
- **Media Keys:** `KEY_MUTE`, `KEY_VOLUMEUP`, `KEY_VOLUMEDOWN`, `KEY_PLAYPAUSE`, `KEY_NEXTSONG`, `KEY_PREVIOUSSONG`

Feel free to add more in src/parser.rs

## Building

### Prerequisites

- Rust / Cargo toolchain
- Read/write access to `/dev/input/` and `/dev/uinput`.

To run `keymapd` without root privileges, make sure your user belongs to the `input` group:

```bash
sudo usermod -aG input $USER
```

*(You will need to log out and log back in for group changes to take effect.)*

### Build Steps

```bash
cargo build --release
```

The binary will be compiled to `target/release/keymapd`.

## Setting Up as a systemd Service

You can set up `keymapd` to run automatically as a systemd service.

### 1. Copy the Binary

Copy the release binary to `/usr/local/bin`:

```bash
sudo cp target/release/keymapd /usr/local/bin/
```

### 2. Create the Service File

#### Option A: System Service (`/etc/systemd/system/keymapd.service`)

Create `/etc/systemd/system/keymapd.service`:

```ini
[Unit]
Description=Keymap Daemon
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/keymapd
Restart=always
RestartSec=3
User=your_username

[Install]
WantedBy=multi-user.target
```

*Note: `User=your_username` ensures systemd runs the daemon as your user so it automatically reads your `~/.config/keymapd/keymapd.conf`.*

Then enable and start it:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now keymapd
```

---

#### Option B: User Service (`~/.config/systemd/user/keymapd.service`)

If your user account already has permission to read `/dev/input/` and `/dev/uinput` (e.g. via `input` / `uinput` groups), you can set it up as a systemd user service without needing `sudo` to manage it.

Create `~/.config/systemd/user/keymapd.service`:

```ini
[Unit]
Description=Keymap Daemon

[Service]
Type=simple
ExecStart=/usr/local/bin/keymapd
Restart=always
RestartSec=3

[Install]
WantedBy=default.target
```

Then enable and start it with the `--user` flag:

```bash
systemctl --user daemon-reload
systemctl --user enable --now keymapd
```

### Managing the daemon

- **User service status:**
  ```bash
  systemctl --user status keymapd
  ```

- **Start and stop the daemon**
  ```bash
  systemctl --user start keymapd
  systemctl --user stop keymapd
  ```
  

## License

MIT
