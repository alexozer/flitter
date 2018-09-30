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

let preamble run width =
  let center = center_pad width in
  let bold_color = A.(Colors.text ++ st bold) in
  let title = I.string bold_color run.game.title |> center in
  let category = I.string bold_color run.game.category |> center in

  I.(title <-> category)

let splits_header width =
  let labels = ["Delta"; "Sgmt"; "Time"] in

  let colored = List.map ~f:(I.string Colors.label) labels in
  let cell_padded = List.map ~f:(left_pad time_col_width) colored in
  let joined = I.hcat cell_padded in
  let padded = left_pad width joined in

  let br = I.uchar Colors.label (Caml.Uchar.of_int 0x2500) width 1 in

  I.(padded <-> br)

let time_color run split_num =
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

  if Splits.is_gold run split_num then Colors.rainbow () else
    match Splits.ahead_by run split_num with
    | None -> Colors.ahead_gain
    | Some delta ->
      match Splits.ahead_by run (split_num - 1) with
      | None -> (if delta < 0 then Colors.ahead_gain else Colors.behind_loss)
      | Some prev_delta -> (
          if delta < 0 
          then if delta < prev_delta then Colors.ahead_gain else Colors.ahead_loss
          else if delta > prev_delta then Colors.behind_loss else Colors.behind_gain
        )

let split_row run width i =
  let bg_color = if i = run.curr_split then Colors.selection_bg else Colors.default_bg in

  let title = I.string A.(Colors.text ++ bg bg_color) run.game.split_names.(i) in
  let time_cols =
    if i > run.curr_split then I.char Colors.bg ' ' (time_col_width * 3) 1

    else
      let delta_image =
        match Splits.ahead_by run i with
        | None -> I.string A.(Colors.text ++ bg bg_color) "-"
        | Some delta -> 
          let time_str = Duration.to_string delta 1 in
          let time_str_sign = if delta >= 0 then "+" ^ time_str else time_str in
          I.string A.(time_color run i ++ bg bg_color) time_str_sign
      in

      let sgmt_image =
        match Splits.segment_time run i with
        | None -> I.string A.(Colors.text ++ bg bg_color) "-"
        | Some sgmt -> I.string A.(Colors.text ++ bg bg_color) (Duration.to_string sgmt 1)
      in

      let time_str =
        if i = run.curr_split then 
          Duration.to_string (Duration.since run.start_time) 1
        else
          match run.splits.(i) with
          | Some time -> Duration.to_string time 1
          | None -> if i < run.curr_split then "-" else ""
      in
      let time_image = I.string A.(Colors.text ++ bg bg_color) time_str in

      List.map [delta_image; sgmt_image; time_image] ~f:(left_pad time_col_width)
      |> I.hcat
  in

  let row_top = join_pad width title time_cols in
  let row_bottom = I.char A.(fg bg_color ++ bg bg_color) ' ' width 1 in
  I.(row_top </> row_bottom)

let splits run width =
  Array.mapi run.game.split_names ~f:(fun i _ -> split_row run width i)
  |> Array.to_list |> I.vcat

let big_timer run width =
  let time, color = match run.state with
    | Idle -> 0, Colors.idle

    | Timing -> Duration.since run.start_time, time_color run run.curr_split

    | Paused pause_time ->
      let time = (pause_time -. run.start_time) *. 1000. |> Int.of_float in
      let color = time_color run run.curr_split in
      time, color

    | Done -> 0, Colors.ahead_gain
  in

  Duration.to_string time 2
  |> Big.image_of_string color
  |> left_pad width

let sob run width =
  let sob_time = match run.game.golds with
    | Some golds ->
      let sob = Array.reduce_exn golds ~f:(+) in
      I.string Colors.text (Duration.to_string sob 2)
    | None -> I.empty
  in

  let sob_desc = I.string Colors.text "Sum of Best Segments" in
  join_pad width sob_desc sob_time

let post_info run width =
  sob run width

(* Result might be slightly bigger than given size *)
let rec subdivide_space color w h max_size =
  if w > max_size || h > max_size then
    let subspace = subdivide_space color (w / 2 + 1) (h / 2 + 1) max_size in
    I.((subspace <|> subspace) <-> (subspace <|> subspace))
  else
    I.char color ' ' w h

let display run (w, h) =
  (* TODO remedy this Notty bug workaround 
     Overlaying the timer with a Notty char grid (I.char) seems to cause 
     flickering at a high draw rate, but drawing smaller regions of the background
     doesn't seem to.

     I'd guess this is a bug in Notty as I was able to reproduce in
     a few different terminals (Gnome-terminal, Termite, urxvt, not xterm though)
  *)
  I.(
    (
      preamble run w <->
      void w 1 <->
      splits_header w <->
      splits run w <->
      void w 1 <->
      big_timer run w <->
      post_info run w
    ) </> subdivide_space Colors.bg w h 40
  )

type t = Notty_lwt.Term.t

let make () =
  Notty_lwt.Term.create ()

let draw term run =
  let open Notty_lwt in
  let image = display run (Term.size term) in
  Term.image term image