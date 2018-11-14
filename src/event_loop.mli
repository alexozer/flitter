type t

val make : Timer.t -> t Lwt.t
val loop : t -> unit Lwt.t
