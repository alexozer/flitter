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

You have two choices of Python libraries to rely on.

* `evdev` is a Linux-only library that works with raw keycodes, so (for instance) it can tell the difference between the main "Enter" key and the keypad one. It is independent of the GUI, and allows for device filtering so that only keypresses on certain hardware will be accepted. Besides being Linux-only, it also requires low-level access to `/dev/input/`, either by adding the current user to the appropriate group or running as root.
* `pynput` is a cross-platform library that works after keycodes are translated to key presses, so (for instance) "q" and "Q" are treated as two separate keys; this also means CapsLock or NumLock can prevent your keys from registering with Flitter. It works out-of-the-box with no system tweaking, unlike `evdev`. On Linux it captures global hotkeys by relying on Xlib at the moment, which means it fails to capture anything if you're using Wayland instead.

To install either of these libraries, run the appropriate line:

```bash
$ pip3 install --user pynput
$ pip3 install --user evdev
```

If both libraries are installed, `evdev` is preferred. See the section on key bindings for a way to override that default without uninstalling `evdev`.

As mentioned above, for `evdev` you have two choices. The simplest of the two is to run Flitter as root via `sudo`, which creates the obvious security issues. Alternatively, you can add yourself to the group which has ownership over `/dev/input`; this is usually "input", but double-check the permissions under `/dev/input` to confirm that. Assuming it is "input", then executing

```bash
$ sudo usermod -G input -a $USER
```

and logging out then back in will do the trick. This *also* creates a security problem, as now any program executed with your uid could read every key you press (and if the default permissions allow writing, they could also inject keypresses and mouse movements). There is no solution involving `evdev` that doesn't create one of these two security holes, as far as we're aware. Incidentally, Xlib has the same security issues.

### Install Flitter

First, clone the repository:

```bash
$ git clone --recursive https://github.com/alexozer/flitter.git
$ cd flitter
```

In the "examples" directory you'll find `test_evdev.py` and `test_pynput.py`. These run independently of Flitter, and allow you to test that you've got the appropriate Python library installed properly. They are also useful for developing your own key bindings, discussed later.

To build and install Flitter,

```bash
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

### Key Bindings

Here is the default key binding for Flitter:


| Keys        | Action                                              |
| ----------- | --------------------------------------------------- |
| `space`     | Start / split / save and reset if run finished      |
| `j`         | Start / split                                       |
| `k`         | Undo split                                          |
| `d`         | Delete last segment                                 |
| `backspace` | Pause / reset (save run if finished and save golds) |
| `delete`    | Pause / delete run (don't save anything)            |
| `q`         | Quit (if not currently timing)                      |

You can substitute another key binding by placing a JSON file with a specific name in the working directory. If using `evdev`, this file should be called `keymap_evdev.json`, whereas with `pynput` use `keymap_pynput.json`. The different names allows Linux users to pick and choose the library they wish to use (the presence of the latter JSON file and the absence of the former will force the use of `pynput`, even if `evdev` is installed). The `examples` directory contains alternate configurations for both libraries.

That directory also contains `test_evdev.py` and `test_pynput.py`, which allow you to verify both libraries are correctly installed. They are also useful for developing and testing key bindings. Run one of

```bash
python3 examples/test_pynput.py [KEYMAP FILE]
python3 examples/test_evdev.py [KEYMAP FILE]
```

and both programs will attempt to load your keymap, helping you verify if the appropriate events are being fired. `test_evdev.py` has the same issues with permissions as mentioned earlier, so either run it with `sudo` or grant the current user permission to read `/dev/input`.

`keymap_evdev.json` also gives you the ability to create a filter so that only buttons from a specific device count. Say, for instance, you're running a game that doesn't use the mouse at all. Binding Flitter to mouse buttons seems like a great idea, until you alt-Tab away to chat. Thanks to this filter, you can now do the following:

1. Plug in a second mouse.
2. Run `egrep '^N:' /proc/bus/input/devices` or `test_evdev.py` to find the name of this new device. Let's say it's "Logitech Wireless Mouse PID:1024".
3. Put the following in `keymap_evdev.json`:

```
{
  "BTN_MIDDLE":"start-stop-reset",
  "BTN_LEFT":  "pause-reset",
  "BTN_RIGHT": "undo",
  "device":    "Logitech Wireless Mouse"
}
```

4. Securely attach the mouse to the floor in front of your chair.

This is a poor person's foot pedal: you control Flitter by tapping the appropriate button with your big toe, yet Flitter won't activate when you alt-Tab away from the game and start clicking around with your main mouse. Note that Flitter only checks if the target string is present in the device name. This allows you to be lazy ("Logitech" would also work if I only had one Logitech device) or creative (any and all devices with a specific brand name will control Flitter).

`test_evdev.py` will list the name of every device it is listening to on start-up. If it lists none, then either your keymap's device filter is too strict or you do not have the correct permissions. You can test out various filter strings by executing

```bash
python3 examples/test_evdev.py "STRING"
```

where "STRING" is not the name of a valid JSON file.

`pynput` can also read input from all mice. Since it emits "right" for both the right mouse button and the right arrow key, you disambiguate the two by prepending "mouse:" for the mouse inputs. This works in general; use `test_pynput.py` to help determine if the button can be captured and the correct name of each button.

## Contributing

Feel free to make an issue or a pull request!
