type t

val make : unit -> t
val draw : t -> Timer_types.speedrun -> unit Lwt.t