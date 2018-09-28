type t = int (* Milliseconds *)

val of_string : string -> t option
val to_string : t -> int -> string (* int is # of decimal places *)