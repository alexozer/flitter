type t

val make : unit -> t
val draw : t -> Timer_types.timer -> unit
val close : t -> unit