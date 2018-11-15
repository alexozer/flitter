open Flitter

let timer = Loadsave.load "examples/splits.scm"

let%bench "loop" =
  Lwt_main.run (run_one_loop timer)

let%bench "test" = ()
