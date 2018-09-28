open Base
open Notty
open Splits

module Color = struct
  let of_string str =
    match String.to_list str with
    | '#' :: r1 :: r2 :: g1 :: g2 :: b1 :: b2 :: [] ->
      let compon_of_hex_digits left right = 
        ['0'; 'x'; left; right] |> String.of_char_list |> Int.of_string in
      let r = compon_of_hex_digits r1 r2 in
      let g = compon_of_hex_digits g1 g2 in
      let b = compon_of_hex_digits b1 b2 in

      A.rgb_888 ~r:r ~g:g ~b:b

    | _ -> failwith "Failed to convert color"

  (* https://flatuicolors.com/palette/cn *)
  let ahead_gain = of_string "#2ed573"
  (* let ahead_loss = of_string "#7bed9f"
     let behind_gain = of_string "#ff6b81"
     let behind_loss = of_string "#ff4757"
     let gold = of_string "#ffa502"
     let idle = of_string "#1e90ff"
     let label = of_string "#a4b0be"
     let selection = of_string "#57606f"
     let fg = of_string "#f1f2f6" *)
  let bg = of_string "#2f3542"
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
  let title = I.string A.empty run.game_info.game |> center in
  let category = I.string A.empty run.game_info.category |> center in

  I.(title <-> category)

let splits_header width =
  let labels = ["Delta"; "Sgmt"; "Time"] in

  let colored = List.map ~f:(I.string A.empty) labels in
  let cell_padded = List.map ~f:(left_pad time_col_width) colored in
  let joined = I.hcat cell_padded in
  let padded = left_pad width joined in

  let br = I.uchar A.empty (Caml.Uchar.of_int 0x2500) width 1 in

  I.(padded <-> br)

let big_timer run width =
  let time = 
    (Unix.gettimeofday () -. run.start_time) *. 1000.
    |> Int.of_float in

  Duration.to_string time 2
  |> Big.image_of_string A.(fg Color.ahead_gain ++ bg Color.bg)
  |> left_pad width

let sob run width =
  let sob = Array.fold run.golds ~init:0 ~f:(fun sum g2 -> sum + g2.duration) in
  let sob_desc = I.string A.empty "Sum of Best Segments" in
  let sob_time = I.string A.empty (Duration.to_string sob 2) in
  join_pad width sob_desc sob_time

let post_info run width =
  sob run width

let display run width =
  let width_ = width in

  I.(
    preamble run width_ <->
    void width_ 1 <->
    splits_header width_ <->
    (* splits run width_ <-> *)
    big_timer run width_ <->
    void width_ 1 <->
    post_info run width_
  )

type t = Notty_unix.Term.t

let make () =
  Notty_unix.Term.create ()

let draw term run =
  let open Notty_unix in
  let width, _ = Term.size term in
  let image = display run width in
  Term.image term image