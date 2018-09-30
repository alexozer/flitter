type t

val make : unit -> t
val draw : t -> Splits.speedrun -> unit Lwt.t