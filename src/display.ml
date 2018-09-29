open Base
open Notty
open Splits

module Color = struct
  let color_of_string str =
    match String.to_list str with
    | '#' :: r1 :: r2 :: g1 :: g2 :: b1 :: b2 :: [] ->
      let compon_of_hex_digits left right = 
        ['0'; 'x'; left; right] |> String.of_char_list |> Int.of_string in
      let r = compon_of_hex_digits r1 r2 in
      let g = compon_of_hex_digits g1 g2 in
      let b = compon_of_hex_digits b1 b2 in

      A.rgb_888 ~r:r ~g:g ~b:b

    | _ -> failwith "Failed to convert color"

  let bg_color = color_of_string "#2f3542"

  let attr_of_string str =
    A.(fg (color_of_string str) ++ bg bg_color)

  (* https://flatuicolors.com/palette/cn *)
  let text = attr_of_string "#f1f2f6"
  let ahead_gain = attr_of_string "#2ed573"
  let ahead_loss = attr_of_string "#7bed9f"
  let behind_gain = attr_of_string "#ff6b81"
  let behind_loss = attr_of_string "#ff4757"
  let gold = attr_of_string "#ffa502"
  let idle = attr_of_string "#1e90ff"
  let bg = A.(fg bg_color ++ bg bg_color)
  let label = attr_of_string "#a4b0be"
  (* let selection = attr_of_string "#57606f" *)
end

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
  let title = I.string Color.text run.game.title |> center in
  let category = I.string Color.text run.game.category |> center in

  I.(title <-> category)

let splits_header width =
  let labels = ["Delta"; "Sgmt"; "Time"] in

  let colored = List.map ~f:(I.string Color.label) labels in
  let cell_padded = List.map ~f:(left_pad time_col_width) colored in
  let joined = I.hcat cell_padded in
  let padded = left_pad width joined in

  let br = I.uchar Color.label (Caml.Uchar.of_int 0x2500) width 1 in

  I.(padded <-> br)

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

let is_gold run split_num =
  if split_num = run.curr_split then false else
    match run.game.golds with
    | None -> false
    | Some golds -> (
        match run.splits.(split_num), run.splits.(split_num - 1) with
        | Some t1, Some t2 -> t1 - t2 < golds.(split_num)
        | _ -> false
      )

let time_color run split_num =
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

  if is_gold run split_num then Color.gold else
    match ahead_by run split_num with
    | None -> Color.ahead_gain
    | Some delta ->
      match ahead_by run (split_num - 1) with
      | None -> (if delta < 0 then Color.ahead_gain else Color.behind_loss)
      | Some prev_delta -> (
          if delta < 0 
          then if delta < prev_delta then Color.ahead_gain else Color.ahead_loss
          else if delta > prev_delta then Color.behind_loss else Color.behind_gain
        )

let big_timer run width =
  let time, color = match run.state with
    | Idle -> 0, Color.idle

    | Timing ->
      let time = 
        (Unix.gettimeofday () -. run.start_time) *. 1000.
        |> Int.of_float in
      let color = time_color run run.curr_split in
      time, color

    | Paused pause_time ->
      let time = (pause_time -. run.start_time) *. 1000. |> Int.of_float in
      let color = time_color run run.curr_split in
      time, color

    | Done -> 0, Color.ahead_gain
  in

  Duration.to_string time 2
  |> Big.image_of_string color
  |> left_pad width

let sob run width =
  let sob_time = match run.game.golds with
    | Some golds ->
      let sob = Array.reduce_exn golds ~f:(+) in
      I.string Color.text (Duration.to_string sob 2)
    | None -> I.empty
  in

  let sob_desc = I.string Color.text "Sum of Best Segments" in
  join_pad width sob_desc sob_time

let post_info run width =
  sob run width

let display run (w, h) =
  I.(
    (
      preamble run w <->
      void w 1 <->
      splits_header w <->
      (* splits run width_ <-> *)
      big_timer run w <->
      void w 1 <->
      post_info run w
    ) </> I.char Color.bg ' ' w h
  )

type t = Notty_unix.Term.t

let make () =
  Notty_unix.Term.create ()

let draw term run =
  let open Notty_unix in
  let image = display run (Term.size term) in
  Term.image term image