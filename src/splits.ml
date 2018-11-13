open Core
open Timer_types

let split_time (timer : Timer.t) ?now split_num =
  if split_num < 0 then Some Duration.zero
  else
    let curr_time = Option.value now ~default:(Time_ns.now ()) in
    match timer.state with
    | Idle -> None
    | Paused (splits, start_time, pause_time) ->
        if split_num > List.length splits then None
        else if split_num = List.length splits then
          Some (Duration.between start_time pause_time)
        else List.nth splits split_num |> Option.join
    | Timing (splits, start_time) | Done (splits, start_time) ->
        if split_num > List.length splits then None
        else if split_num = List.length splits then
          Some (Duration.between start_time curr_time)
        else List.nth splits split_num |> Option.join

let duration (timer : Timer.t) =
  match timer.state with
  | Idle -> Duration.zero
  | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) -> (
    match split_time timer (List.length splits) with
    | Some t -> t
    | None -> assert false )

let ahead_by timer ?now split_num =
  if split_num < 0 then None
  else
    let split_time = split_time timer ?now split_num in
    let comp_time =
      Option.map timer.comparison ~f:Archived_run.splits
      |> Option.bind ~f:(fun comp -> List.nth comp split_num)
      |> Option.bind ~f:Split.time
    in
    Option.map2 split_time comp_time ~f:Time_ns.Span.( - )

let segment_time timer ?now split_num =
  let t0 = split_time timer ?now (split_num - 1) in
  let t1 = split_time timer ?now split_num in
  Option.map2 t1 t0 ~f:Time_ns.Span.( - )

let current_split (timer : Timer.t) =
  match timer.state with
  | Idle -> None
  | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) ->
      Some (List.length splits)

let is_gold timer split_num =
  (* lezed1: I'm pretty sure this first condition isn't needed. *)
  let has_valid_split =
    Option.exists (current_split timer) ~f:(fun n -> split_num < n)
  in
  let is_lower =
    segment_time timer split_num
    |> Option.exists ~f:(fun seg_time ->
           List.nth timer.golds split_num
           |> Option.bind ~f:Gold.duration
           |> Option.exists ~f:(fun duration -> seg_time < duration) )
  in
  has_valid_split && is_lower

let updated_golds (timer : Timer.t) =
  match timer.state with
  | Idle -> timer.golds
  | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) ->
      let seg_durations =
        List.length splits |> List.init ~f:(fun i -> segment_time timer i)
      in
      let old_durations = List.map timer.golds ~f:Gold.duration in
      let new_durations =
        List.length timer.split_names
        |> List.init ~f:(fun i ->
               if i >= List.length splits then
                 List.nth old_durations i |> Option.join
               else
                 Option.merge
                   (List.nth seg_durations i |> Option.join)
                   (List.nth old_durations i |> Option.join)
                   ~f:Time_ns.Span.min )
      in
      List.map2_exn timer.split_names new_durations ~f:(fun name dur ->
          {Gold.title= name; duration= dur} )

let gold_sum timer start bound =
  let gold_list = List.slice (updated_golds timer) start bound in
  List.fold gold_list ~init:(Some Duration.zero) ~f:(fun sum gold ->
      Option.map2 sum gold.duration ~f:Time_ns.Span.( + ) )

let archived_split_time (timer : Timer.t) split_num =
  if split_num < 0 then Some Duration.zero
  else
    Option.map timer.comparison ~f:Archived_run.splits
    |> Option.bind ~f:(fun comp -> List.nth comp split_num)
    |> Option.bind ~f:Split.time

let archived_segment_time run split_num =
  let t0 = archived_split_time run (split_num - 1) in
  let t1 = archived_split_time run split_num in
  Option.map2 t1 t0 ~f:Time_ns.Span.( - )

let archive_done_run (timer : Timer.t) splits =
  let run_splits =
    List.mapi timer.split_names ~f:(fun i name ->
        { Split.title= name
        ; time= List.nth splits i |> Option.join
        ; is_gold= is_gold timer i } )
  in
  {Archived_run.attempt= timer.attempts; splits= run_splits}

let updated_pb (timer : Timer.t) =
  match timer.state with
  | Idle | Timing _ | Paused _ -> timer.pb
  | Done (splits, _) ->
      Option.bind timer.pb ~f:(fun pb_run ->
          let last_idx = List.length splits - 1 in
          Option.both
            (List.nth splits last_idx |> Option.join)
            (List.nth pb_run.splits last_idx)
          |> Option.bind ~f:(fun (new_t, old_t) ->
                 Option.map old_t.time ~f:(Tuple2.create new_t) )
          |> Option.bind ~f:(fun (new_t, old_t) ->
                 if Time_ns.Span.(new_t < old_t) then
                   Some (archive_done_run timer splits)
                 else timer.pb ) )
