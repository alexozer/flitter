open Base

type t = int

let milli : t = 1
let second : t = 1000
let minute : t = second * 60
let hour : t = minute * 60
let day : t = hour * 60

let compiled_re =
  {|^(?:(?:(?:(\d+):)?(\d+):)?(\d+):)?(\d+)(?:\.(\d{1,3}))?$|}
  |> Re.Perl.re |> Re.compile

let left_pad_zeros_char_list str size =
  let rec prepend char_list n =
    if n <= 0 then char_list else prepend ('0' :: char_list) (n - 1)
  in
  prepend (String.to_list str) (size - String.length str)

let left_pad_zeros size str =
  left_pad_zeros_char_list str size
  |> String.of_char_list

let right_pad_zeros size str =
  left_pad_zeros_char_list str size
  |> List.rev
  |> String.of_char_list

let of_string str =
  match Re.exec_opt compiled_re str with
  | None -> None
  | Some groups ->
    let group_strs = Re.Group.all groups in

    let to_int_default x = if String.length x = 0 then 0 else Int.of_string x in

    let days = group_strs.(1) |> to_int_default in
    let hours = group_strs.(2) |> to_int_default in
    let minutes = group_strs.(3) |> to_int_default in
    let seconds = group_strs.(4) |> to_int_default in
    let millis = group_strs.(5) |> right_pad_zeros 3 |> to_int_default in

    Some (
      day * days +
      hour * hours +
      minute * minutes +
      second * seconds +
      milli * millis
    )

let to_string_pos duration decimals =
  let days = duration / day in
  let duration_day = duration % day in

  let hours = duration_day / hour in
  let duration_hour = duration_day % hour in

  let minutes = duration_hour / minute in
  let duration_minute = duration_hour % minute in

  let seconds = duration_minute / second in
  let millis = duration_minute % second in

  let millis_str = 
    let zero_padded = left_pad_zeros 3 (Int.to_string millis) in
    String.prefix zero_padded decimals
  in
  let seconds_str = 
    let str = Int.to_string seconds in
    if duration >= minute then left_pad_zeros 2 str else str
  in

  let minutes_str =
    if duration >= hour
    then (Int.to_string minutes |> left_pad_zeros 2) ^ ":"
    else if duration >= minute then Int.to_string minutes ^ ":" else ""
  in

  let hours_str =
    if duration >= day
    then (Int.to_string hours |> left_pad_zeros 2) ^ ":"
    else if duration >= hour then Int.to_string hours ^ ":" else ""
  in

  let days_str = if duration >= day then Int.to_string days ^ ":" else "" in

  String.concat [
    days_str; 
    hours_str;
    minutes_str; 
    seconds_str;
    ".";
    millis_str;
  ]

let to_string duration decimals =
  if duration < 0 then "-" ^ to_string_pos (-duration) decimals
  else to_string_pos duration decimals

let since time_float =
  (Unix.gettimeofday () -. time_float) *. 1000.
  |> Int.of_float