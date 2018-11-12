type split = {
  title : string;
  time : Duration.t option;
  is_gold: bool;
}

type gold = {
  title : string;
  duration : Duration.t option;
}

type archived_run = {
  attempt : int;
  splits : split array;
}

type live_splits = Duration.t option array

type timer_state = 
  | Idle
  | Timing of live_splits * float (* splits * start time *)
  | Paused of live_splits * float * float (* splits * start time * paused time *)
  | Done of live_splits * float (* completed splits * start time *)

(* Most of timer state is bundled together in this single package.
   Since it is the minimum information both the display and control logic need available,
   it doesn't make sense to further subdivide the state or abstract its contents away. *)
type timer = {
  title : string;
  category : string;
  attempts : int;
  completed : int;

  split_names : string array;
  golds : gold array;
  history : archived_run list;
  comparison : archived_run option;
  pb : archived_run option;
  wr : archived_run option;

  state : timer_state;

  splits_file : string;
}