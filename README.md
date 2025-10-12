# Flitter

A LiveSplit-inspired speedrunning split timer for the terminal.

![Animated demo GIF](/doc/flitter.gif)

## Features

- Configurable global hotkeys
- :rainbow: Animated rainbow best splits
- Undo and delete split
- Splits stored in single human-editable file
- 24-bit terminal color
- 60 FPS rendering with low CPU usage

## Install

Flitter is designed to work on macOS and Linux (X11). Windows is not currently supported.

Head over to the [releases](https://github.com/alexozer/flitter/releases) page for precompiled binaries and an installation one-liner.

Alternatively, to quickly build and install `flitter` from source, [install Rust](https://www.rust-lang.org/tools/install) and then run:

```bash
cargo install --git https://github.com/alexozer/flitter.git
```

## Global Hotkeys Setup

### macOS

On recent versions of macOS, on first launch you will be prompted to enable accessibility permissions for your terminal. This is required for Flitter to read global hotkeys when the terminal is not focused. Go to System Settings -> Privacy & Security -> Accessibility and enable the toggle for your terminal.

### Linux

Install the X11 development libraries.

Debian/Ubuntu:

```bash
sudo apt install libx11-dev
```

Fedora/RHEL/CentOS:

```bash
sudo dnf install xorg-x11-server-devel
```

## Usage

Create your splits:

1. Copy the template file [`examples/splits_minimal.json`](/examples/splits_minimal.json) somewhere.
2. Edit the file (`title`, `category`, `split_names`) to represent your current run.
3. (Optional) to insert existing `golds` and `personal_best` manually , see [`examples/splits.json`](/examples/splits.json). Missing/skipped personal best segments and golds are represented by `null`.

Launch Flitter with your splits file:

```bash
$ flitter path/to/my-splits.json
```

**Warning:** Don't edit your splits file while Flitter is running, your changes will be overwritten.

### Keybindings

Keybindings are all global hotkeys; they will work even when the terminal is not focused. The following table is the default keybindings:

| Keys        | Action                                  |
| ----------- | --------------------------------------- |
| `Space`     | Split                                   |
| `PageUp`    | Undo split                              |
| `End`       | Delete split                            |
| `P`         | Toggle Pause                            |
| `Backspace` | Reset and save PB + best segments       |
| `Delete`    | Reset and discard PB + best segments    |
| `Q`         | Quit (not a global hotkey)              |

To change them, create `$HOME/.config/flitter-timer/config.json` and populate it with [the example config](/examples/default-config.json). See the [full list of keys](/doc/keys.txt) for which key names you can use.

## Contributing

Feel free to make an issue or a pull request!
