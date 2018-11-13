type t

val make : unit -> t

val draw : t -> Timer_types.Timer.t -> unit

val close : t -> unit
