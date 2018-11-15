type t

val make : ?disable_python:unit -> Timer_types.timer -> t Lwt.t
val loop : t -> unit Lwt.t
val run_once : t -> unit Lwt.t
