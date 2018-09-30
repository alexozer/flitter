type keypress = float * string
type t = keypress Lwt_stream.t

val make_stream : unit -> t Lwt.t