type game_info = {
  title : string;
  category : string;
  attempts : int;
  completed_runs : int;

  split_names : string array;
  golds : Duration.t array option;
}

type timer_state = Idle | Timing | Paused of float | Done

type speedrun = {
  game : game_info;
  comparison : Duration.t array option;

  start_time : float;
  state : timer_state;
  splits : Duration.t option array;
  curr_split : int;
}