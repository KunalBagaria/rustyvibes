# Rustyvibes

A Rust CLI that makes mechanical keyboard sound effects on every key press

## Installation
OG
```bash
cargo install rustyvibes
```
My fix
```bash
cargo install --git https://github.com/insasquatchcountry/rustyvibes.git --branch feature/silent-mode-default
```
## Linux

You will need to install Advanced Linux Sound Architecture [ALSA]

**Ubuntu / Debian**

```
sudo apt-get install alsa-tools
```

**Fedora**

```
sudo dnf install alsa-lib-devel
```

## Usage

```
rustyvibes <soundpack_path> [-v <volume>] [-d]
```

- `<soundpack_path>`: Path to the soundpack directory.
- `-v <volume>`: Volume level (0-100, default: 100).
- `-d, --debug`: Enable debug mode to show console output (e.g., errors, unmapped keys). By default, only the ASCII art is displayed.

### Mechvibes vs. Rustyvibes

How does Rustyvibes compare to its competitors like Mechvibes? Mechvibes uses Electron and Chromium which is very resource intensive. Rustyvibes on the other hand is made with Rust and can be up to 10x-100x more resource efficient.

**Mechvibes Soundpacks**: [Here](https://github.com/hainguyents13/mechvibes)

Certain custom soundpacks may not work with Rustyvibes, you can use [this tool](https://github.com/hainguyents13/mechvibes) to fix those.

### Privacy and Permissions

Rustyvibes is a fully open-sourced project and never uses any network activity at all. macOS by default will ask you for input monitoring permissions when you start the app for the first time. If you were unable to enable it the first time, you'll need to add your default terminal you're using in the allowed input monitoring apps.

![image](https://github.com/user-attachments/assets/4e6b3a2e-ffd9-4c0a-975c-2f8a3b5e7c3b)

## Contribute to this project

[Buy Me a Coffee](https://www.buymeacoffee.com/kb24x7)
