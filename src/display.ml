open Core
open Notty

let time_col_width = 10

let left_pad width i =
  (* That's right folks you saw it right here with your own eyes *)
  I.hpad (width - I.width i) 0 i
;;

let center_pad width i =
  if I.width i > width
  then I.hcrop 0 (I.width i - width) i
  else
    let pad = width - I.width i in
    let lpad = pad / 2 in
    let rpad = pad - lpad in
    I.hpad lpad rpad i
;;

let join_pad width left right =
  let center_size = width - I.width left - I.width right in
  let padded_right = I.hpad center_size 0 right in
  I.(left <|> padded_right)
;;

let preamble (timer : Timer.t) width =
  let center = center_pad width in
  let bold_color = A.(Colors.text ++ st bold) in
  let title = I.string bold_color timer.title |> center in
  let category = I.string bold_color timer.category |> center in
  I.(title <-> category)
;;

let splits_header (timer : Timer.t) width =
  let labels =
    match timer.wr with
    | Some _ -> ["Delta"; "Sgmt"; "Time"; "WR"]
    | None -> ["Delta"; "Sgmt"; "Time"]
  in
  let colored = List.map ~f:(I.string Colors.label) labels in
  let cell_padded = List.map ~f:(left_pad time_col_width) colored in
  let joined = I.hcat cell_padded in
  let padded = left_pad width joined in
  let br = I.uchar Colors.label (Caml.Uchar.of_int 0x2500) width 1 in
  I.(padded <-> br)
;;

type time_status =
  | Ahead_gain
  | Ahead_loss
  | Behind_gain
  | Behind_loss
  | Gold

let time_status (timer : Timer.t) split_num =
  (*
  If this isn't the current split, check if segment is a gold
  else
    Find current time
    Find amount we're ahead/behind by
    Find time ahead/behind by in last split possible
    If this isn't available
      Color is either ahead gain or behind loss
    else
      color depends on whether currently ahead and how lead/loss compares to last available lead/loss
  *)
  if Splits.is_gold timer split_num
  then Gold
  else
    match Splits.ahead_by timer split_num with
    | None -> Ahead_gain
    | Some delta ->
      (match Splits.ahead_by timer (split_num - 1) with
      | None -> if Time_ns.Span.(delta < zero) then Ahead_gain else Behind_loss
      | Some prev_delta ->
        if Time_ns.Span.(delta < zero)
        then if delta < prev_delta then Ahead_gain else Ahead_loss
        else if delta > prev_delta
        then Behind_loss
        else Behind_gain)
;;

let time_color timer split_num =
  match time_status timer split_num with
  | Ahead_gain -> Colors.ahead_gain
  | Ahead_loss -> Colors.ahead_loss
  | Behind_gain -> Colors.behind_gain
  | Behind_loss -> Colors.behind_loss
  | Gold -> Colors.rainbow ()
;;

let show_delta (timer : Timer.t) split_num =
  (* if previous split or behind or 
      (if gold available and segment time avail and seg slower than gold): 
       show
     else 
       hide 
  *)
  match timer.state with
  | Idle -> false
  | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) ->
    if split_num < List.length splits
    then true
    else (
      match time_status timer split_num with
      | Behind_gain | Behind_loss -> true
      | Ahead_gain | Ahead_loss | Gold ->
        let sgmt = Splits.segment_time timer split_num in
        let gold = List.nth timer.golds split_num |> Option.bind ~f:Gold.duration in
        Option.map2 sgmt gold ~f:Time_ns.Span.( > ) |> Option.value ~default:false )
;;

