open Base

type t = int

let milli : t = 1
let second : t = 1000
let minute : t = second * 60
let hour : t = minute * 60
let day : t = hour * 60

let compiled_re =
  let re_str = {|^(?:(?:(?:(\d+):)?(\d+):)?(\d+):)?(\d+)(?:\.(\d{1,3}))?$|} in
  Re.str re_str |> Re.compile

let left_pad_zeros_char_list str size =
  let rec prepend char_list n =
    if n <= 0 then char_list else prepend ('0' :: char_list) (n - 1)
  in
  prepend (String.to_list str) (size - String.length str)

let left_pad_zeros str size =
  left_pad_zeros_char_list str size
  |> String.of_char_list

let right_pad_zeros str size =
  left_pad_zeros_char_list str size
  |> List.rev
  |> String.of_char_list

let of_string str =
  match Re.exec_opt compiled_re str with
  | None -> None
  | Some groups -> (
      let time_strs = Re.Group.all groups in

      let to_int_default x = if String.length x = 0 then 0 else Int.of_string x in
      let time_ints = List.map (Array.to_list time_strs) ~f:to_int_default in

      match time_ints with
      | days :: hours :: minutes :: seconds :: _ :: [] -> (
          let parsed_millis_str = Array.get time_strs 4 in
          let millis = right_pad_zeros parsed_millis_str 3 |> Int.of_string in

          Some (
            day * days +
            hour * hours +
            minute * minutes +
            second * seconds +
            milli * millis
          )
        )
      | _ -> None
    )

let to_string duration decimals =
  let days = duration / day in
  let duration = duration % day in

  let hours = duration / hour in
  let duration = duration % hour in

  let minutes = duration / minute in
  let duration = duration % minute in

  let seconds = duration / second in
  let duration = duration % second in

  let millis = duration in

  let millis_str = left_pad_zeros (Int.to_string millis) decimals in
  let seconds_str = left_pad_zeros (Int.to_string seconds) 2 in

  let minutes_str =
    if minutes = 0 then "" else (
      let str = Int.to_string minutes in
      if String.length str < 2 && hours > 0 then "0" ^ str else str
    ) ^ ":"
  in

  let hours_str =
    if hours = 0 then "" else (
      let str = Int.to_string hours in
      if String.length str < 2 && days > 0 then "0" ^ str else str
    ) ^ ":"
  in

  let days_str = if days > 0 then Int.to_string days ^ ":" else "" in

  String.concat [days_str; hours_str; minutes_str; seconds_str; millis_str]