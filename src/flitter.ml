open Base
open Splits

type t = {
  game : game_info;
  history : Duration.t array list;
  pb : Duration.t array option;

  start_time : float;
  state : timer_state;
  splits : Duration.t option array;
  curr_split : int;

  display : Display.t;
  hotkeys_stream : Hotkeys.t;
}

(* A crutch for quick testing *)
let of_speedrun run =
  let history = match run.comparison with
    | Some comp -> [comp]
    | None -> []
  in 
  let%lwt hotkeys_stream = Hotkeys.make_stream () in
  Lwt.return {
    game = run.game;
    history = history;
    pb = run.comparison;

    start_time = run.start_time;
    state = run.state;
    splits = run.splits;
    curr_split = run.curr_split;

    display = Display.make ();
    hotkeys_stream = hotkeys_stream;
  }

let to_speedrun flitter =
  {
    game = flitter.game;
    comparison = flitter.pb;

    start_time = flitter.start_time;
    state = flitter.state;
    splits = flitter.splits;
    curr_split = flitter.curr_split;
  }

let loop flitter =
  let period = 1. /. 60. in

  let rec refresh () =
    let deadline = Unix.gettimeofday () +. period in
    let%lwt () = Display.draw flitter.display (to_speedrun flitter) in
    let t = Unix.gettimeofday () in
    let%lwt () =
      if Float.(t < deadline)
      then Lwt_unix.sleep (deadline -. t) 
      else Lwt.return ()
    in
    refresh ()
  in
  refresh ()

let test () =
  let run = {
    game = {
      title = "Super Monkey Ball 2: Monkeyed Ball";
      category = "Story Mode All Levels";
      attempts = 3000;
      completed_runs = 40;

      split_names = [|
        "Green";
        "Apricot";
        "Blue";
      |];

      golds = Some [|
          2000;
          2000;
          2000;
        |];
    };

    comparison = Some [|
        3000;
        5000;
        8000;
      |];

    start_time = Unix.gettimeofday ();
    state = Timing;
    splits = Array.of_list [Some 1500; None; None];
    curr_split = 2;
  } in

  let%lwt flitter = of_speedrun run in
  loop flitter

let run () =
  Lwt_main.run (test ())

(* let show_hotkeys () =
   let%lwt stream = Hotkeys.make_stream () in
   let rec show () =
    match%lwt Lwt_stream.get stream with
    | Some (time, keypress) ->
      let%lwt () = Lwt_io.printl @@ "Got " ^ keypress ^ " at time " ^ Float.to_string time in
      show ()
    | None -> Lwt.return ()
   in
   show () *)