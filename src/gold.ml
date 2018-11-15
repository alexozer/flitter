open Core

type t =
  { title : string
  ; duration : Duration.t sexp_option }
[@@deriving fields, sexp]
