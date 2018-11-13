open! Core

type t = Time_ns.Span.t [@@deriving sexp]

val of_string : string -> t option

val to_string : t -> int -> string

(* int is # of decimal places *)
val to_string_plus : t -> int -> string

val string_valid : string -> bool

val between : Time_ns.t -> Time_ns.t -> t

val since : Time_ns.t -> t

val zero : t
