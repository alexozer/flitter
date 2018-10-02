type t

val of_speedrun : Timer_types.speedrun -> t
val handle_key : t -> Hotkeys.keypress -> t
val handle_draw : t -> unit Lwt.t