let usage = 
  "Usage:\n" ^
  "\n" ^
  "flitter -d\n" ^
  "Open demo splits.\n" ^
  "\n" ^
  "flitter -o <splits_path>\n" ^
  "Open the splits file pointed to by `splits_path`.\n"

let run_event_loop timer =
  let%lwt event_loop = Event_loop.make timer in
  Event_loop.loop event_loop

let run () =
  match Sys.argv with
  | [|_; "-o"; path|] ->
    let timer = Loadsave.load path in
    Lwt_main.run (run_event_loop timer)

  | [|_; "-d"|] ->
    Lwt_main.run (run_event_loop Demo.timer)

  | _ -> print_string usage