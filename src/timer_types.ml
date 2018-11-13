open Core

module Split = struct
  type t =
    { title: string
    ; time: Duration.t sexp_option
    ; is_gold: bool [@default false] }
  [@@deriving fields, sexp]
end

module Gold = struct
  type t = {title: string; duration: Duration.t sexp_option}
  [@@deriving fields, sexp]
end

module Archived_run = struct
  type t = {attempt: int; splits: Split.t list} [@@deriving fields, sexp]
end

module Live_splits = struct
  type t = Duration.t option list
end

module Timer_state = struct
  type t =
    | Idle
    | Timing of Live_splits.t * Time_ns.t
    (* splits * start time *)
    | Paused of Live_splits.t * Time_ns.t * Time_ns.t
    (* splits * start time * paused time *)
    | Done of Live_splits.t * Time_ns.t
end

(* completed splits * start time *)

(* Most of timer state is bundled together in this single package.
   Since it is the minimum information both the display and control logic need available,
   it doesn't make sense to further subdivide the state or abstract its contents away. *)
module Timer = struct
  type t =
    { title: string
    ; category: string
    ; attempts: int
    ; completed: int
    ; split_names: string list
    ; golds: Gold.t list
    ; history: Archived_run.t list
    ; comparison: Archived_run.t option
    ; pb: Archived_run.t option
    ; wr: Archived_run.t option
    ; state: Timer_state.t
    ; splits_file: string }
  [@@deriving fields]
end
