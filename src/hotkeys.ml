open Core_kernel

type keypress = float * string
type t = keypress Lwt_stream.t

let python_detect_keys = {|
from asyncio import sleep, gather, run, set_event_loop, new_event_loop
from evdev import InputDevice, ecodes, list_devices
import json
from sys import exit
from time import time

keymap = {"KEY_SPACE":    "start-split-reset",
          "KEY_J":        "start-split",
          "KEY_K":        "undo",
          "KEY_D":        "delete-last",
          "KEY_BACKSPACE":"pause-reset",
          "KEY_DELETE":   "pause-delete",
          "KEY_Q":        "quit"
#         "device":       "Secondary Device Name"
         }
keymap_override = 'keymap.json'

async def listen(device, mapping):
    """Handle press events from the device, with the given key mapping."""
    async for event in device.async_read_loop():
        if (event.value == 1) and (event.code in mapping):
            print( f"{event.sec}.{event.usec} {mapping[event.code]}", flush=True )

async def heartbeat():
    """Periodically print a heartbeat."""
    while True:
        await sleep(1)
        print( f"{time()} heartbeat", flush=True )

async def main( devices, mapping ):
    """Call this to run both the heartbeat and listeners at once."""
    listeners = [listen(dev, mapping) for dev in devices]
    await gather( *listeners, heartbeat() )

def translate_map( mapping ):
    """Convert text labels to evdev codes."""
    return {ecodes.ecodes[key]:mapping[key] for key in mapping if key in ecodes.ecodes}

# load an external map, if possible
try:
    with open( keymap_override, 'rt' ) as file:
        keymap = json.load( file )
except:
    pass

devices = [InputDevice(path) for path in list_devices()]
if 'device' in keymap:
    devices = [dev for dev in devices if keymap['device'] in dev.name]
mapping = translate_map( keymap )

try:
    run( main( devices, mapping ) )
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
