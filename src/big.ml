open Core_kernel
open Notty

let big_font_map =
  let font =
    [ "00000111112222233333444445555566666777778888899999  !!::.."
    ; ".^^.  .|  .^^. .^^. .  | |^^^ .^^  ^^^| .^^. .^^.   |     "
    ; "|  |   |    .^   .^ |..| |..  |..    ][ ^..^ ^..|   | ^   "
    ; "|  |   |  .^   .  |    |    | |  |   |  |  |    |   ^ ^   "
    ; " ^^   ^^^ ^^^^  ^^     ^ ^^^   ^^    ^   ^^   ^^    ^   ^ " ]
  in
  let uchar_of_char =
    let open Caml.Uchar in
    function
    | '[' -> of_int 0x258C
    | ']' -> of_int 0x2590
    | '|' -> of_int 0x2588
    | '.' -> of_int 0x2584
    | '^' -> of_int 0x2580
    | ch -> of_char ch
  in
  let fst_line = List.hd_exn font in
  let extract_char_at start_idx =
    let ch = fst_line.[start_idx] in
    let end_idx = String.rindex_exn fst_line ch + 1 in
    let char_rows =
      List.map (List.tl_exn font) ~f:(fun line ->
          let row_str = String.(drop_prefix (prefix line end_idx) start_idx) in
          let row_list = String.to_list row_str in
          Array.of_list_map row_list ~f:uchar_of_char )
    in
    char_rows, end_idx
  in
  let rec map_chars_at map idx =
    if idx >= String.length fst_line
    then map
    else
      let img, next_idx = extract_char_at idx in
      let ch = fst_line.[idx] in
      map_chars_at (Map.add_exn map ~key:ch ~data:img) next_idx
  in
  map_chars_at (Map.empty (module Char)) 0
;;

let image_of_string attr str =
  List.map (String.to_list str) ~f:(fun ch ->
      List.map (Map.find_exn big_font_map ch) ~f:(I.uchars attr) |> I.vcat )
  |> I.hcat
;;
