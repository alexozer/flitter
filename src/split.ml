open Core

type t =
  { title : string
  ; time : Duration.t sexp_option
  ; is_gold : bool [@default false] }
[@@deriving fields, sexp]
