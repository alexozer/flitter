open Base
open Notty

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
let ahead_gain = A.(attr_of_string "#2ed573" ++ st bold)
let ahead_loss = attr_of_string "#7bed9f"
let behind_gain = attr_of_string "#ff6b81"
let behind_loss = A.(attr_of_string "#ff4757" ++ st bold)
let gold = attr_of_string "#ffa502"
let idle = attr_of_string "#1e90ff"
let bg = A.(fg bg_color ++ bg bg_color)
let label = attr_of_string "#a4b0be"
(* let selection = attr_of_string "#57606f" *)