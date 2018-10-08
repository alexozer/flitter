open Core_kernel
open Timer_types

let split_time timer ?now split_num =
  if split_num <= 0 then Some 0 else
    let curr_time = match now with Some t -> t | None -> Unix.gettimeofday () in

    match timer.state with
    | Idle -> None

    | Paused (splits, start_time, pause_time) ->
      if split_num > Array.length splits then None
      else if split_num = Array.length splits
      then Some (Duration.between start_time pause_time)
      else splits.(split_num)

    | Timing (splits, start_time) | Done (splits, start_time) ->
      if split_num > Array.length splits then None
      else if split_num = Array.length splits
      then Some (Duration.between start_time curr_time)
      else splits.(split_num)

let duration timer =
  match timer.state with
  | Idle -> 0
  | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) -> (
      match split_time timer (Array.length splits) with
      | Some t -> t
      | None -> assert false
    )

let ahead_by timer ?now split_num =
  let split_time = split_time timer ?now split_num in
  let comp_time = match timer.comparison with
    | None -> None
    | Some comp -> comp.splits.(split_num).time
  in

  match split_time, comp_time with
  | Some st, Some ct -> Some (st - ct)
  | _ -> None

let segment_time timer ?now split_num =
  let t0 = split_time timer ?now (split_num - 1) in
  let t1 = split_time timer ?now split_num in

  match t0, t1 with
  | Some t0', Some t1' -> Some (t1' - t0')
  | _ -> None

let is_gold timer split_num =
  match segment_time timer split_num with
  | Some seg_time -> (
      match timer.golds.(split_num).duration with
      | Some duration -> seg_time < duration
      | None -> true
    )
  | None -> false

let updated_golds timer =
  match timer.state with
  | Idle -> timer.golds
  | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) ->

    let seg_durations = Array.mapi splits ~f:(fun i _ ->
        segment_time timer i
      ) in
    let old_durations = Array.map timer.golds ~f:(fun g -> g.duration) in

    let new_durations = Array.mapi timer.split_names ~f:(fun i _ ->
        if i >= Array.length splits then None else
          match seg_durations.(i), old_durations.(i) with
          | Some n, Some o -> if n < o then Some n else Some o
          | Some n, None -> Some n
          | None, Some o -> Some o
          | None, None -> None
      ) in

    Array.map2_exn timer.split_names new_durations ~f:(fun name dur ->
        {title = name; duration = dur}
      )

let archived_split_time run split_num =
  if split_num < 0 then Some 0 else
    match run.comparison with
    | Some comp -> comp.splits.(split_num).time
    | None -> None

let archived_segment_time run split_num =
  let t0 = archived_split_time run (split_num - 1) in
  let t1 = archived_split_time run split_num in
  match t0, t1 with
  | Some t0', Some t1' -> Some (t1' - t0')
  | _ -> None