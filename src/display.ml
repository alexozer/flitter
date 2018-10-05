open Base
open Notty
open Timer_types

let time_col_width = 10

let left_pad width i =
  (* That's right folks you saw it right here with your own eyes *)
  I.hpad (width - I.width i) 0 i

let center_pad width i =
  if I.width i > width 
  then I.hcrop 0 (I.width i - width) i
  else
    let pad = (width - I.width i) in
    let lpad = pad / 2 in
    let rpad = pad - lpad in
    I.hpad lpad rpad i

let join_pad width left right =
  let center_size = width - I.width left - I.width right in
  let padded_right = I.hpad center_size 0 right in
  I.(left <|> padded_right)

let preamble timer width =
  let center = center_pad width in
  let bold_color = A.(Colors.text ++ st bold) in
  let title = I.string bold_color timer.title |> center in
  let category = I.string bold_color timer.category |> center in

  I.(title <-> category)

let splits_header width =
  let labels = ["Delta"; "Sgmt"; "Time"] in

  let colored = List.map ~f:(I.string Colors.label) labels in
  let cell_padded = List.map ~f:(left_pad time_col_width) colored in
  let joined = I.hcat cell_padded in
  let padded = left_pad width joined in

  let br = I.uchar Colors.label (Caml.Uchar.of_int 0x2500) width 1 in

  I.(padded <-> br)

let time_color timer split_num =
  (* 
  If this isn't the current split, check if segment is a gold
  else
    Find current time
    Find amount we're ahead/behind by
    Find time ahead/behind by in last split possible
    If this isn't available
      Colors.is either ahead gain or behind loss
    else
      color depends on whether currently ahead and how lead/loss compares to last available lead/loss
  *)

  if Splits.is_gold timer split_num then Colors.rainbow () else
    match Splits.ahead_by timer split_num with
    | None -> Colors.ahead_gain
    | Some delta ->
      match Splits.ahead_by timer (split_num - 1) with
      | None -> (if delta < 0 then Colors.ahead_gain else Colors.behind_loss)
      | Some prev_delta -> (
          if delta < 0 
          then if delta < prev_delta then Colors.ahead_gain else Colors.ahead_loss
          else if delta > prev_delta then Colors.behind_loss else Colors.behind_gain
        )

let split_row timer width i =
  let idle = match timer.state with Idle -> true | _ -> false in
  let bg_color = if not idle && i = timer.curr_split then Colors.selection_bg else Colors.default_bg in

  let title = I.string A.(Colors.text ++ bg bg_color) timer.split_names.(i) in
  let time_cols =
    if idle || i > timer.curr_split then
      I.char Colors.bg ' ' (time_col_width * 3) 1

    else
      let delta_image =
        match Splits.ahead_by timer i with
        | None -> I.string A.(Colors.text ++ bg bg_color) "-"
        | Some delta -> 
          let time_str = Duration.to_string delta 1 in
          let time_str_sign = if delta >= 0 then "+" ^ time_str else time_str in
          I.string A.(time_color timer i ++ bg bg_color) time_str_sign
      in

      let sgmt_image =
        match Splits.segment_time timer i with
        | None -> I.string A.(Colors.text ++ bg bg_color) "-"
        | Some sgmt -> I.string A.(Colors.text ++ bg bg_color) (Duration.to_string sgmt 1)
      in

      let time_str =
        if i = timer.curr_split then 
          Duration.to_string (Splits.run_duration timer) 1
        else
          match timer.splits.(i) with
          | Some time -> Duration.to_string time 1
          | None -> if i < timer.curr_split then "-" else ""
      in
      let time_image = I.string A.(Colors.text ++ bg bg_color) time_str in

      List.map [delta_image; sgmt_image; time_image] ~f:(left_pad time_col_width)
      |> I.hcat
  in

  let row_top = join_pad width title time_cols in
  let row_bottom = I.char A.(fg bg_color ++ bg bg_color) ' ' width 1 in
  I.(row_top </> row_bottom)

let splits timer width =
  Array.mapi timer.split_names ~f:(fun i _ -> split_row timer width i)
  |> Array.to_list |> I.vcat

let big_timer timer width =
  let time, color = match timer.state with
    | Idle -> 0, Colors.idle

    | Timing -> Duration.since timer.start_time, time_color timer timer.curr_split

    | Paused pause_time ->
      let time = (pause_time -. timer.start_time) *. 1000. |> Int.of_float in
      let color = time_color timer timer.curr_split in
      time, color

    | Done -> (
        let last_split_num = Array.length timer.split_names - 1 in
        match timer.splits.(last_split_num) with
        | None -> failwith "Last split found empty on done"
        | Some time -> (
            match timer.comparison with
            | None -> time, Colors.ahead_gain
            | Some comp -> (
                match comp.splits.(last_split_num).time with
                | Some comp_time ->
                  let color = if time < comp_time then Colors.rainbow () else Colors.behind_loss in
                  time, color
                | None -> time, Colors.idle
              )
          )
      )
  in

  Duration.to_string time 2
  |> Big.image_of_string color
  |> left_pad width

let sob timer width =
  let sob_time =
    let updated_golds = Splits.updated_golds timer in 
    let sum = Array.fold updated_golds ~init:(Some 0) ~f:(fun sum gold ->
        match sum, gold.duration with
        | Some x, Some y -> Some (x + y)
        | _ -> None
      ) in

    match sum with
    | Some sob -> I.string Colors.text (Duration.to_string sob 2)
    | None -> I.empty
  in

  let sob_desc = I.string Colors.text "Sum of Best Segments" in
  join_pad width sob_desc sob_time

let post_info timer width =
  sob timer width

(* Result might be slightly bigger than given size *)
let rec subdivide_space color w h max_size =
  if w > max_size || h > max_size then
    let subspace = subdivide_space color (w / 2 + 1) (h / 2 + 1) max_size in
    I.((subspace <|> subspace) <-> (subspace <|> subspace))
  else
    I.char color ' ' w h

let display timer (w, h) =
  (* TODO remedy this Notty bug workaround 
     Overlaying the timer with a Notty char grid (I.char) seems to cause 
     flickering at a high draw rate, but drawing smaller regions of the background
     doesn't seem to.

     I'd guess this is a bug in Notty as I was able to reproduce in
     a few different terminals (Gnome-terminal, Termite, urxvt, not xterm though)
  *)
  I.(
    (
      preamble timer w <->
      void w 1 <->
      splits_header w <->
      splits timer w <->
      void w 1 <->
      big_timer timer w <->
      post_info timer w
    ) </> subdivide_space Colors.bg w h 10
  )

type t = Notty_lwt.Term.t

let make () =
  Notty_lwt.Term.create ()

let draw term timer =
  let open Notty_lwt in
  let image = display timer (Term.size term) in
  Term.image term image