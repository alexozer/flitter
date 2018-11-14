open Core
open Timer_types

module Game = struct
  type t =
    { title : string
    ; category : string
    ; attempts : int
    ; completed : int
    ; split_names : string list
    ; golds : Gold.t list sexp_option
    ; personal_best : Archived_run.t sexp_option
    ; world_record : Archived_run.t sexp_option
    ; history : Archived_run.t sexp_list }
  [@@deriving sexp]

  let t_of_sexp sexp =
    let game = t_of_sexp sexp in
    let num_splits = List.length game.split_names in
    if num_splits = 0
    then of_sexp_error "No split names defined" sexp
    else
      let check_run run =
        let pb_ok =
          match (run : Archived_run.t option) with
          | Some r -> List.length r.splits = num_splits
          | None -> true
        in
        if not pb_ok
        then
          of_sexp_error
            "Personal best has different number of splits than split_names"
            sexp
        else ()
      in
      check_run game.personal_best;
      check_run game.world_record;
      let history_runs_ok =
        List.fold game.history ~init:true ~f:(fun all_ok run ->
            all_ok && List.length run.splits = num_splits )
      in
      if not history_runs_ok
      then
        of_sexp_error
          "Not all history runs have same number of splits as split_names"
          sexp
      else game
  ;;
end

let load_golds (parsed_game : Game.t) =
  match parsed_game.golds with
  | Some segments ->
    List.map segments ~f:(fun seg -> {Gold.title = seg.title; duration = seg.duration})
  | None ->
    List.map parsed_game.split_names ~f:(fun name -> {Gold.title = name; duration = None})
;;

let load_run (parsed_run : Archived_run.t) =
  let splits =
    List.map parsed_run.splits ~f:(fun split ->
        match split.time with
        | Some duration ->
          {Split.title = split.title; time = Some duration; is_gold = split.is_gold}
        | None -> {Split.title = split.title; time = None; is_gold = false} )
  in
  {Archived_run.attempt = parsed_run.attempt; splits}
;;

let load_run_opt = Option.map ~f:load_run

let load filepath =
  let game = Sexp.load_sexp_conv_exn filepath Game.t_of_sexp in
  let pb = load_run_opt game.personal_best in
  let wr = load_run_opt game.world_record in
  let golds = load_golds game in
  let history = List.map game.history ~f:load_run in
  { Timer.title = game.title
  ; category = game.category
  ; attempts = game.attempts
  ; completed = game.completed
  ; split_names = game.split_names
  ; golds
  ; history
  ; comparison = pb
  ; pb
  ; wr
  ; state = Idle
  ; splits_file = filepath }
;;

let save (timer : Timer_types.Timer.t) =
  let game =
    { Game.title = timer.title
    ; category = timer.category
    ; attempts = timer.attempts
    ; completed = timer.completed
    ; split_names = timer.split_names
    ; golds = Some timer.golds
    ; history = timer.history
    ; personal_best = timer.pb
    ; world_record = timer.wr }
  in
  let sexp = Game.sexp_of_t game in
  let sexp_string = Sexp_pretty.sexp_to_string sexp in
  Out_channel.write_all timer.splits_file ~data:sexp_string
;;
