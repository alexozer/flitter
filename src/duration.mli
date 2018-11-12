type t = int (* Milliseconds *)

val of_string : string -> t option
val to_string : t -> int -> string (* int is # of decimal places *)
val to_string_plus : t -> int -> string
val string_valid : string -> bool

val between : float -> float -> t
val since : float -> t