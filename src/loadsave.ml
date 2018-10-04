open Base

[@@@warning "-32"]

(* Game type before some preprocessing like parsing the time strings *)
include struct
  (* Workaround, ppx_deriving_yojson is giving a "unused rec" warning *)
  [@@@warning "-39"]

  type split = {
    title : string;
    time : string option;
    isGold : bool;
  }
  [@@deriving yojson]

  type gold = {
    title : string;
    duration : string option;
  }
  [@@deriving yojson]

  type archived_run = {
    attempt : int;
    splits : split array;
  }
  [@@deriving yojson]

  type game = {
    title : string;
    category : string;
    attempts : int;
    completed : int;

    splitNames : string array;
    golds : gold array option;
    personalBest : archived_run option;
    history : archived_run list;
  }
  [@@deriving yojson]
end

let process_golds game =
  match game.golds with
  | Some segments ->
    if Array.length segments <> Array.length game.splitNames then
      failwith "Number of gold segments must equal the number of splits"
    else
      Array.map segments ~f:(fun seg ->
          match seg.duration with
          | Some duration -> (
              match Duration.of_string duration with
              | Some parsed_duration -> {
                  New_types.title = seg.title;
                  duration = Some parsed_duration;
                }
              | None -> failwith ("Invalid time '" ^ duration ^ "' for gold segment " ^ seg.title)
            )

          | None -> {
              New_types.title = seg.title;
              duration = None;
            }
        )

  | None -> 
    Array.map game.splitNames ~f:(fun name ->
        {New_types.title = name; duration = None}
      )

let process_run run =
  let splits = Array.map run.splits ~f:(fun split ->
      match split.time with
      | Some time_str -> (
          match Duration.of_string time_str with
          | Some time -> {
              New_types.title = split.title;
              time = Some time;
              is_gold = split.isGold;
            }
          | None -> failwith ("Split '" ^ split.title ^ "' has invalid time '" ^ time_str ^ "'")
        )

      | None -> {
          New_types.title = split.title;
          time = None;
          is_gold = false;
        }
    )
  in

  {New_types.attempt = run.attempt; splits = splits}

let load filepath : New_types.game =
  let json = Yojson.Safe.from_file filepath in
  match game_of_yojson json with
  | Error err -> failwith err
  | Ok game -> 
    let pb = match game.personalBest with
      | Some run -> Some (process_run run)
      | None -> None
    in
    let golds = process_golds game in
    let history = List.map game.history ~f:process_run in

    {
      New_types.title = game.title;
      category = game.category;
      attempts = game.attempts;
      completed = game.completed;

      split_names = game.splitNames;
      pb = pb;
      golds = golds;
      history = history;
    }
