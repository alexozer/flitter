type keypress = float * string
type t = keypress Lwt_stream.t

val make_stream : ?disable_python:unit -> unit -> t Lwt.t
