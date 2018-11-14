open Core

module Live_splits = struct
  type t = Duration.t option list
end

type t =
  | Idle
  (* splits * start time *)
  | Timing of Live_splits.t * Time_ns.t
  (* splits * start time * paused time *)
  | Paused of Live_splits.t * Time_ns.t * Time_ns.t
  (* completed splits * start time *)
  | Done of Live_splits.t * Time_ns.t
