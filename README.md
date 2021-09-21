```
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
```                                                                               


A Rust CLI to play mechanical keyboard sounds globally


# Installation

### macOS:

```
brew tap kb24x7/rustyvibes && brew install rustyvibes
```

### Windows / Linux:

```
To be updated
```

# Usage

```
rustyvibes <soundpack_path>
```

### Download Soundpacks: [Here](https://docs.google.com/spreadsheets/d/1PimUN_Qn3CWqfn-93YdVW8OWy8nzpz3w3me41S8S494/edit)

---

### Mechvibes vs. Rustyvibes

Mechvibes uses Electron and is shipped with Chromium to enable multi-platform support. While this may sound good, Chromium is very resource intensive and uses a lot of CPU and Memory.

![Mechvibes CPU Usage](https://i.imgur.com/MwplMkW.png)

Rustyvibes on the other hand, is a CLI tool made with Rust, which is a really efficient language and can be compiled to work across all the platforms without having to ship an entire broswer with it. It uses almost 10x lesser CPU and Memory power than Mechvibes.


(Insert Rustyvibes CPU Usage Screenshot here)

