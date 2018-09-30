open Base
open Timer_types

let rec ahead_by run split_num =
  if split_num < 0 then None else
    match run.comparison with
    | None -> None
    | Some comp_times ->
      if split_num = run.curr_split then
        Some (Duration.since run.start_time - comp_times.(split_num))

      else
        match run.splits.(split_num) with
        | None -> ahead_by run (split_num - 1)
        | Some time -> Some (time - comp_times.(split_num))

let segment_time run split_num =
  if split_num > run.curr_split then None else

    let curr_time =
      if split_num = run.curr_split 
      then Some (Duration.since run.start_time)
      else run.splits.(split_num)
    in

    let last_time = if split_num = 0 then Some 0 else run.splits.(split_num - 1) in

    match curr_time, last_time with
    | Some t1, Some t2 -> Some (t1 - t2)
    | _ -> None

let is_gold run split_num =
  if split_num >= run.curr_split then false else
    match run.game.golds with
    | None -> false
    | Some golds -> (
        match segment_time run split_num with 
        | Some seg_time -> seg_time < golds.(split_num)
        | None -> false
      )
