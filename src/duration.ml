open Core

type t = Time_ns.Span.t [@@deriving sexp]

module Parts = struct
  type t =
    { sign : Sign.t
    ; days : int
    ; hours : int
    ; minutes : int
    ; seconds : int
    ; millis : int }

  let of_span span =
    let parts = Time_ns.Span.to_parts span in
    { sign = parts.sign
    ; days = parts.hr / 24
    ; hours = parts.hr % 24
    ; minutes = parts.min
    ; seconds = parts.sec
    ; millis = parts.ms }
  ;;

  (* let to_span t =
   *   Time_ns.Span.create ~day:t.days ~hr:t.hours ~min:t.minutes ~sec:t.seconds
   *     ~ms:t.millis () *)
end

let compiled_re =
  {|^(?:(?:(?:(\d+):)?(\d+):)?(\d+):)?(\d+)(?:\.(\d{1,3}))?$|}
  |> Re.Perl.re
  |> Re.compile
;;

let string_valid = Re.execp compiled_re

let left_pad_zeros_char_list str size =
  let rec prepend char_list n =
    if n <= 0 then char_list else prepend ('0' :: char_list) (n - 1)
  in
  prepend (String.to_list str) (size - String.length str)
;;

let left_pad_zeros size str = left_pad_zeros_char_list str size |> String.of_char_list

let right_pad_zeros size str =
  left_pad_zeros_char_list str size |> List.rev |> String.of_char_list
;;

let of_string str =
  match Re.exec_opt compiled_re str with
  | None -> None
  | Some groups ->
    let group_strs = Re.Group.all groups in
    let to_int_default x = if String.length x = 0 then 0 else Int.of_string x in
    let day = group_strs.(1) |> to_int_default in
    let hr = group_strs.(2) |> to_int_default in
    let min = group_strs.(3) |> to_int_default in
    let sec = group_strs.(4) |> to_int_default in
    let ms = group_strs.(5) |> right_pad_zeros 3 |> to_int_default in
    Time_ns.Span.create ~day ~hr ~min ~sec ~ms () |> Option.some
;;

let to_string_pos span decimals =
  let duration = Parts.of_span span in
  let open Time_ns.Span in
  let ms_str =
    let zero_padded = left_pad_zeros 3 (Int.to_string duration.millis) in
    String.prefix zero_padded decimals
  in
  let sec_str =
    let str = Int.to_string duration.seconds in
    if span >= minute then left_pad_zeros 2 str else str
  in
  let min_str =
    if span >= hour
    then (Int.to_string duration.minutes |> left_pad_zeros 2) ^ ":"
    else if span >= minute
    then Int.to_string duration.minutes ^ ":"
    else ""
  in
  let hr_str =
    if span >= day
    then (Int.to_string duration.hours |> left_pad_zeros 2) ^ ":"
    else if span >= hour
    then Int.to_string duration.hours ^ ":"
    else ""
  in
  let day_str = if span >= day then Int.to_string duration.days ^ ":" else "" in
  String.concat [day_str; hr_str; min_str; sec_str; "."; ms_str]
;;

let to_string span decimals =
  match Time_ns.Span.sign span with
  | Pos | Zero -> to_string_pos span decimals
  | Neg -> "-" ^ to_string_pos span decimals
;;

let to_string_plus span decimals =
  let str = to_string span decimals in
  match Time_ns.Span.sign span with Pos -> "+" ^ str | Neg | Zero -> str
;;

let between start finish = Time_ns.diff finish start
let since time = between time (Time_ns.now ())
let zero = Time_ns.Span.zero

let t_of_sexp = function
  | Sexp.Atom s -> of_string s |> Option.value_exn
  | _ as bad_sexp ->
    Error.raise_s [%message "A duration must be an atom" (bad_sexp : Sexp.t)]
;;

let sexp_of_t duration = Sexp.Atom (to_string duration 3)
