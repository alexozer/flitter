type game_info = {
  game : string;
  category : string;
  attempts : int;
  completed_runs : int;
}

type split_segment = {
  title : string;
  time : Duration.t;
  is_gold_split : bool;
}

type splits = split_segment array

type gold_segment = {
  title : string;
  duration : Duration.t;
}

type golds = gold_segment array

type speedrun_split = {
  segment_idx : int;
  time : Duration.t option;
}

type state = Idle | Timing | Paused of float | Done

type speedrun = {
  game_info : game_info;
  comparison : splits;
  golds : golds;

  start_time : float;
  state : state;
  splits : speedrun_split list;
}