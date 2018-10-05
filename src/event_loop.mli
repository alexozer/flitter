type t

val make : Timer_types.timer -> t Lwt.t
val loop : t -> unit Lwt.t