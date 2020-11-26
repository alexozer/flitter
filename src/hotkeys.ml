open Core_kernel

type keypress = float * string
type t = keypress Lwt_stream.t

let python_detect_keys = {|
from asyncio import sleep, gather, run, set_event_loop, new_event_loop
from importlib.util import find_spec
import json
from sys import exit
from time import time

async def heartbeat():
  """Periodically print a heartbeat."""
  while True:
    await sleep(1)
    print( f"{time()} heartbeat", flush=True )

async def main( listeners ):
  """Launch the main loop."""
  await gather( *listeners, heartbeat() )


keymap_pynput = {
  "space":    "start-split-reset",
  "j":        "start-split",
  "k":        "undo",
  "d":        "delete-last",
  "backspace":"pause-reset",
  "delete":   "pause-delete",
  "q":        "quit"
# "mouse:right":"start-split-reset"
         }
keymap_evdev = {
  "KEY_SPACE":    "start-split-reset",
  "KEY_J":        "start-split",
  "KEY_K":        "undo",
  "KEY_D":        "delete-last",
  "KEY_BACKSPACE":"pause-reset",
  "KEY_DELETE":   "pause-delete",
  "KEY_Q":        "quit"
# "device":       "Secondary Device Name"
         }

# attempt to load keymaps
keys = [None, None]
for i,v in enumerate(['evdev','pynput']):
  try:
    with open(f'keymap_{v}.json') as file:
      keys[i] = json.load( file )
  except:
    keys[i] = None

# which library?
if (find_spec('evdev') != None) and ((type(keys[0]) is dict) or (keys[1] is None)):

  from evdev import InputDevice, ecodes, list_devices

  def translate( mapping ):
    retVal = {ecodes.ecodes[key]:mapping[key] for key in mapping if key in ecodes.ecodes}
    if 'device' in mapping:
      retVal['device'] = mapping['device']
    return retVal

  if type(keys[0]) is dict:
    keymap = translate( keys[0] )
  else:
    keymap = translate( keymap_evdev )

  devices = [InputDevice(path) for path in list_devices()]
  if 'device' in keymap:
    devices = [dev for dev in devices if keymap['device'] in dev.name]

  async def listen(device, mapping):
    """Handle press events from the device, with the given key mapping."""
    async for event in device.async_read_loop():
      if (event.value == 1) and (event.code in mapping):
        print( f"{event.sec}.{event.usec} {mapping[event.code]}", flush=True )

  listeners = [listen(dev, keymap) for dev in devices]

else:

  from pynput import keyboard,mouse

  async def async_wrapper( object ):
    """Make pynput's classes compatible with asyncio."""
    with object.Events() as events:
      while True:
        event = events.get(0.005)
        if event is None:
          await sleep(0.005)
        else:
          yield event
  if type(keys[1]) is dict:
    keymap = keys[1]
  else:
    keymap = keymap_pynput

  async def listen_keys( mapping ):
    """Handle pynput's keyboard output."""

    async for event in async_wrapper( keyboard ):
      if (type(event) == keyboard.Events.Press):
        if (type(event.key) == keyboard.Key) and (event.key.name in mapping):
          print( f'{time()} {mapping[event.key.name]}', flush=True )
        elif (type(event.key) == keyboard._xorg.KeyCode) and (event.key.char in mapping):
          print( f'{time()} {mapping[event.key.char]}', flush=True )

 async def listen_mouse( mapping ):
    """Handle pynput's mouse output."""

    async for event in async_wrapper( mouse ):
      if (type(event) == mouse.Events.Click) and event.pressed:
        if (type(event.button) == mouse.Button):
          key = f'mouse:{event.button.name}'
          if key in mapping:
            print( f'{time()} {mapping[key]}', flush=True )
      elif (type(event) == mouse.Events.Scroll):
        key = None
        if event.dy > 0:
          key = "mouse:scroll_wheel_up"
        elif event.dy < 0:
          key = "mouse:scroll_wheel_down"
        elif event.dx > 0:
          key = "mouse:scroll_wheel_left"
        elif event.dx < 0:
          key = "mouse:scroll_wheel_right"
        if key in mapping:
          print( f'{time()} {mapping[key]}', flush=True )

  listeners = [listen_mouse( keymap ), listen_keys( keymap )]

try:
  run( main(listeners) )
except:
  # the event loop is invalid on Control-C, replace it
  set_event_loop(new_event_loop())
  exit(0)
|}

let stream_of_python python_src =
  let cmd = "", [|"python3"; "-"|] in
  let pipe_out_fd, pipe_out_fd_unix = Lwt_unix.pipe_out () in
  let () = Lwt_unix.set_close_on_exec pipe_out_fd_unix in
  let redir = `FD_move pipe_out_fd in

  let py_stream = Lwt_process.pread_lines ~stdin:redir cmd in

  let%lwt n = Lwt_unix.write_string pipe_out_fd_unix python_src 0 (String.length python_src) in
  if n < String.length python_src then failwith "Failed to write python to pipe" 
  else 
    let%lwt () = Lwt_unix.close pipe_out_fd_unix in
    Lwt.return py_stream

let make_stream () =
  let%lwt str_stream = stream_of_python python_detect_keys in
  let stream = Lwt_stream.from (fun () ->
      match%lwt Lwt_stream.get str_stream with
      | Some str -> (
          match String.split str ~on:' ' with
          | time_str :: key_str :: [] -> 
            Lwt.return (Some ((Float.of_string time_str), key_str))

          | _ -> failwith "Invalid output from Python keypress server"

        )
      | None -> Lwt.return None
    )
  in

  Lwt.return stream
