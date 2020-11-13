# Flitter

A Livesplit-inspired speedrunning split timer for terminal / command-line.

![Animated demo GIF](/doc/demo.gif)

[All screenshots](/doc/)

## Features

- Global hotkeys
- :rainbow: Animated rainbow best splits
- 24-bit terminal color
- Undo / delete split
- Pause / resume
- Splits and history stored in single human-editable file
- 60 FPS rendering with low CPU usage
- Robust time computations: float math is mostly avoided

## Install

Flitter has been tested on Linux, but in theory it should work on MacOS as well. Windows is not supported.

### OCaml Dependencies

Flitter is mostly written in OCaml.

Install opam: [opam install instructions](https://opam.ocaml.org/doc/Install.html)

Set up opam and install OCaml dependencies:

```bash
$ opam init
$ opam switch create 4.07.0
$ opam install dune core lwt re color sexp_pretty uutf lwt_ppx uuseg notty
```

### Python Dependencies

Flitter uses a tiny amount of Python to provide global hotkeys.

Install pip for Python 3. For example, on Ubuntu / Debian:

```bash
$ sudo apt install python3-pip
```

Install Python package dependencies:

```bash
$ pip3 install --user pynput
```

### Install Flitter

```bash
$ git clone --recursive https://github.com/alexozer/flitter.git
$ cd flitter
$ dune build
$ dune install
```

## Usage

Create your splits:

Copy `examples/splits.scm` somewhere. Edit it and add your game and split information. The personal best splits, world record splits, and gold segments are not required.

Launch Flitter with your splits file:

```bash
$ flitter my-splits.scm
```

**Warning:** Don't edit your splits file while Flitter is running, your changes will be overwritten.

### Keybindings

Keybindings are all global hotkeys; they will work even when the terminal is not focused. Here are the defaults:

| Keys        | Action                                              |
| ----------- | --------------------------------------------------- |
| `Space`     | Start / split / save and reset if run finished      |
| `J`         | Start / split                                       |
| `K`         | Undo split                                          |
| `D`         | Delete last segment                                 |
| `Backspace` | Pause / reset (save run if finished and save golds) |
| `Delete`    | Pause / delete run (don't save anything)            |
| `Q`         | Quit (if not currently timing)                      |

You can modify these by placing a file named "keymap.json" in the working directory; check the examples for an alternate keymap that uses the number pad. This file also allows you to create a filter so that only buttons from a specific device count. Say, for instance, you're running a game that doesn't use the mouse at all. Binding Flitter to mouse buttons seems like a great idea, until you alt-Tab away to chat. Thanks to this filter, you can now do the following:

1. Plug in a second mouse.
2. Run `egrep '^N:' /proc/bus/input/devices` to find the name of this new device. Let's say it's "Logitech Wireless Mouse PID:1024".
3. Put the following in "keymap.json":

```
{
  "BTN_MIDDLE":"start-stop-reset",
  "BTN_LEFT":  "pause-reset",
  "BTN_RIGHT": "undo",
  "device":    "Logitech Wireless Mouse"
}
```

4. Securely tape the mouse to the floor in front of your chair.

This is a poor person's foot pedal: you control Flitter by tapping the appropriate button with your big toe, yet Flitter won't activate when you alt-Tab away from the game and start clicking around with your main mouse. Note that Flitter only checks if the target string is present in the device name. This allows you to be lazy ("Logitech" would also work if I only had one Logitech device) or creative (any and all devices with a specific brand name will control Flitter).

## Contributing

Feel free to make an issue or a pull request!
