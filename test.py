#!/usr/bin/env python3

import time
from pynput import keyboard

def on_press(key):
    t = time.time()
    try:
        # Alphanumeric key pressed
        print('{} {}'.format(t, key.char), flush=True)
    except AttributeError:
        # Special key pressed
        key_name = str(key)[4:] # Strip "Key."
        print('{} {}'.format(t, key_name), flush=True)

# Collect events until released
with keyboard.Listener(on_press=on_press) as listener:
    listener.join()
