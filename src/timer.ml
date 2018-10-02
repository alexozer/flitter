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

let handle_key flitter (t, key_str) =
  match flitter.state with
  | Idle -> (
      match key_str with
      | "space" | "j" -> {
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

          splits = (
            let split_time = Duration.between flitter.start_time t in
            array_replace flitter.splits flitter.curr_split (Some split_time)
          );

          curr_split = flitter.curr_split + 1;
        }
      | "k" -> {
          flitter with
          state = if flitter.curr_split = 0 then Idle else Timing;
          curr_split = flitter.curr_split - 1;
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
      | "space" | "j" -> {
          flitter with 
          start_time = flitter.start_time +. t -. pause_t;
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
      | "q" -> raise Stdlib.Exit;
      | _ -> flitter
    )

let handle_draw flitter =
  Display.draw flitter.display (to_speedrun flitter)