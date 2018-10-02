open Base
open Timer_types

type t = {
  timer : Timer.t;
  last_draw : float;
  hotkeys_stream : Hotkeys.t;
}

type event = Draw_tick | Key of Hotkeys.keypress

let draw_rate = 60.

let draw_event flitter =
  let period = 1. /. draw_rate in

  let deadline = flitter.last_draw +. period in
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
        | Draw_tick -> 
          let draw_time = Unix.gettimeofday () in
          let%lwt () = Timer.handle_draw flitter.timer in
          Lwt.return {flitter with last_draw = draw_time}

        | Key keypress -> 
          Lwt.return {flitter with timer = Timer.handle_key flitter.timer keypress}
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

  let%lwt hotkeys_stream = Hotkeys.make_stream () in

  let flitter = {
    timer = Timer.of_speedrun run;
    last_draw = Unix.gettimeofday () -. 1. /. draw_rate;
    hotkeys_stream = hotkeys_stream;
  }
  in

  loop flitter

let run () =
  Lwt_main.run (test ())
