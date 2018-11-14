type t

val make : Timer_types.Timer.t -> t Lwt.t
val loop : t -> unit Lwt.t
