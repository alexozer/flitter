open Core

(* Most of timer state is bundled together in this single package.
   Since it is the minimum information both the display and control logic need available,
   it doesn't make sense to further subdivide the state or abstract its contents away. *)
type t =
  { title : string
  ; category : string
  ; attempts : int
  ; completed : int
  ; split_names : string list
  ; golds : Gold.t list
  ; history : Archived_run.t list
  ; comparison : Archived_run.t option
  ; pb : Archived_run.t option
  ; wr : Archived_run.t option
  ; state : Timer_state.t
  ; splits_file : string }
[@@deriving fields]
