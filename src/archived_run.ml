open Core

type t =
  { attempt : int
  ; splits : Split.t list }
[@@deriving fields, sexp]