let split_row (timer : Timer.t) width i =
  let bg_attr =
    match timer.state with
    | Idle | Done _ -> Colors.default_bg
    | Timing (splits, _) | Paused (splits, _, _) ->
      if i = List.length splits then Colors.selection_bg else Colors.default_bg
  in
  let uncolored_attr = A.(Colors.text ++ bg_attr) in
  let curr_split =
    match timer.state with
    | Idle -> -1
    | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) -> List.length splits
  in
  let show_comparison = i > curr_split in
  let title = List.nth_exn timer.split_names i |> I.string bg_attr in
  (* Compute the split's ahead/behind time image *)
  let delta_image =
    if show_comparison
    then I.string uncolored_attr "-"
    else
      match Splits.ahead_by timer i with
      | None -> I.string uncolored_attr "-"
      | Some delta ->
        if not (show_delta timer i)
        then I.string uncolored_attr ""
        else
          let time_str = Duration.to_string_plus delta 1 in
          let color = A.(time_color timer i ++ bg_attr) in
          I.string color time_str
  in
  (* Compute the image of the split's segment time *)
  let sgmt_image =
    let seg_time =
      if show_comparison
      then Splits.archived_segment_time timer i
      else Splits.segment_time timer i
    in
    match seg_time with
    | None -> I.string uncolored_attr "-"
    | Some sgmt -> I.string uncolored_attr (Duration.to_string sgmt 1)
  in
  (* Compute the image of the split's absolute time *)
  let time =
    if show_comparison
    then Splits.archived_split_time timer i
    else Splits.split_time timer i
  in
  let time_str =
    Option.value_map time ~default:"-" ~f:(fun t -> Duration.to_string t 1)
  in
  let time_image = I.string uncolored_attr time_str in
  (* Compute the image of the WR comparison cell *)
  let wr_image =
    Option.map timer.wr ~f:(fun wr_run ->
        let default_img =
          List.nth wr_run.splits i
          |> Option.bind ~f:Split.time
          |> Option.value_map ~default:"-" ~f:(fun t -> Duration.to_string t 2)
          |> I.string uncolored_attr
        in
        if i >= curr_split
        then default_img
        else
          (* Determine how much we're ahead or behind WR *)
          match timer.state with
          | Idle -> default_img
          | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) ->
            Option.map2
              (List.nth splits i |> Option.join)
              (List.nth wr_run.splits i |> Option.bind ~f:Split.time)
              ~f:(fun curr_t wr_t ->
                let delta = Time_ns.Span.(curr_t - wr_t) in
                let delta_str = Duration.to_string_plus delta 2 in
                let delta_color =
                  match Time_ns.Span.sign delta with
                  | Neg -> Colors.ahead_gain
                  | Pos | Zero -> Colors.behind_gain
                in
                I.string delta_color delta_str )
            |> Option.value ~default:(I.string uncolored_attr "-") )
  in
  (* Combine the three time columns together with proper padding *)
  let time_cells =
    match wr_image with
    | Some wr -> [delta_image; sgmt_image; time_image; wr]
    | None -> [delta_image; sgmt_image; time_image]
  in
  let time_cols_combined = List.map time_cells ~f:(left_pad time_col_width) |> I.hcat in
  (* Add the split title and background color to fill in the padding *)
  let row_top = join_pad width title time_cols_combined in
  let row_bottom = I.char bg_attr ' ' width 1 in
  I.(row_top </> row_bottom)
;;

let splits (timer : Timer.t) width =
  List.length timer.split_names
  |> List.init ~f:(fun i -> split_row timer width i)
  |> List.to_list
  |> I.vcat
;;

let big_timer (timer : Timer.t) width =
  let time, color =
    match timer.state with
    | Idle -> Duration.zero, Colors.idle
    | Timing (splits, start_time) ->
      let time = Duration.since start_time in
      let color = time_color timer (List.length splits) in
      time, color
    | Paused (splits, start_time, pause_time) ->
      let time = Duration.between start_time pause_time in
      let color = time_color timer (List.length splits) in
      time, color
    | Done (splits, _) ->
      let last_split_num = List.length splits - 1 in
      let time =
        List.nth splits last_split_num
        |> Option.join
        |> Option.value_exn ~message:"Last split found empty on done"
      in
      Option.value_map
        timer.comparison
        ~default:(time, Colors.ahead_gain)
        ~f:(fun comp ->
          let comp_time =
            List.nth comp.splits last_split_num
            |> Option.bind ~f:Split.time
            |> Option.value_exn ~message:"Last split of comparison found empty"
          in
          let color =
            if time < comp_time then Colors.rainbow () else Colors.behind_loss
          in
          time, color )
  in
  Duration.to_string time 2 |> Big.image_of_string color |> left_pad width
;;

let previous_segment (timer : Timer.t) width =
  let desc_img = I.string Colors.default_bg "Previous Segment" in
  let empty_time_img = I.string Colors.default_bg "-" in
  let time_img =
    match timer.state with
    | Idle -> empty_time_img
    | Timing (splits, _) | Paused (splits, _, _) | Done (splits, _) ->
      let curr_split = List.length splits in
      let prev_delta = Splits.ahead_by timer (curr_split - 1) in
      let prev_prev_delta = Splits.ahead_by timer (curr_split - 2) in
      (match prev_delta, prev_prev_delta with
      | Some pd, Some ppd ->
        let diff = Time_ns.Span.(pd - ppd) in
        let diff_str = Duration.to_string_plus diff 2 in
        let color =
          if Time_ns.Span.(diff < zero) then Colors.ahead_gain else Colors.behind_loss
        in
        I.string color diff_str
      | _ -> empty_time_img)
  in
  join_pad width desc_img time_img
;;

let best_possible_time (timer : Timer.t) width =
  let t =
    match timer.state with
    | Idle -> Splits.gold_sum timer 0 (List.length timer.split_names)
    | Timing (splits, _) | Paused (splits, _, _) ->
      let curr_split = List.length splits in
      let total_splits = List.length timer.split_names in
      let future_sob = Splits.gold_sum timer (curr_split + 1) total_splits in
      let curr_gold =
        List.nth (Splits.updated_golds timer) curr_split |> Option.bind ~f:Gold.duration
      in
      let last_split_time =
        if curr_split = 0
        then Some Duration.zero
        else List.nth splits (curr_split - 1) |> Option.join
      in
      let curr_seg = Splits.segment_time timer curr_split in
      (match future_sob, curr_gold, last_split_time, curr_seg with
      | Some future_sob', Some curr_gold', Some last_split_time', Some curr_seg' ->
        Some Time_ns.Span.(last_split_time' + max curr_seg' curr_gold' + future_sob')
      | _ -> None)
    | Done (splits, _) -> List.nth splits (List.length splits - 1) |> Option.join
  in
  let time_img =
    match t with
    | Some t' ->
      let time_str = Duration.to_string t' 2 in
      I.string Colors.text time_str
    | None -> I.string Colors.text "-"
  in
  let desc_img = I.string Colors.text "Best Possible Time" in
  join_pad width desc_img time_img
;;

let sob timer width =
  let sob_time = Splits.gold_sum timer 0 (List.length timer.split_names) in
  let sob_img =
    match sob_time with
    | Some sob -> I.string Colors.text (Duration.to_string sob 2)
    | None -> I.empty
  in
  let sob_desc = I.string Colors.text "Sum of Best Segments" in
  join_pad width sob_desc sob_img
;;

(* Result might be slightly bigger than given size *)
let rec subdivide_space color w h max_size =
  if w > max_size || h > max_size
  then
    let subspace = subdivide_space color ((w / 2) + 1) ((h / 2) + 1) max_size in
    I.(subspace <|> subspace <-> (subspace <|> subspace))
  else I.char color ' ' w h
;;

let display timer (w, h) =
  (* TODO remedy this Notty bug workaround
     Overlaying the timer with a Notty char grid (I.char) seems to cause
     flickering at a high draw rate, but drawing smaller regions of the background
     doesn't seem to.

     I'd guess this is a bug in Notty as I was able to reproduce in
     a few different terminals (Gnome-terminal, Termite, urxvt, not xterm though)
  *)
  I.(
    preamble timer w
    <-> void w 1
    <-> splits_header timer w
    <-> splits timer w
    <-> void w 1
    <-> big_timer timer w
    <-> previous_segment timer w
    <-> sob timer w
    <-> best_possible_time timer w
    </> subdivide_space Colors.default_bg w h 10)
;;

type t = Notty_unix.Term.t

let make () = Notty_unix.Term.create ()

let draw term timer =
  let open Notty_unix in
  let image = display timer (Term.size term) in
  Term.image term image
;;

let close term = Notty_unix.Term.release term
