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
}

(* A crutch for quick testing *)
let of_speedrun run =
  let history = match run.comparison with
    | Some comp -> [comp]
    | None -> []
  in
  {
    game = run.game;
    history = history;
    pb = run.comparison;

    start_time = run.start_time;
    state = run.state;
    splits = run.splits;
    curr_split = run.curr_split;

    display = Display.make ();
  }

let to_speedrun timer =
  {
    game = timer.game;
    comparison = timer.pb;

    start_time = timer.start_time;
    state = timer.state;
    splits = timer.splits;
    curr_split = timer.curr_split;
  }

let array_replace arr i value =
  let copy = Array.copy arr in
  copy.(i) <- value;
  copy

let handle_key timer (t, key_str) =
  match timer.state with
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
            if timer.curr_split = (Array.length timer.game.split_names) - 1
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

let handle_draw timer =
  Display.draw timer.display (to_speedrun timer)