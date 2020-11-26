#!/usr/bin/env python3

from asyncio import sleep, gather, run, set_event_loop, new_event_loop
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

# load a keymap, if one provided
if len(argv) > 1:
  try:
    with open(argv[1]) as file:
      temp = json.load( file )
      if type(temp) is dict:
         keymap_pynput = temp
  # otherwise, ignore it
  except:
    pass

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

keymap = keymap_pynput

async def listen_keys( mapping ):
  """Handle pynput's keyboard output."""
  async for event in async_wrapper( keyboard ):
    if (type(event) == keyboard.Events.Press):
      if (type(event.key) == keyboard.Key) and (event.key.name in mapping):
        print( f'{time()}: {event.key.name} => {mapping[event.key.name]}', flush=True )
      elif (type(event.key) == keyboard._xorg.KeyCode) and (event.key.char in mapping):
        print( f'{time()}: {event.key.char} => {mapping[event.key.char]}', flush=True )
      else:
        print( f'{time()}: {event} => ignored.', flush=True )
    else:
      print( f'{time()}: {event} => ignored.', flush=True )

async def listen_mouse( mapping ):
  """Handle pynput's mouse output."""
  async for event in async_wrapper( mouse ):
    if (type(event) == mouse.Events.Click) and event.pressed:
      if (type(event.button) == mouse.Button):
        key = f'mouse:{event.button.name}'
        if key in mapping:
          print( f'{time()}: {key} => {mapping[key]}', flush=True )
        else:
          print( f'{time()}: {key} => ignored.', flush=True )
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
      elif key is not None:
        print( f'{time()}: {key} => ignored.', flush=True )

listeners = [listen_mouse( keymap ), listen_keys( keymap )]

try:
  run( main(listeners) )
except:
  # the event loop is invalid on Control-C, replace it
  set_event_loop(new_event_loop())
  exit(0)
