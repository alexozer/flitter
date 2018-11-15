type t

val make : Timer_types.timer -> t Lwt.t
val loop : t -> unit Lwt.t
val run_once : t -> unit Lwt.t
