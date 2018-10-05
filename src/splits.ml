open Base
open Timer_types

let run_duration run =
  match run.state with
  | Paused paused_time -> Duration.between run.start_time paused_time
  | _ -> Duration.since run.start_time

let rec ahead_by run ?now split_num =
  let curr_run_duration = match now with
    | Some t -> Duration.between run.start_time t
    | None -> run_duration run
  in

  if split_num < 0 then None else
    match run.comparison with
    | Some comp_times -> (
        match comp_times.splits.(split_num).time with
        | Some comp_time -> (
            if split_num = run.curr_split then
              Some (curr_run_duration - comp_time)

            else
              match run.splits.(split_num) with
              | None -> ahead_by run (split_num - 1)
              | Some time -> Some (time - comp_time)
          )
        | None -> None
      )

    | None -> None

let segment_time run ?now split_num =
  let curr_run_duration = match now with
    | Some t -> Duration.between run.start_time t
    | None -> run_duration run
  in

  if split_num > run.curr_split then None else
    let curr_time =
      if split_num = run.curr_split 
      then Some curr_run_duration
      else run.splits.(split_num)
    in

    let last_time = if split_num = 0 then Some 0 else run.splits.(split_num - 1) in

    match curr_time, last_time with
    | Some t1, Some t2 -> Some (t1 - t2)
    | _ -> None

let is_gold run split_num =
  if split_num = run.curr_split then false else
    match segment_time run split_num with
    | Some seg_time -> (
        match run.golds.(split_num).duration with
        | Some duration -> seg_time < duration
        | None -> true
      )
    | None -> false

let updated_golds run =
  let seg_durations = Array.mapi run.splits ~f:(fun i _ ->
      segment_time run i
    ) in
  let old_durations = Array.map run.golds ~f:(fun g -> g.duration) in

  let new_durations = Array.mapi run.split_names ~f:(fun i _ ->
      if i >= run.curr_split then None else
        match seg_durations.(i), old_durations.(i) with
        | Some n, Some o -> if n < o then Some n else Some o
        | Some n, None -> Some n
        | None, Some o -> Some o
        | None, None -> None
    ) in

  Array.map2_exn run.split_names new_durations ~f:(fun name dur ->
      {title = name; duration = dur}
    )