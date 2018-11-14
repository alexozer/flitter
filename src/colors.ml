open Core
open Notty

let color_of_hexstring str =
  match Color.of_hexstring str with
  | None -> failwith "Failed to derive color from hexstring"
  | Some color ->
    let rgba = Color.to_rgba color in
    A.rgb_888 ~r:rgba.r ~g:rgba.g ~b:rgba.b
;;

let color_of_hsl h s l =
  let hsl = Color.of_hsl h s l in
  let rgba = Color.to_rgba hsl in
  A.rgb_888 ~r:rgba.r ~g:rgba.g ~b:rgba.b
;;

let default_bg = A.(bg (color_of_hexstring "#2f3542"))
let selection_bg = A.(bg (color_of_hexstring "#485460"))
let attr_of_hexstring str = A.(fg (color_of_hexstring str) ++ default_bg)

(* https://flatuicolors.com/palette/cn *)
(* TODO Find better color palette *)
let text = attr_of_hexstring "#f1f2f6"
let ahead_gain = A.(attr_of_hexstring "#2ed573" ++ st bold)
let ahead_loss = attr_of_hexstring "#7bed9f"
let behind_gain = attr_of_hexstring "#ff6b81"
let behind_loss = A.(attr_of_hexstring "#ff4757" ++ st bold)
let idle = attr_of_hexstring "#1e90ff"
let label = attr_of_hexstring "#a4b0be"

let rainbow () =
  let period = 3. in
  let h = Float.mod_float (Unix.gettimeofday ()) period /. period *. 360. in
  let rb = color_of_hsl h 1. 0.7 in
  A.(fg rb ++ default_bg ++ st bold)
;;
