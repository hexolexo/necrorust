# Necrorust
Necrorust is a simple (albeit poorly written) tui application in rust that is designed to simplify the mage hand press's [Necromancer Class](https://magehandpress.com/2022/09/necromancer-base-class.html) specifically the Thrall mechanic

## Features
- Minimal TUI interface
- Configurations via JSON files
- Cross platform compatibility

## Install 
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone repository
3. Copy `config/necrorust` to your systems config directory

**Linux/MacOS:** `~/.config/necrorust`

**Windows:** `C:\Users\[Username]\AppData` or `%APPDATA%`

5. Use cargo to install to your path
```bash
cargo install --path .
```
Now you can run necrorust from the terminal anywhere

5. (ALTERNATIVE) if that doesn't work you can just use `cargo run`

## Configuration
1. Find your systems config directory
2. Copy a template and give it a unique name
3. Change the stats of the undead in the file

Example config file

```json
{
  "hp": 22,
  "ac": 8,
  "cr": 1,
  "name": "Zombie",
  "hit": 3,
  "damage": [1, 6, 1],
  "hit_dice": [3, 8, 9]
} 
```


