open Core_kernel

type keypress = float * string
type t = keypress Lwt_stream.t

let python_detect_keys = {|
import time
import sys
from pynput import keyboard


def on_press(key):
    try:
        t = time.time()
        try:
            # Alphanumeric key pressed
            print('{} {}'.format(t, key.char), flush=True)
        except AttributeError:
            # Special key pressed
            key_name = str(key)[4:] # Strip "Key."
            print('{} {}'.format(t, key_name), flush=True)
    except:
        sys.exit(0)

# Collect events until released
with keyboard.Listener(on_press=on_press) as listener:
    try:
        while True:
            t = time.time()
            print('{} heartbeat'.format(t))
            time.sleep(1)
    except:
        sys.exit(0)
    listener.join()
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

let make_stream ?disable_python () =
  match disable_python with
  | Some () -> Lwt_stream.create () |> fst  |> Lwt.return
  | None -> 
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
