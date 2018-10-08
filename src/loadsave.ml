open Core_kernel

type split = {
  title : string;
  time : string sexp_option;
  is_gold : bool [@default false];
}
[@@deriving sexp]

let split_of_sexp sexp =
  let split = split_of_sexp sexp in
  match split.time with
  | Some str -> (
      if not (Duration.string_valid str) then of_sexp_error "Invalid time" sexp
      else split
    )
  | None -> (
      if split.is_gold then of_sexp_error "Missing time cannot be a gold split" sexp
      else split
    )

type gold = {
  title : string;
  duration : string sexp_option;
}
[@@deriving sexp]

let gold_of_sexp sexp =
  let gold = gold_of_sexp sexp in
  match gold.duration with
  | Some str -> (
      if not (Duration.string_valid str) then of_sexp_error "Invalid duration" sexp
      else gold
    )
  | None -> gold

type archived_run = {
  attempt : int;
  splits : split array;
}
[@@deriving sexp]

type game = {
  title : string;
  category : string;
  attempts : int;
  completed : int;

  split_names : string array;
  golds : gold array sexp_option;
  personal_best : archived_run sexp_option;
  history : archived_run sexp_list;
}
[@@deriving sexp]

let game_of_sexp sexp =
  let game = game_of_sexp sexp in
  let num_splits = Array.length game.split_names in
  if num_splits = 0 then of_sexp_error "No split names defined" sexp
  else
    let pb_ok = match game.personal_best with
      | Some run -> Array.length run.splits = num_splits
      | None -> true
    in
    if not pb_ok 
    then of_sexp_error "Personal best has different number of splits than split_names" sexp
    else
      let history_runs_ok = List.fold game.history ~init:true ~f:(
          fun all_ok run -> all_ok && Array.length run.splits = num_splits
        )
      in
      if not history_runs_ok 
      then of_sexp_error "Not all history runs have same number of splits as split_names" sexp
      else game

let load_golds parsed_game =
  match parsed_game.golds with
  | Some segments ->
    Array.map segments ~f:(fun seg ->
        match seg.duration with
        | Some duration -> (
            match Duration.of_string duration with
            | Some parsed_duration -> {
                Timer_types.title = seg.title;
                duration = Some parsed_duration;
              }
            | None -> assert false
          )

        | None -> {
            Timer_types.title = seg.title;
            duration = None;
          }
      )

  | None -> 
    Array.map parsed_game.split_names ~f:(fun name ->
        {Timer_types.title = name; duration = None}
      )

let load_run parsed_run =
  let splits = Array.map parsed_run.splits ~f:(fun split ->
      match split.time with
      | Some time_str -> (
          match Duration.of_string time_str with
          | Some time -> {
              Timer_types.title = split.title;
              time = Some time;
              is_gold = split.is_gold;
            }
          | None -> assert false
        )

      | None -> {
          Timer_types.title = split.title;
          time = None;
          is_gold = false;
        }
    )
  in

  {Timer_types.attempt = parsed_run.attempt; splits = splits}

let load filepath =
  let game = Sexp.load_sexp_conv_exn filepath game_of_sexp in
  let pb = match game.personal_best with
    | Some run -> Some (load_run run)
    | None -> None
  in
  let golds = load_golds game in
  let history = List.map game.history ~f:load_run in

  {
    Timer_types.title = game.title;
    category = game.category;
    attempts = game.attempts;
    completed = game.completed;

    split_names = game.split_names;
    golds = golds;
    history = history;

    comparison = pb;
    state = Idle;
  }