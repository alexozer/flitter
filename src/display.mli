type t

val make : unit -> t
val draw : t -> Timer.t -> unit
val close : t -> unit
