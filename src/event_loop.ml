open Base
open Timer_types

type t = {
  timer : Timer_types.timer;
  display : Display.t;
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

let array_replace arr i value =
  let copy = Array.copy arr in
  copy.(i) <- value;
  copy

let handle_key flitter (t, key_str) =
  let timer = flitter.timer in
  let new_timer = match flitter.timer.state with
    | Idle -> (
        match key_str with
        | "space" | "j" -> {
            timer with
            state = Timing;
            start_time = t;
            curr_split = 0;
          }
        | "q" -> raise Stdlib.Exit
        | _ -> timer
      )

    | Timing -> (
        match key_str with
        | "space" | "j" -> {
            timer with

            state = 
              if timer.curr_split = (Array.length timer.split_names) - 1
              then Done else Timing;

            splits = (
              let split_time = Duration.between timer.start_time t in
              array_replace timer.splits timer.curr_split (Some split_time)
            );

            curr_split = timer.curr_split + 1;
          }
        | "k" -> {
            timer with
            state = if timer.curr_split = 0 then Idle else Timing;
            curr_split = timer.curr_split - 1;
          }
        | "backspace" | "delete" -> {timer with state = Paused t}
        | "d" -> {
            timer with
            splits =
              if timer.curr_split > 0 then
                array_replace timer.splits (timer.curr_split - 1) None
              else
                timer.splits;
          }
        | _ -> timer
      )

    | Paused pause_t -> (
        match key_str with
        | "space" | "j" -> {
            timer with 
            start_time = timer.start_time +. t -. pause_t;
            state = Timing;
          }

        (* TODO save golds on backspace, but not delete *)
        | "backspace" | "delete" -> {timer with state = Idle}
        | _ -> timer
      )

    | Done -> (
        match key_str with
        (* TODO save golds on backspace, but not delete *)
        | "backspace" | "delete" | "space" -> {timer with state = Idle}
        | "k" -> {
            timer with
            curr_split = timer.curr_split - 1;
            state = Timing;
          }
        | "q" -> raise Stdlib.Exit;
        | _ -> timer
      )
  in
  {flitter with timer = new_timer}

let handle_draw flitter =
  let draw_time = Unix.gettimeofday () in
  let%lwt () = Display.draw flitter.display flitter.timer in
  Lwt.return {flitter with last_draw = draw_time}

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

let make timer =
  let%lwt hotkeys_stream = Hotkeys.make_stream () in
  Lwt.return {
    timer = timer;
    display = Display.make ();
    (* Make sure we're due for a draw *)
    last_draw = Unix.gettimeofday () -. (1. /. draw_rate);
    hotkeys_stream = hotkeys_stream;
  }

let rec loop flitter =
  let%lwt events = Lwt.npick [(draw_event flitter); (keyboard_event flitter)] in
  let%lwt new_flitter = handle_events flitter events in
  loop new_flitter
