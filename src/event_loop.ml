open Core_kernel
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
            state = Timing ([||], t)
          }
        | "q" -> raise Stdlib.Exit
        | _ -> timer
      )

    | Timing (splits, start_time) -> (
        let curr_split = Array.length splits in
        match key_str with
        | "space" | "j" -> 
          let curr_split_time = Some (Duration.between start_time t) in
          let new_splits = Array.append splits [|curr_split_time|] in

          let new_state = 
            if Array.length new_splits = Array.length timer.split_names
            then Done (new_splits, start_time)
            else Timing (new_splits, start_time)
          in
          {timer with state = new_state}

        | "k" -> 
          let new_state =
            if curr_split = 0 then Idle 
            else Timing ((Array.slice splits 0 (curr_split - 1)), start_time)
          in
          {timer with state = new_state}

        | "backspace" | "delete" -> {timer with state = Paused (splits, start_time, t)}

        | "d" -> 
          let new_state =
            if curr_split > 0 then
              let new_splits = array_replace splits (curr_split - 1) None in
              Timing (new_splits, start_time)
            else
              Idle
          in
          {timer with state = new_state}

        | _ -> timer
      )

    | Paused (splits, start_time, pause_time) -> (
        match key_str with
        | "space" | "j" -> 
          let new_state = Timing (splits, start_time +. t -. pause_time) in
          {timer with state = new_state}

        (* TODO save golds on backspace, but not delete *)
        | "backspace" | "delete" -> {timer with state = Idle}
        | _ -> timer
      )

    | Done (splits, start_time) -> (
        match key_str with
        (* TODO save golds on backspace, but not delete *)
        | "backspace" | "delete" | "space" -> {timer with state = Idle}

        | "k" -> 
          let new_splits = Array.(slice splits 0 (length splits - 1)) in
          {timer with state = Timing (new_splits, start_time)}

        | "q" -> raise Stdlib.Exit
        | _ -> timer
      )
  in
  {flitter with timer = new_timer}

let handle_draw flitter =
  let draw_time = Unix.gettimeofday () in
  Display.draw flitter.display flitter.timer;
  {flitter with last_draw = draw_time}

let rec handle_events flitter events =
  match events with
  | evt :: remaining_evts -> (
      let new_flitter = match evt with
        | Draw_tick -> handle_draw flitter
        | Key keypress -> handle_key flitter keypress
      in

      handle_events new_flitter remaining_evts
    )

  | [] -> flitter

let make timer =
  let%lwt hotkeys_stream = Hotkeys.make_stream () in
  Lwt.return {
    timer = timer;
    display = Display.make ();
    (* Make sure we're due for a draw *)
    last_draw = Unix.gettimeofday () -. (1. /. draw_rate);
    hotkeys_stream = hotkeys_stream;
  }

let loop flitter =
  let rec loop' flitter =
    let%lwt events = Lwt.npick [(draw_event flitter); (keyboard_event flitter)] in
    let new_flitter = handle_events flitter events in
    loop' new_flitter
  in

  Lwt.catch (fun () -> loop' flitter)
    (function 
      | Stdlib.Exit -> Display.close flitter.display; Lwt.return ()
      | exn -> raise exn
    )
