open Core

type keypress = Time_ns.t * string
type t = keypress Lwt_stream.t

val make_stream : ?disable_python:unit -> unit -> t Lwt.t
