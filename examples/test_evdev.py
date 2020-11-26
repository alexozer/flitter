#!/usr/bin/env python3

from asyncio import sleep, gather, run, set_event_loop, new_event_loop
from evdev import InputDevice, ecodes, list_devices, categorize
import json
from sys import exit, argv
from time import time

async def heartbeat():
  """Periodically print a heartbeat."""
  while True:
    await sleep(1)
    print( f"{time()} heartbeat", flush=True )

async def main( listeners ):
  """Launch the main loop."""
  await gather( *listeners, heartbeat() )


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

# load a keymap, if one provided
if len(argv) > 1:
  try:
    with open(argv[1]) as file:
      temp = json.load( file )
      if type(temp) is dict:
         keymap_evdev = temp
  # otherwise, assume it's a filter
  except:
      keymap_evdev['device'] = argv[1]
  

keymap = {ecodes.ecodes[key]:keymap_evdev[key] for key in keymap_evdev if key in ecodes.ecodes}

devices = [InputDevice(path) for path in list_devices()]
if 'device' in keymap_evdev:
  devices = [dev for dev in devices if keymap_evdev['device'] in dev.name]

async def listen(device, mapping):
  """Handle press events from the device, with the given key mapping."""
  async for event in device.async_read_loop():
    if (event.value == 1) and (event.code in mapping):
      print( f"{event.sec}.{event.usec}: {categorize(event)} => {mapping[event.code]}", flush=True )
    else:
      print( f"{event.sec}.{event.usec}: {categorize(event)} => ignored.", flush=True )

listeners = [listen(dev, keymap) for dev in devices]

try:
  run( main(listeners) )
except:
  # the event loop is invalid on Control-C, replace it
  set_event_loop(new_event_loop())
  exit(0)
