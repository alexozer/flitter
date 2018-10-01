open Base
open Timer_types

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
  last_draw : float option;
}

type event = Draw_tick | Key of Hotkeys.keypress

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
    last_draw = None;
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

let array_replace arr i value =
  let copy = Array.copy arr in
  copy.(i) <- value;
  copy

let handle_draw flitter =
  let t = Unix.gettimeofday () in
  let%lwt () = Display.draw flitter.display (to_speedrun flitter) in
  Lwt.return {flitter with last_draw = Some t}

let handle_key flitter (t, key_str) =
  let run = to_speedrun flitter in

  match flitter.state with
  | Idle -> (
      match key_str with
      | "space" -> {
          flitter with
          state = Timing;
          start_time = t;
          curr_split = 0;
        }
      | "q" -> raise Stdlib.Exit
      | _ -> flitter
    )

  | Timing -> (
      match key_str with
      | "space" | "j" -> {
          flitter with

          state = 
            if flitter.curr_split = (Array.length flitter.game.split_names) - 1
            then Done else Timing;

          splits = array_replace flitter.splits flitter.curr_split 
              (Splits.segment_time run ~now:t flitter.curr_split);

          curr_split = flitter.curr_split + 1;
        }
      | "backspace" | "delete" -> {flitter with state = Paused t}
      | "d" -> {
          flitter with
          splits =
            if flitter.curr_split > 0 then
              array_replace flitter.splits (flitter.curr_split - 1) None
            else
              flitter.splits;
        }
      | _ -> flitter
    )

  | Paused pause_t -> (
      match key_str with
      | "space" -> {
          flitter with 
          start_time = pause_t -. flitter.start_time;
          state = Timing;
        }

      (* TODO save golds on backspace, but not delete *)
      | "backspace" | "delete" -> {flitter with state = Idle}
      | _ -> flitter
    )

  | Done -> (
      match key_str with
      (* TODO save golds on backspace, but not delete *)
      | "backspace" | "delete" | "space" -> {flitter with state = Idle}
      | "k" -> {
          flitter with
          curr_split = flitter.curr_split - 1;
          state = Timing;
        }
      | _ -> flitter
    )

let draw_event flitter =
  let period = 1. /. 60. in

  match flitter.last_draw with
  | None -> Lwt.return Draw_tick
  | Some t ->
    let deadline = t +. period in
    let delay = deadline -. Unix.gettimeofday () in
    if Float.(delay > 0.) then
      let%lwt () = Lwt_unix.sleep delay in
      Lwt.return Draw_tick
    else
      Lwt.return Draw_tick

let keyboard_event flitter =
  match%lwt Lwt_stream.get flitter.hotkeys_stream with
  | Some keypress -> Lwt.return (Key keypress)
  | None -> failwith "Hotkeys stream exited unexpectedly"

let rec handle_events flitter events =
  match events with
  | evt :: remaining_evts -> (
      let%lwt new_flitter = match evt with
        | Draw_tick -> handle_draw flitter
        | Key keypress -> Lwt.return (handle_key flitter keypress)
      in

      handle_events new_flitter remaining_evts
    )

  | [] -> Lwt.return flitter

let rec loop flitter =
  let%lwt events = Lwt.npick [(draw_event flitter); (keyboard_event flitter)] in
  let%lwt new_flitter = handle_events flitter events in
  loop new_flitter

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